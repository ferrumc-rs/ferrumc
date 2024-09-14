use std::env;
use std::process::exit;

use ferrumc::{create_state, setup, utils, world};
use tokio::net::TcpListener;
use tracing::{error, info, trace};

use ferrumc::{
    net::systems::{kill_all_systems, start_all_systems},
    utils::{config::get_global_config, prelude::*},
};

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
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_cpus::get())
            .build_global()
            .expect("Failed to build rayon thread pool");
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
