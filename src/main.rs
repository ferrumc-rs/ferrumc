#![feature(box_into_inner)]
#![feature(fs_try_exists)]
#![feature(async_closure)]
#![feature(future_join)]

use std::env;
use std::future::join;
use std::sync::Arc;

#[warn(unused_imports)]
use clap::Parser;
#[allow(unused_imports)]
use tokio::fs::try_exists;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing::{debug, error, info, Instrument, trace};

use ferrumc_utils::prelude::*;

mod setup;
mod tests;
mod utils;

type SafeConfig = Arc<RwLock<ferrumc_utils::config::ServerConfig>>;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(long, default_value = "false")]
    setup: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    if handle_setup().await? {
        return Ok(());
    }

    utils::setup_logger();
    info!("Initializing server...");

    let start = std::time::Instant::now();
    let config = ferrumc_utils::config::ServerConfig::new()?;
    let elapsed = start.elapsed();

    debug!("Found Config: {:?} in {:?}", config, elapsed);

    let config = Arc::new(RwLock::new(config));

    let procs = tokio::join!(
        start_server(config.clone()),
        ferrumc_world::start_database()
    );

    procs.0.unwrap();
    procs.1.unwrap();

    tokio::signal::ctrl_c().await?;

    Ok(())
}

/// Starts the server. Sets up the sockets and listens for incoming connections
///
/// The actual management of connections in handled by [ferrumc_net::init_connection]
async fn start_server(config: SafeConfig) -> Result<()> {
    let config = config.read().await;
    trace!("Starting server on {}:{}", config.host, config.port);

    let tcp_addr = format!("{}:{}", config.host, config.port);

    let listener = TcpListener::bind(tcp_addr).await?;
    let addr = listener.local_addr()?;

    info!("Server started on {}", addr);
    drop(config);

    loop {
        let (socket, addy) = listener.accept().await?;
        // show a line of 100 dashes
        trace!("{}", "-".repeat(100));
        debug!("Accepted connection from: {:?}", socket.peer_addr()?);

        tokio::task::spawn(
            async {
                if let Err(e) = ferrumc_net::init_connection(socket).await {
                    error!("Error handling connection: {:?}", e);
                }
            }
            .instrument(tracing::info_span!("handle_connection", %addy)),
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
    let args = Cli::parse();

    // This env var will be present if the server is running in a CI environment
    // This will lead to set up not running, but we just need to check for compilation success, not actual functionality
    if env::var("GITHUB_ACTIONS").is_ok() {
        env::set_var("RUST_LOG", "info");
        Ok(false)
    // If the setup flag is passed, run the setup regardless of the config file
    } else if args.setup {
        setup::setup().await?;
        return Ok(true);
    // Check if the config file exists already and run the setup if it doesn't
    } else {
        // Get the path to the current executable
        let exe = std::env::current_exe()?;
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
