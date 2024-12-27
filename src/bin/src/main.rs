#![feature(portable_simd)]
#![forbid(unsafe_code)]
extern crate core;

use crate::errors::BinaryError;
use clap::Parser;
use ferrumc_config::statics::get_global_config;
use ferrumc_ecs::Universe;
use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_net::server::create_server_listener;
use ferrumc_state::ServerState;
use ferrumc_world::World;
use std::sync::Arc;
use systems::definition;
use tracing::{error, info};

pub(crate) mod errors;
use crate::cli::{CLIArgs, Command, ImportArgs};
mod cli;
mod packet_handlers;
mod systems;

pub type Result<T> = std::result::Result<T, BinaryError>;

#[tokio::main]
async fn main() {
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

async fn entry() -> Result<()> {
    let state = create_state().await?;
    let global_state = Arc::new(state);

    // Needed for some reason because ctor doesn't really want to do ctor things otherwise.
    ferrumc_default_commands::init();

    let all_system_handles = tokio::spawn(definition::start_all_systems(global_state.clone()));

    // Start the systems and wait until all of them are done
    all_system_handles.await??;

    // Stop all systems
    definition::stop_all_systems(global_state).await?;

    Ok(())
}

async fn handle_import(import_args: ImportArgs) -> Result<()> {
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

async fn create_state() -> Result<ServerState> {
    let listener = create_server_listener().await?;

    Ok(ServerState {
        universe: Universe::new(),
        tcp_listener: listener,
        world: World::new().await,
    })
}
