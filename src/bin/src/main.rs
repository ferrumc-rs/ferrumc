#![feature(try_blocks)]

use crate::cli::{CLIArgs, Command};
use crate::errors::BinaryError;
use clap::Parser;
use ferrumc_config::whitelist::create_whitelist;
use ferrumc_state::GlobalState;
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
mod tui;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[cfg(all(feature = "tracy", not(feature = "dhat")))]
#[global_allocator]
static GLOBAL: tracy_client::ProfiledAllocator<std::alloc::System> =
    tracy_client::ProfiledAllocator::new(std::alloc::System, 100);

fn main() {
    #[cfg(feature = "dhat")]
    let _profiler = dhat::Profiler::new_heap();

    let start_time = Instant::now();

    let cli_args = CLIArgs::parse();
    ferrumc_logging::init_logging(cli_args.log.into(), cli_args.no_tui);

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
            if let Err(e) = launch::handle_import(import_args) {
                error!("Import failed with the following error: {}", e.to_string());
            } else {
                info!("Import completed successfully.");
            }
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
            if let Err(e) = entry(start_time, cli_args.no_tui) {
                error!("Server exited with the following error: {}", e.to_string());
            } else {
                info!("Server exited successfully.");
            }
        }
    }
    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode on exit");
}

fn entry(start_time: Instant, no_tui: bool) -> Result<(), BinaryError> {
    let state = launch::create_state(start_time)?;
    let global_state = Arc::new(state);
    create_whitelist();

    if !global_state
        .world
        .chunk_exists(ChunkPos::new(0, 0), "overworld")?
    {
        launch::generate_spawn_chunks(global_state.clone())?;
    }

    if no_tui {
        ctrlc::set_handler({
            let global_state = global_state.clone();
            move || {
                shutdown_handler(global_state.clone());
            }
        })
        .expect("Error setting Ctrl-C handler");
    }

    #[cfg(feature = "dashboard")]
    ferrumc_dashboard::start_dashboard(global_state.clone());

    game_loop::start_game_loop(global_state.clone(), no_tui)?;

    if !no_tui {
        ratatui::restore()
    }

    Ok(())
}

pub(crate) fn shutdown_handler(state: GlobalState) {
    info!("Shutting down server...");
    state
        .shut_down
        .store(true, std::sync::atomic::Ordering::Relaxed);
    state
        .world
        .sync()
        .expect("Failed to sync world before shutdown")
}
