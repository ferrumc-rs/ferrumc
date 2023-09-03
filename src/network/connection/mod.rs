use anyhow::{Error, Result};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::network::handler::handle_packet;
use crate::network::packet::PacketRegistry;

pub async fn handle_connection(mut socket: TcpStream) -> Result<()> {
    println!("New connection from {}", socket.peer_addr()?);

    let mut buf = [0; 1024];

    let instance = PacketRegistry::instance();

    let registry_guard = instance.lock().await;

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(n) if n == 0 => return Ok(()),
            Ok(n) => n,
            Err(_e) => return Err(Error::msg("failed to read from socket")),
        };

        let packet_data = Vec::from(&buf[0..n]);

        println!("Received packet: {:?}", packet_data);

        if packet_data[0] == 1 {
            continue;
        }


        if let Some(packet) = registry_guard.deserialize_inbound(packet_data) {
            println!("Packet Id: {:?}", packet.get_id());
            handle_packet(packet, &*registry_guard, &mut socket).await;
        }
    }
}
