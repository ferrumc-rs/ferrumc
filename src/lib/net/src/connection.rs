use tokio::io::BufReader;
use std::sync::Arc;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tracing::{trace, warn};
use crate::{handle_packet, NetResult, ServerState};
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

/*pub struct ConnectionStream {
    pub reader: parking_lot::RwLock<BufReader<OwnedReadHalf>>,
    pub writer: parking_lot::RwLock<OwnedWriteHalf>,
}

pub type ConnectionStreamWrapper = Arc<ConnectionStream>;*/

pub type StreamReader = BufReader<OwnedReadHalf>;
pub type StreamWriter = OwnedWriteHalf;


pub async fn handle_connection(state: Arc<ServerState>, tcp_stream: TcpStream) -> NetResult<()> {
    let (reader, writer) = tcp_stream.into_split();

    let entity = state.universe
        .builder()
        .with(BufReader::new(reader))
        .with(writer)
        .with(ConnectionState::Handshaking)
        .build();


    let mut reader = state
        .universe
        .get_mut::<StreamReader>(entity)?;

    'recv: loop {
        let mut packet_skele = PacketSkeleton::new(&mut *reader).await?;

        trace!("Received packet: {:?}", packet_skele);

        if let Err(e) = handle_packet(
            packet_skele.id,
            entity,
            &*state.universe
                .get::<ConnectionState>(entity)?,
            &mut packet_skele.data,
            Arc::clone(&state),
        ).await {
            warn!("Failed to handle packet: {:?}", e);
            // Kick the player (when implemented).
            break 'recv;
        };
    }

    Ok(())
}