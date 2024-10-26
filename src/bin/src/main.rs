// Security or something like that
#![forbid(unsafe_code)]

use ferrumc_ecs::Universe;

use ferrumc_events::infrastructure::Event;
use ferrumc_net::{packets::outgoing::tick_event::TickEvent, ServerState};
use std::{sync::Arc, time::Duration};
use systems::definition;
use tokio::time::Instant;
use tracing::{debug, error, info};

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
    let global_state = Arc::new(create_state().await?);


    let all_systems = tokio::spawn(definition::start_all_systems(Arc::clone(&global_state)));

    // Start the systems and wait until all of them are done
    all_systems.await??;

    // Stop all systems
    definition::stop_all_systems(global_state).await?;

    Ok(())
}

async fn create_state() -> Result<ServerState> {
    let config = ferrumc_config::statics::get_global_config();
    let addy = format!("{}:{}", config.host, config.port);

    let listener = tokio::net::TcpListener::bind(addy).await?;

    Ok(ServerState {
        universe: Universe::new(),
        tcp_listener: listener,
    })
}
