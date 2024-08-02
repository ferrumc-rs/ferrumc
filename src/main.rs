#![feature(box_into_inner)]
#![feature(fs_try_exists)]
#![feature(async_closure)]
#![feature(future_join)]

use std::env;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing::{debug, error, info, info_span, Instrument, trace};

use crate::{
    net::{Connection, ConnectionWrapper},
    net::systems::{kill_all_systems, start_all_systems},
    utils::{config, config::get_global_config, prelude::*},
};
use crate::state::GlobalState;

pub mod ecs;
pub mod net;
mod setup;
mod tests;
pub mod utils;

mod database;
mod state;
pub mod world;
#[tokio::main]
async fn main() -> Result<()> {
    utils::setup_logger();

    if handle_setup().await? {
        return Ok(());
    }

    info!("Initializing server...");

    let start = std::time::Instant::now();
    let config = config::ServerConfig::new()?;
    let elapsed = start.elapsed();

    debug!("Found Config: {:?} in {:?}", config, elapsed);

    let db = database::start_database().await?;

    let state: GlobalState = Arc::new(RwLock::new(state::ServerState {
        world: ecs::world::World::new(),
        connections: net::ConnectionList::new(),
        database: db,
    }));

    if env::args().nth(1).unwrap_or_default() == "import" {
        let import_path = env::current_exe().unwrap().parent().unwrap().join("import");
        world::importing::import_regions(import_path, state.clone())
            .await
            .unwrap();
        return Ok(());
    } else {
        start_server(state).await.expect("Server failed to start!");

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

    let listener = TcpListener::bind(tcp_addr).await?;
    let addr = listener.local_addr()?;

    info!("Server started on {}", addr);

    let read_connections = tokio::spawn(read_connections(listener, state.clone()));
    // Start all systems (separate task)
    let all_systems = tokio::task::spawn(start_all_systems(state.clone()));
    let (con, systems) = tokio::try_join!(read_connections, all_systems)?;
    con?;
    systems?;

    // Kill all systems since we're done.
    kill_all_systems().await?;

    Ok(())
}

async fn read_connections(listener: TcpListener, state: GlobalState) -> Result<()> {
    loop {
        let (socket, addy) = listener.accept().await?;
        // show a line of 100 dashes
        trace!("{}", "-".repeat(100));
        debug!("Accepted connection from: {:?}", socket.peer_addr()?);
        let state = state.clone();
        tokio::task::spawn(
            async move {
                if let Err(e) = net::init_connection(socket, state.clone()).await {
                    error!("Error handling connection: {:?}", e);
                }
            }
            .instrument(info_span!("handle_connection", %addy)),
        );
    }
}

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
