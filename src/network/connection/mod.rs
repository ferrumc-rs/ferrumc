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
    // let mut buf = [0; 32];  // Made buffer to 32 bytes
    //
    // loop {
    //     let n = match stream.read(&mut buf).await {
    //         Ok(n) if n == 0 => return Ok(()),
    //         Ok(n) => n,
    //         Err(_e) => return Err(Error::msg("failed to read from socket")),
    //     };
    //
    //     let packet_data = Vec::from(&buf[0..n]);
    //
    //     println!("Received packet: {:?}", packet_data);
    //
    //     if packet_data[0] == 1 {
    //         handle_ping(&mut stream).await?;
    //         continue;
    //     }
    //
    //     if let Some(packet) = REGISTRY.deserialize_inbound(ConnectionState::Handshaking, packet_data).await {
    //         println!("Packet Id: {:?}", packet.get_id());
    //         handle_packet(packet, &mut stream).await;
    //     }
    // }
}


async fn handle_ping(stream: &mut TcpStream) -> Result<()> {
    let status_packet = PacketPlayOutStatus {
        motd: "Hello, this is a Minecraft server made in Rust.".to_string(),
    };

    if let Ok(serialized_data) = status_packet.serialize().await {
        let _ = stream.write_all((&serialized_data).as_ref()).await;
        // println!("sent data: {:?}", serialized_data);
    }

    // send a new buffer with [1,0] for ping
    let mut ping_buffer = vec![];
    write_varint(&mut ping_buffer, 0x01).await;
    write_varint(&mut ping_buffer, 0x00).await;

    let _ = stream.write_all((&ping_buffer).as_ref()).await;

    stream.flush().await.unwrap();

    Ok(())
}