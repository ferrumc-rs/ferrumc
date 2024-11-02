// Security or something like that
#![forbid(unsafe_code)]
extern crate core;

use clap::Parser;
use ferrumc_config::statics::get_global_config;
use ferrumc_ecs::Universe;
use ferrumc_general_purpose::paths::get_root_path;
use ferrumc_net::server::create_server_listener;
use ferrumc_net::ServerState;
use ferrumc_world::World;
use std::sync::Arc;
use systems::definition;
use tracing::{error, info};
use crate::errors::BinaryError;

#[derive(clap::Parser)]
struct CLIArgs {
    #[clap(long)]
    import: bool,
    #[clap(long)]
    log: Option<String>,
}


pub(crate) mod errors;
mod packet_handlers;
mod systems;

pub type Result<T> = std::result::Result<T, errors::BinaryError>;

#[tokio::main]
async fn main() {
    let cli_args = CLIArgs::parse();
    ferrumc_logging::init_logging(cli_args.log.clone());

    println!("good day to ya. enjoy your time with ferrumc!");

    if let Err(e) = entry(cli_args).await {
        error!("Server exited with the following error;");
        error!("{:?}", e);
    } else {
        info!("Server exited successfully.");
    }
}

async fn entry(cli_args: CLIArgs) -> Result<()> {
    handle_import(cli_args.import).await?;

    let state = create_state().await?;
    let global_state = Arc::new(state);


    let all_system_handles = tokio::spawn(definition::start_all_systems(global_state.clone()));

    // Start the systems and wait until all of them are done
    all_system_handles.await??;

    // Stop all systems
    definition::stop_all_systems(global_state).await?;

    Ok(())
}

async fn handle_import(import: bool) -> Result<()> {
    if !import {
        return Ok(());
    }

    info!("`--import` flag detected. Importing world...");

    // Import the world
    let config = get_global_config();
    let mut world = World::new().await;

    let root_path = get_root_path();
    let database_opts = &config.database;


    let import_path = root_path.join(database_opts.import_path.clone());
    let db_path = root_path.join(database_opts.db_path.clone());

    if let Err(e) = world.import(import_path, db_path).await {
        error!("Could not import world: {:?}", e);
        return Err(BinaryError::from(e));
    }

    Ok(())
}

async fn create_state() -> Result<ServerState> {
    let listener = create_server_listener().await?;

    Ok(ServerState {
        universe: Universe::new(),
        tcp_listener: listener,
    })
}
