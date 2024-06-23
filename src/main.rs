#![feature(box_into_inner)]

use std::sync::Arc;

use log::{debug, info, trace};
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use ferrumc_utils::prelude::*;


mod constants;
mod prelude;
mod tests;
mod utils;

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

/*async fn handle_handshake(mut cursor: Cursor<Vec<u8>>) -> Result<()> {
    trace!("Handling handshake packet");

    let protocol_version = ferrumc_utils::encoding::varint::read_varint(&mut cursor).await?;
    let server_address = ferrumc_utils::encoding::string::read_string(&mut cursor).await?;
    let server_port = cursor.read_u16::<BigEndian>().unwrap();
    let next_state = ferrumc_utils::encoding::varint::read_varint(&mut cursor).await?;

    trace!("Protocol Version: {}", protocol_version);
    trace!("Server Address: {}", server_address);
    trace!("Server Port: {}", server_port);
    trace!("Next State: {}", next_state);

    Ok(())
}
*/