use tokio::net::TcpStream;
use crate::network::packet::PacketRegistry;
use crate::network::packet::Packet;

pub async  fn handle_packet(packet: Box<dyn Packet>, registry: &PacketRegistry, stream: &mut TcpStream) {
    let id = packet.get_id();
    if let Some(_packet_handler) = registry.inbound.get(&id) {
        packet.handle(stream).await;
    }
}
