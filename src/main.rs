#![feature(box_into_inner)]
#![feature(fs_try_exists)]

use std::env;
use std::sync::Arc;

#[warn(unused_imports)]
use clap::Parser;
use ferrumc_utils::prelude::*;
use log::{debug, error, info, trace};
#[allow(unused_imports)]
use tokio::fs::try_exists;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

mod prelude;
mod tests;
mod utils;
mod setup;

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

    start_server(config.clone())
        .await
        .expect("Server failed to start!");

    tokio::signal::ctrl_c().await?;

    Ok(())
}

async fn start_server(config: SafeConfig) -> Result<()> {
    let config = config.read().await;
    trace!("Starting server on {}:{}", config.host, config.port);

    let tcp_addr = format!("{}:{}", config.host, config.port);

    let listener = TcpListener::bind(tcp_addr).await?;
    let addr = listener.local_addr()?;

    info!("Server started on {}", addr);
    drop(config);

    loop {
        let (socket, _) = listener.accept().await?;
        // show a line of 100 dashes
        trace!("{}", "-".repeat(100));
        debug!("Accepted connection from: {:?}", socket.peer_addr()?);

        tokio::task::spawn(async {
            if let Err(e) = ferrumc_net::handle_connection(socket).await {
                error!("Error handling connection: {:?}", e);
            }
        });
    }
}

/// Handles the setup of the server
/// bool : true if to exit the program
async fn handle_setup() -> Result<bool> {
    let args = Cli::parse();

    if env::var("GITHUB_ACTIONS").is_ok() {
        env::set_var("RUST_LOG", "info");
        Ok(false)
    } else if args.setup {
        setup::setup().await?;
        return Ok(true);
    } else {
        let exe = std::env::current_exe()?;
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
                error!("No parent directory found for executable! Please don't try run ferrumc from root, its really not a good idea");
                return Ok(true);
            }
        }
    }
}