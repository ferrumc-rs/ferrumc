use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::network::packet::OutboundPacket;
use crate::network::packet::outbound::packet_play_out_status::PacketPlayOutStatus;
use crate::player::Player;
use crate::utils::write_varint;

pub mod state;

pub async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    println!("New connection from {}", stream.peer_addr()?);

    let player = Player::new(stream);
    player.init().await?;

    Ok(())
}