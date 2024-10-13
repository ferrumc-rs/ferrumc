use crate::packets::incoming::PacketSkeleton;
use crate::{handle_packet, NetResult, ServerState};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use std::sync::Arc;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tracing::{trace, warn};

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

pub struct StreamReader {
    pub reader: BufReader<OwnedReadHalf>,
    pub compressed: bool,
}

impl StreamReader {
    pub fn new(reader: BufReader<OwnedReadHalf>) -> Self {
        Self {
            reader,
            compressed: false,
        }
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
        let mut buf = Vec::new();
        packet.encode(&mut buf, net_encode_opts)?;
        self.writer.write_all(buf.as_slice()).await?;

        Ok(())
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
        .build();

    let mut reader = state.universe.get_mut::<StreamReader>(entity)?;

    'recv: loop {
        let compressed = reader.compressed;
        let mut packet_skele = PacketSkeleton::new(&mut reader.reader, compressed).await?;

        trace!("Received packet: {:?}", packet_skele);

        if let Err(e) = handle_packet(
            packet_skele.id,
            entity,
            &*state.universe.get::<ConnectionState>(entity)?,
            &mut packet_skele.data,
            Arc::clone(&state),
        )
        .await
        {
            warn!("Failed to handle packet: {:?}", e);
            // Kick the player (when implemented).
            break 'recv;
        };
    }

    Ok(())
}
