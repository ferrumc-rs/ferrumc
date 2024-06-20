#![allow(unused)]

use std::collections::LinkedList;
use std::io::Cursor;
use std::sync::Arc;

use log::{debug, info, trace};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener};
use tokio::sync::RwLock;

use crate::prelude::*;
use ferrumc_net;

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
        let (mut socket, _) = listener.accept().await?;
        // show a line of 100 dashes
        trace!("{}", "-".repeat(100));
        trace!("Accepted connection from: {:?}", socket.peer_addr()?);

        ferrumc_net::handle_connection(socket);
    }
}

// async fn handle_connection(mut socket: tokio::net::TcpStream) -> Result<()> {
//     let mut length_buffer = vec![0u8; 1];
//     socket.read_exact(&mut length_buffer).await?;
//
//     // trace!("Received length: {:?}", length_buffer);
//
//     let length = length_buffer[0] as usize;
//
//     // trace!("Reading {} bytes", length);
//
//     let mut buffer = vec![0u8; length];
//
//     socket.read_exact(&mut buffer).await?;
//
//     // trace!("Received buffer: {:?}", buffer);
//
//     let mut buffer = vec![length_buffer, buffer].concat();
//
//     let mut cursor = Cursor::new(buffer);
//
//     let packet_length = read_varint(&mut cursor).await?;
//     let packet_id = read_varint(&mut cursor).await?;
//
//     trace!("Packet Length: {}", packet_length);
//     trace!("Packet ID: {}", packet_id);
//
//     match packet_id {
//         0 => handle_handshake(cursor).await?,
//         _ => {
//             log::warn!("Unknown packet id: {}", packet_id);
//         }
//     }
//
//
//     Ok(())
// }

async fn read_string(cursor: &mut Cursor<Vec<u8>>) -> Result<String> {
    let length = utils::varint::read_varint(cursor).await?;
    let mut buffer = vec![0u8; length as usize];
    cursor.read_exact(&mut buffer).await?;
    Ok(String::from_utf8(buffer)?)
}

async fn handle_handshake(mut cursor: Cursor<Vec<u8>>) -> Result<()> {
    trace!("Handling handshake packet");

    let protocol_version = utils::varint::read_varint(&mut cursor).await?;
    let server_address = read_string(&mut cursor).await?;
    let server_port = cursor.read_u16().await?;
    let next_state = utils::varint::read_varint(&mut cursor).await?;

    trace!("Protocol Version: {}", protocol_version);
    trace!("Server Address: {}", server_address);
    trace!("Server Port: {}", server_port);
    trace!("Next State: {}", next_state);

    Ok(())
}