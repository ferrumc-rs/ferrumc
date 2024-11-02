// Security or something like that
#![forbid(unsafe_code)]
extern crate core;

use std::path::PathBuf;
use ferrumc_ecs::Universe;
use ferrumc_net::server::create_server_listener;
use ferrumc_net::ServerState;
use std::sync::Arc;
use clap::Parser;
use systems::definition;
use tracing::{error, info};
use ferrumc_config::statics::get_global_config;
use ferrumc_world::World;

#[derive(clap::Parser)]
struct CLIArgs {
    #[clap(long)]
    import: bool
}


pub(crate) mod errors;
mod packet_handlers;
mod systems;

pub type Result<T> = std::result::Result<T, errors::BinaryError>;

#[tokio::main]
async fn main() {
    ferrumc_logging::init_logging();

    println!("good day to ya. enjoy your time with ferrumc!");
    
    let cli_args = CLIArgs::parse();
    
    if cli_args.import {
        // Import the world
        let config = get_global_config();
        let mut world = World::new().await;
        let import_path = PathBuf::from(config.database.import_path.clone());
        let db_path = PathBuf::from(config.database.db_path.clone());
        if let Err(e) = world.import(import_path, db_path).await {
            error!("Could not import world: {:?}", e);
            return;
        }
        return;
    }

    if let Err(e) = entry().await {
        error!("Server exited with the following error;");
        error!("{:?}", e);
    } else {
        info!("Server exited successfully.");
    }
}

async fn entry() -> Result<()> {
    let state = create_state().await?;
    let global_state = Arc::new(state);


    let all_system_handles = tokio::spawn(definition::start_all_systems(global_state.clone()));

    // Start the systems and wait until all of them are done
    all_system_handles.await??;

    // Stop all systems
    definition::stop_all_systems(global_state).await?;

    Ok(())
}

async fn create_state() -> Result<ServerState> {
    let listener = create_server_listener().await?;

    Ok(ServerState {
        universe: Universe::new(),
        tcp_listener: listener,
    })
}
