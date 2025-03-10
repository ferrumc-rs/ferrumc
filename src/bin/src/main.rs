#![feature(portable_simd)]
#![feature(random)]
extern crate core;

use crate::errors::BinaryError;
use clap::Parser;
use ferrumc_config::statics::get_global_config;
use ferrumc_config::whitelist::create_whitelist;
use ferrumc_ecs::Universe;
use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_net::server::create_server_listener;
use ferrumc_state::ServerState;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::World;
use ferrumc_world_gen::errors::WorldGenError;
use ferrumc_world_gen::WorldGenerator;
use rayon::prelude::*;
use std::sync::Arc;
use systems::definition;
use tokio::runtime::Handle;
use tracing::{error, info};

pub(crate) mod errors;
use crate::cli::{CLIArgs, Command, ImportArgs};
mod chunk_sending;
mod cli;
mod packet_handlers;
mod systems;

// #[tokio::main(flavor = "current_thread")]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let cli_args = CLIArgs::parse();
    ferrumc_logging::init_logging(cli_args.log.into());

    let current_active_threads = Handle::current().metrics().num_workers();

    info!("FERRUMC IS USING {} THREAD(s)", current_active_threads);

    match cli_args.command {
        Some(Command::Setup) => {
            info!("Starting setup...");
            if let Err(e) = ferrumc_config::setup::setup() {
                error!("Could not set up the server: {}", e.to_string());
            } else {
                info!("Server setup complete.");
            }
        }

        Some(Command::Import(import_args)) => {
            info!("Starting import...");
            if let Err(e) = handle_import(import_args).await {
                error!("Import failed with the following error: {}", e.to_string());
            } else {
                info!("Import completed successfully.");
            }
        }
        Some(Command::Run) | None => {
            info!("Starting server...");
            if let Err(e) = entry().await {
                error!("Server exited with the following error: {}", e.to_string());
            } else {
                info!("Server exited successfully.");
            }
        }
    }
}

async fn entry() -> Result<(), BinaryError> {
    let state = create_state().await?;
    let global_state = Arc::new(state);
    create_whitelist().await;
    if !global_state.world.chunk_exists(0, 0, "overworld").await? {
        info!("No overworld spawn chunk found, generating spawn chunks...");
        // Generate a 12x12 chunk area around the spawn point
        let mut chunks = Vec::new();
        for x in -12..12 {
            for z in -12..12 {
                chunks.push((x, z));
            }
        }
        let generated_chunks: Vec<Result<Chunk, WorldGenError>> = chunks
            .chunks(72)
            .par_bridge()
            .map(|chunk_coord_arr| {
                let mut generated_chunks = Vec::new();
                for (x, z) in chunk_coord_arr {
                    let state = global_state.clone();
                    generated_chunks.push(state.terrain_generator.generate_chunk(*x, *z));
                }
                generated_chunks
            })
            .flatten()
            .collect();
        for chunk in generated_chunks {
            let chunk = chunk.map_err(|e| {
                error!("Error generating chunk: {:?}", e);
                BinaryError::Custom("Error generating chunk".to_string())
            })?;
            global_state.world.save_chunk(chunk).await?;
        }
        info!("Finished generating spawn chunks...");
    }

    let all_system_handles = tokio::spawn(definition::start_all_systems(global_state.clone()));

    //Start the systems and wait until all of them are done
    all_system_handles.await??;

    // Stop all systems
    definition::stop_all_systems(global_state).await?;

    Ok(())
}

async fn handle_import(import_args: ImportArgs) -> Result<(), BinaryError> {
    //! Handles the import of the world.
    info!("Importing world...");

    let config = get_global_config();
    let mut world = World::new().await;

    let root_path = get_root_path();
    let database_opts = &config.database;

    let mut import_path = root_path.join(import_args.import_path);
    if import_path.is_relative() {
        import_path = root_path.join(import_path);
    }
    let mut db_path = root_path.join(database_opts.db_path.clone());
    if db_path.is_relative() {
        db_path = root_path.join(db_path);
    }

    if let Err(e) = world.import(import_path, db_path).await {
        error!("Could not import world: {}", e.to_string());
        return Err(BinaryError::Custom("Could not import world.".to_string()));
    }

    Ok(())
}

async fn create_state() -> Result<ServerState, BinaryError> {
    let listener = create_server_listener().await?;

    Ok(ServerState {
        universe: Universe::new(),
        tcp_listener: listener,
        world: World::new().await,
        terrain_generator: WorldGenerator::new(0),
    })
}
