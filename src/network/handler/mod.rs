use tokio::net::TcpStream;

use crate::network::packet::{InboundPacket};

pub async fn handle_packet(packet: Box<dyn InboundPacket>, stream: &mut TcpStream) {
    packet.handle(stream).await;
}