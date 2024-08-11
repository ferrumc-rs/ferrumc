#![feature(const_type_id)] // For TypeId::of as a const fn
#![feature(box_into_inner)]
#![feature(async_closure)]
#![feature(future_join)]
#![feature(portable_simd)]
extern crate core;

use std::env;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;

use clap::Parser;
use clap_derive::Parser;
use tokio::net::TcpListener;
use tracing::{debug, error, info, trace};

use crate::{
    net::Connection,
    net::systems::{kill_all_systems, start_all_systems},
    utils::{config, config::get_global_config, prelude::*},
};
use crate::ecs::world::World;
use crate::net::ConnectionList;
use crate::state::{GlobalState, ServerState};

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

    if handle_setup().await? {
        return Ok(());
    }

    info!("Initializing server...");

    let start = std::time::Instant::now();
    let config = config::ServerConfig::new()?;
    let elapsed = start.elapsed();

    debug!("Found Config: \n{:#?} \nin {:?}", config, elapsed);

    if env::args().nth(1).unwrap_or_default() == "import" {
        let import_path = env::current_exe().unwrap().parent().unwrap().join("import");
        world::importing::import_regions(import_path, state.clone())
            .await
            .unwrap();
        return Ok(());
    } else {
        start_server().await?;

        tokio::signal::ctrl_c().await?;

        Ok(())
    }
}

/// Starts the server. Sets up the sockets and listens for incoming connections
///
/// The actual management of connections in handled by [r#mod::init_connection]
async fn start_server(state: GlobalState) -> Result<()> {
    let config = get_global_config();
    trace!("Starting server on {}:{}", config.host, config.port);

    let tcp_addr = format!("{}:{}", config.host, config.port);

    // let listener = TcpListener::bind(tcp_addr).await?;
    let Ok(listener) = TcpListener::bind(tcp_addr.clone()).await else {
        /*error!("Failed to bind to address: {}", &tcp_addr);
        error!("Perhaps the address {} is already in use?", &tcp_addr);
        return Err(Error::TcpError("Failed to bind to address".to_string()));*/
        // let error = format!("Failed to bind to address: {} \nPerhaps the address {} is already in use?", &tcp_addr, &tcp_addr);
        //
        // error!("{}", error);

        error!("Failed to bind to address: {}", &tcp_addr);
        error!("Perhaps the port {} is already in use?", &config.port);

        return Err(Error::TcpError("Failed to bind to address".to_string()));
    };

    let addr = listener.local_addr()?;

    info!("Server started on {}", addr);

    let state = create_state(listener).await?;

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

/*async fn read_connections(listener: TcpListener, state: GlobalState) -> Result<()> {
    loop {
        let state = state.clone();
        let (socket, addy) = listener.accept().await?;
        // show a line of 100 dashes
        trace!("{}", "-".repeat(100));
        debug!("Accepted connection from: {:?}", socket.peer_addr()?);
        let state = state.clone();
        tokio::task::spawn(
            async {
                if let Err(e) = net::init_connection(socket).await {
                    error!("Error handling connection: {:?}", e);
                }
            }
                .instrument(info_span!("handle_connection", %addy)),
        );
    }
}*/

/// Handles the setup of the server
///
/// If the server is running in a CI environment, it will set the log level to info
///
/// Returns True if the server should exit after setup
///
/// Runs [setup::setup] if the server needs setting up
async fn handle_setup() -> Result<bool> {
    // This env var will be present if the server is running in a CI environment
    // This will lead to set up not running, but we just need to check for compilation success, not actual functionality
    if env::var("GITHUB_ACTIONS").is_ok() {
        env::set_var("RUST_LOG", "info");
        Ok(false)
        // If the setup flag is passed, run the setup regardless of the config file
    } else if env::args().any(|x| x == "setup") {
        setup::setup().await?;
        return Ok(true);
        // Check if the config file exists already and run the setup if it doesn't
    } else {
        // Get the path to the current executable
        let exe = env::current_exe()?;
        // This should be the directory the executable is in.
        // This should always work but if it doesn't, we'll just return an error
        let dir = exe.parent();
        match dir {
            Some(dir) => {
                let config_path = dir.join("config.toml");
                if !config_path.exists() {
                    setup::setup().await?;
                }
                Ok(false)
            }
            None => {
                error!("Failed to get the directory of the executable. Exiting...");
                return Ok(true);
            }
        }
    }
}
