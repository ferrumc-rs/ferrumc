#![feature(try_blocks)]
use crate::errors::BinaryError;
use clap::Parser;
use dashmap::DashMap;
use ferrumc_config::statics::get_global_config;
use ferrumc_config::whitelist::create_whitelist;
use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_state::{GlobalState, ServerState};
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::World;
use ferrumc_world_gen::errors::WorldGenError;
use ferrumc_world_gen::WorldGenerator;
use std::sync::Arc;
use tracing::{error, info};

pub(crate) mod errors;
use crate::cli::{CLIArgs, Command, ImportArgs};
mod chunk_sending;
mod cli;
mod game_loop;
mod packet_handlers;
mod register_events;
mod systems;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _profiler = dhat::Profiler::new_heap();

    let cli_args = CLIArgs::parse();
    ferrumc_logging::init_logging(cli_args.log.into());

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
            if let Err(e) = handle_import(import_args) {
                error!("Import failed with the following error: {}", e.to_string());
            } else {
                info!("Import completed successfully.");
            }
        }
        Some(Command::Run) | None => {
            info!("Starting server...");
            if let Err(e) = entry() {
                error!("Server exited with the following error: {}", e.to_string());
            } else {
                info!("Server exited successfully.");
            }
        }
    }
}

fn generate_chunks(state: GlobalState) -> Result<(), BinaryError> {
    info!("No overworld spawn chunk found, generating spawn chunks...");
    // Generate a 12x12 chunk area around the spawn point
    let mut chunks = Vec::new();
    let radius = get_global_config().chunk_render_distance as i32;
    for x in -radius..=radius {
        for z in -radius..=radius {
            chunks.push((x, z));
        }
    }
    let generated_chunks: Vec<Result<Chunk, WorldGenError>> = chunks
        .iter()
        .map(|(x, z)| {
            let state = state.clone();
            state.terrain_generator.generate_chunk(*x, *z)
        })
        .collect();
    for chunk in generated_chunks {
        let chunk = chunk.map_err(|e| {
            error!("Error generating chunk: {:?}", e);
            BinaryError::Custom("Error generating chunk".to_string())
        })?;
        state.world.save_chunk(chunk)?;
    }
    info!("Finished generating spawn chunks...");
    Ok(())
}

fn entry() -> Result<(), BinaryError> {
    let state = create_state()?;
    let global_state = Arc::new(state);
    create_whitelist();
    if !global_state.world.chunk_exists(0, 0, "overworld")? {
        generate_chunks(global_state.clone())?;
    }

    ctrlc::set_handler({
        let global_state = global_state.clone();
        move || {
            info!("Shutting down server...");
            global_state
                .shut_down
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
    })
    .expect("Error setting Ctrl-C handler");

    game_loop::start_game_loop(global_state.clone())?;

    Ok(())
}

fn handle_import(import_args: ImportArgs) -> Result<(), BinaryError> {
    //! Handles the import of the world.
    info!("Importing world...");

    // let config = get_global_config();
    let mut world = World::new();

    let root_path = get_root_path();
    let mut import_path = root_path.join(import_args.import_path);
    if import_path.is_relative() {
        import_path = root_path.join(import_path);
    }

    if let Err(e) = world.import(
        import_path,
        import_args.batch_size,
        // import_args.max_concurrent_tasks,
    ) {
        error!("Could not import world: {}", e.to_string());
        return Err(BinaryError::Custom("Could not import world.".to_string()));
    }

    Ok(())
}

fn create_state() -> Result<ServerState, BinaryError> {
    Ok(ServerState {
        world: World::new(),
        terrain_generator: WorldGenerator::new(0),
        shut_down: false.into(),
        players: DashMap::default(),
    })
}
