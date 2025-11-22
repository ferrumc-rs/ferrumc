#![feature(try_blocks)]

use crate::errors::BinaryError;
use clap::Parser;
use ferrumc_config::server_config::get_global_config;
use ferrumc_config::whitelist::create_whitelist;
use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_state::player_cache::PlayerCache;
use ferrumc_state::player_list::PlayerList;
use ferrumc_state::{GlobalState, ServerState};
use ferrumc_threadpool::ThreadPool;
use ferrumc_world::World;
use ferrumc_world_gen::WorldGenerator;
use std::sync::Arc;
use std::time::Instant;
use tracing::{error, info};

pub(crate) mod errors;
use crate::cli::{CLIArgs, Command, ImportArgs};
mod chunk_sending;
mod cli;
mod game_loop;
mod packet_handlers;
mod register_messages;
mod register_resources;
mod systems;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _profiler = dhat::Profiler::new_heap();

    let start_time = Instant::now();

    let cli_args = CLIArgs::parse();
    ferrumc_logging::init_logging(cli_args.log.into());

    ferrumc_registry::init();

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
            if let Err(e) = ferrumc_config::setup::setup() {
                error!("Could not set up the server: {}", e.to_string());
            } else {
                info!("Server setup complete.");
            }
            if let Err(e) = entry(start_time) {
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
    let start = Instant::now();
    let radius = get_global_config().chunk_render_distance as i32;
    for x in -radius..=radius {
        for z in -radius..=radius {
            chunks.push((x, z));
        }
    }
    let mut batch = state.thread_pool.batch();
    for (x, z) in chunks {
        let state_clone = state.clone();
        batch.execute(move || {
            let chunk = state_clone
                .terrain_generator
                .generate_chunk(x, z)
                .map(Arc::new);
            if let Err(e) = chunk {
                error!("Error generating chunk ({}, {}): {:?}", x, z, e);
            } else {
                let chunk = chunk.unwrap();
                if let Err(e) = state_clone.world.save_chunk(chunk) {
                    error!("Error saving chunk ({}, {}): {:?}", x, z, e);
                }
            }
        });
    }
    batch.wait();
    info!("Finished generating spawn chunks in {:?}", start.elapsed());
    Ok(())
}

fn entry(start_time: Instant) -> Result<(), BinaryError> {
    let state = create_state(start_time)?;
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
            global_state
                .world
                .sync()
                .expect("Failed to sync world before shutdown")
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
    let mut world = World::new(&get_global_config().database.db_path);

    let root_path = get_root_path();
    let mut import_path = root_path.join(import_args.import_path);
    if import_path.is_relative() {
        import_path = root_path.join(import_path);
    }

    if let Err(e) = world.import(import_path, ThreadPool::new()) {
        error!("Could not import world: {}", e.to_string());
        return Err(BinaryError::Custom("Could not import world.".to_string()));
    }

    Ok(())
}

fn create_state(start_time: Instant) -> Result<ServerState, BinaryError> {
    Ok(ServerState {
        world: World::new(&get_global_config().database.db_path),
        terrain_generator: WorldGenerator::new(0),
        shut_down: false.into(),
        players: PlayerList::default(),
        player_cache: PlayerCache::default(),
        thread_pool: ThreadPool::new(),
        start_time,
    })
}
