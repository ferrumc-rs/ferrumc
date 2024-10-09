use tokio::net::TcpStream;
use tracing::{debug, error, trace, warn};
use crate::{handle_packet, NetResult};
use crate::packets::incoming::PacketSkeleton;

pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
}
impl ConnectionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectionState::Handshaking => "handshake",
            ConnectionState::Status => "status",
            ConnectionState::Login => "login",
            ConnectionState::Play => "play",
        }
    }
}


pub async fn handle_connection(tcp_stream: TcpStream) -> NetResult<()> {
    let (reader, _writer) = tcp_stream.into_split();
    let mut reader = tokio::io::BufReader::new(reader);

    'net: loop {
        let mut packet_skele = PacketSkeleton::new(&mut reader).await?;

        trace!("Received packet: {:?}", packet_skele);

        if let Err(e) = handle_packet(
            packet_skele.id,
            0,
            ConnectionState::Handshaking,
            &mut packet_skele.data,
        ).await {
            warn!("Failed to handle packet: {:?}", e);
            // Kick the player (when implemented).
            break 'net;
        };
    }

    Ok(())
}