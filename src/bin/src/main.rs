#![feature(try_blocks)]

use crate::cli::{CLIArgs, Command};
use crate::errors::BinaryError;
use clap::Parser;
use ferrumc_config::whitelist::create_whitelist;
use ferrumc_world::pos::ChunkPos;
use std::sync::Arc;
use std::time::Instant;
use tracing::{error, info};

mod cli;
pub(crate) mod errors;
mod game_loop;
mod launch;
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
            match launch::handle_import(import_args) { Err(e) => {
                error!("Import failed with the following error: {}", e.to_string());
            } _ => {
                info!("Import completed successfully.");
            }}
        }

        Some(Command::Clear(clear_args)) => {
            if let Err(e) = cli::handle_clear(clear_args) {
                error!("Clear failed: {}", e);
            }
        }

        Some(Command::Run) | None => {
            info!("Starting server...");
            if let Err(e) = ferrumc_config::setup::setup() {
                error!("Could not set up the server: {}", e.to_string());
            } else {
                info!("Server setup complete.");
            }
            match entry(start_time) { Err(e) => {
                error!("Server exited with the following error: {}", e.to_string());
            } _ => {
                info!("Server exited successfully.");
            }}
        }
    }
}

fn entry(start_time: Instant) -> Result<(), BinaryError> {
    let state = launch::create_state(start_time)?;
    let global_state = Arc::new(state);

    create_whitelist();
    if !global_state
        .world
        .chunk_exists(ChunkPos::new(0, 0), "overworld")?
    {
        launch::generate_spawn_chunks(global_state.clone())?;
    }

    #[cfg(feature = "dashboard")]
    ferrumc_dashboard::start_dashboard(global_state.clone());

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
