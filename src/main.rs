#![allow(unused)]

use std::sync::Arc;

use log::{debug, info, trace};
use tokio::net::{TcpListener};
use tokio::sync::RwLock;

use crate::prelude::*;

mod error;
mod prelude;
mod utils;
mod constants;

type SafeConfig = Arc<RwLock<utils::config::ServerConfig>>;

#[tokio::main]
async fn main() -> Result<()> {
    utils::setup_logger();
    info!("Initializing server...");

    let start = std::time::Instant::now();
    let config = utils::config::ServerConfig::new()?;
    let elapsed = start.elapsed();

    debug!("Found Config: {:?} in {:?}", config, elapsed);

    let config = Arc::new(RwLock::new(config));

    start_server(config.clone()).await;

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

        trace!("Accepted connection from: {:?}", socket.peer_addr()?);
    }
}
