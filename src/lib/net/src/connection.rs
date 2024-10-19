use crate::packets::incoming::PacketSkeleton;
use crate::packets::incoming::PacketSkeleton;
use crate::{handle_packet, NetResult, ServerState};
use crate::{handle_packet, NetResult, ServerState};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use std::sync::Arc;
use tokio::io::BufReader;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tracing::{debug, trace, warn};
use tracing::{trace, warn};

#[derive(Clone)]
#[repr(u8)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
    Configuration,
}
impl ConnectionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectionState::Handshaking => "handshake",
            ConnectionState::Status => "status",
            ConnectionState::Login => "login",
            ConnectionState::Play => "play",
            ConnectionState::Configuration => "configuration",
        }
    }
}

pub struct StreamReader {
    pub reader: BufReader<OwnedReadHalf>,
}

impl StreamReader {
    pub fn new(reader: BufReader<OwnedReadHalf>) -> Self {
        Self { reader }
    }
}

pub struct StreamWriter {
    pub writer: OwnedWriteHalf,
}

impl StreamWriter {
    pub fn new(writer: OwnedWriteHalf) -> Self {
        Self { writer }
    }

    pub async fn send_packet(
        &mut self,
        packet: &impl NetEncode,
        net_encode_opts: &NetEncodeOpts,
    ) -> NetResult<()> {
        packet
            .encode_async(&mut self.writer, net_encode_opts)
            .await?;
        Ok(())
    }
}

pub struct CompressionStatus {
    pub enabled: bool,
}

impl CompressionStatus {
    pub fn new() -> Self {
        Self { enabled: false }
    }
}

impl Default for CompressionStatus {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn handle_connection(state: Arc<ServerState>, tcp_stream: TcpStream) -> NetResult<()> {
    let (reader, writer) = tcp_stream.into_split();

    let entity = state
        .universe
        .builder()
        .with(StreamReader::new(BufReader::new(reader)))
        .with(StreamWriter::new(writer))
        .with(ConnectionState::Handshaking)
        .with(CompressionStatus::new())
        .build();

    let mut reader = state.universe.get_mut::<StreamReader>(entity)?;

    'recv: loop {
        let compressed = state.universe.get::<CompressionStatus>(entity)?.enabled;
        let Ok(mut packet_skele) = PacketSkeleton::new(&mut reader.reader, compressed).await else {
            warn!("Failed to read packet. Possibly connection closed.");
            break 'recv;
        };

        trace!("Received packet: {:?}", packet_skele);

        let conn_state = state.universe.get::<ConnectionState>(entity)?.clone();
        if let Err(e) = handle_packet(
            packet_skele.id,
            entity,
            &conn_state,
            &mut packet_skele.data,
            Arc::clone(&state),
        )
        .await
        {
            warn!("Failed to handle packet: {:?}", e);
            // Kick the player (when implemented).
            // Send a disconnect event
            break 'recv;
        };
    }

    debug!("Connection closed for entity: {:?}", entity);

    // Remove all components from the entity
    state.universe.remove_all_components(entity);

    Ok(())
}
