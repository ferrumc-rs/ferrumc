// Security or something like that
#![forbid(unsafe_code)]
#![feature(slice_as_chunks)]

use ferrumc_ecs::Universe;
use ferrumc_net::server::create_server_listener;
use ferrumc_net::ServerState;
use std::sync::Arc;
use systems::definition;
use tracing::{error, info};

pub(crate) mod errors;
mod packet_handlers;
mod systems;

pub type Result<T> = std::result::Result<T, errors::BinaryError>;

#[tokio::main]
async fn main() {
    ferrumc_logging::init_logging();

    println!("good day to ya. enjoy your time with ferrumc!");

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
    let result_systems = all_system_handles.await?;

    match result_systems {
        Ok(systems) => {
            definition::stop_all_systems(global_state, systems).await?;
        }
        Err(e) => {
            error!("Something went wrong with the systems: {:?}", e);
        }
    }
    // Stop all systems

    Ok(())
}

async fn create_state() -> Result<ServerState> {
    let listener = create_server_listener().await?;

    Ok(ServerState {
        universe: Universe::new(),
        tcp_listener: listener,
    })
}
