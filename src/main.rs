#![feature(box_into_inner)]

extern crate core;
#[macro_use]
extern crate macro_rules_attribute;

use std::env;
use std::process::exit;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use dashmap::DashMap;
use tokio::net::TcpListener;
use tracing::{error, info, trace};

use crate::ecs::world::World;
use crate::net::ConnectionList;
use crate::state::{GlobalState, ServerState};
use crate::{
    net::systems::{kill_all_systems, start_all_systems},
    net::Connection,
    utils::{config::get_global_config, prelude::*},
};

pub mod ecs;
pub mod net;
mod setup;
#[cfg(test)]
mod tests;
pub mod utils;

mod database;
mod state;
pub mod world;

#[tokio::main]
async fn main() {
    let result = entry().await;

    match result {
        Ok(_) => {
            info!("Server exited successfully!");
        }
        Err(e) => {
            error!("Server exited with an error");
            error!("{}", e);
        }
    }
}

async fn entry() -> Result<()> {
    utils::setup_logger()?;

    if setup::handle_setup().await? {
        return Ok(());
    }

    info!("Initializing server...");

    start_server().await?;

    tokio::signal::ctrl_c().await?;

    info!("Exiting server;");

    Ok(())
}

/// Starts the server. Sets up the sockets and listens for incoming connections
///
/// The actual management of connections tx/rx is handled by [net::systems::connection_handler]
async fn start_server() -> Result<()> {
    let config = get_global_config();
    trace!("Starting server on {}:{}", config.host, config.port);

    let tcp_addr = format!("{}:{}", config.host, config.port);

    let Ok(listener) = TcpListener::bind(tcp_addr.clone()).await else {
        error!("Failed to bind to address: {}", &tcp_addr);
        error!("Perhaps the port {} is already in use?", &config.port);

        return Err(Error::TcpError("Failed to bind to address".to_string()));
    };

    let addr = listener.local_addr()?;

    let state = create_state(listener).await?;

    if env::args().any(|arg| arg == "--import") {
        // world::importing::import_regions(state.clone()).await?;
        rayon::ThreadPoolBuilder::new().num_threads(num_cpus::get() * 2).build_global().expect("Failed to build rayon thread pool");
        world::importing::import_regions(state.clone()).await?;
        exit(0);
    }

    info!("Server started on {}", addr);

    // Start all systems (separate task)
    let all_systems = tokio::task::spawn(start_all_systems(state));

    // Wait for all systems to finish
    all_systems.await??;

    // Kill all systems since we're done.
    kill_all_systems().await?;

    Ok(())
}
async fn create_state(tcp_listener: TcpListener) -> Result<GlobalState> {
    Ok(Arc::new(ServerState {
        world: Arc::new(World::new()),
        connections: ConnectionList {
            connections: DashMap::new(),
            connection_count: AtomicU32::new(0),
        },
        database: database::start_database().await?,
        server_stream: tcp_listener,
    }))
}


