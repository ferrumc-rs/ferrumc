#![feature(box_into_inner)]
#![feature(fs_try_exists)]

use std::fs;
use std::sync::Arc;
#[allow(unused_imports)]
use tokio::fs::try_exists;
#[warn(unused_imports)]
use clap::{Parser};

use log::{debug, info, trace};
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use ferrumc_utils::prelude::*;

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

    let args = Cli::parse();

    // run setup if the flag is set or the config file does not exist in release mode
    if args.setup || (!fs::try_exists("config.toml")? && !cfg!(debug_assertions)) {
        setup::setup().await?;
        return Ok(())
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

        tokio::task::spawn(ferrumc_net::handle_connection(socket));
    }
}