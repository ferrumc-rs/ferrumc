use std::collections::LinkedList;
use std::io::Cursor;
use std::sync::Arc;
use std::io::Read;

use log::{debug, info, trace};
use tokio::net::{TcpListener};
use tokio::sync::RwLock;

use byteorder::{BigEndian, ByteOrder, ReadBytesExt};

use crate::prelude::*;
use ferrumc_net;

mod error;
mod prelude;
mod utils;
mod constants;
mod tests;

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

    start_server(config.clone()).await.expect("Server failed to start!");

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
        let (mut socket, _) = listener.accept().await?;
        // show a line of 100 dashes
        trace!("{}", "-".repeat(100));
        trace!("Accepted connection from: {:?}", socket.peer_addr()?);

        tokio::task::spawn(ferrumc_net::handle_connection(socket));
    }
}



async fn handle_handshake(mut cursor: Cursor<Vec<u8>>) -> Result<()> {
    trace!("Handling handshake packet");

    let protocol_version = utils::encoding::varint::read_varint(&mut cursor)?;
    let server_address = utils::encoding::string::read_string(&mut cursor)?;
    let server_port = cursor.read_u16::<BigEndian>().unwrap();
    let next_state = utils::encoding::varint::read_varint(&mut cursor)?;

    trace!("Protocol Version: {}", protocol_version);
    trace!("Server Address: {}", server_address);
    trace!("Server Port: {}", server_port);
    trace!("Next State: {}", next_state);

    Ok(())
}