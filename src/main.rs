#![feature(box_into_inner)]
#![feature(fs_try_exists)]

use std::env;
use std::sync::Arc;

#[warn(unused_imports)]
use clap::Parser;
use ferrumc_ecs::world::World;
use ferrumc_net::GET_WORLD;
use ferrumc_utils::components::player::Player;
use ferrumc_utils::config::get_global_config;
use ferrumc_utils::encoding::position::Position;
#[allow(unused_imports)]
use tokio::fs::try_exists;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing::{debug, error, info, trace, Instrument};

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

    
    start_server()
        .await
        .expect("Server failed to start!");

    tokio::signal::ctrl_c().await?;

    Ok(())
}

/// Starts the server. Sets up the sockets and listens for incoming connections
///
/// The actual management of connections in handled by [ferrumc_net::init_connection]
async fn start_server() -> Result<()> {
    let config = get_global_config();
    trace!("Starting server on {}:{}", config.host, config.port);

    let tcp_addr = format!("{}:{}", config.host, config.port);

    let listener = TcpListener::bind(tcp_addr).await?;
    let addr = listener.local_addr()?;

    info!("Server started on {}", addr);
    
    
    let read_connections = tokio::task::spawn(async move {
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
    });
    
    let systems = tokio::task::spawn(async {
        loop {
            let mut world = GET_WORLD().write().await;
            // an example system (like just log all players)
            for (id, (player)) in world.query::<(Player)>().iter() {
                info!("[Entity {}] Player: {:?}", id, player);
            }
            
            // wait for a tick (1/20)
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    });
    
    tokio::try_join!(read_connections, systems)?;
    
    Ok(())
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
