use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::{handle_packet, NetResult, ServerState};
use ferrumc_ecs::components::{ComponentRefMut, ComponentStorage};
use ferrumc_ecs::entities::Entity;
use ferrumc_ecs::query::QueryItem;
use ferrumc_ecs::ECSResult;
use ferrumc_net_codec::encode::NetEncode;
use ferrumc_net_codec::encode::NetEncodeOpts;
use std::sync::Arc;
use tokio::io::BufReader;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tracing::{debug, trace, warn};

#[derive(Clone)]
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
impl QueryItem for StreamWriter {
    type Item<'a> = ComponentRefMut<'a, StreamWriter>;

    /// Fetches a mutable reference to the `StreamWriter` for the given `entity`.
    fn fetch<'a>(entity: Entity, storage: &ComponentStorage) -> ECSResult<Self::Item<'a>> {
        storage.get_mut(entity)
    }

    /// Retrieves a list of entities that possess a `StreamWriter` component.
    fn entities(storage: &ComponentStorage) -> Vec<Entity> {
        storage.get_entities_with::<StreamWriter>()
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
        if ferrumc_config::get_global_config().log_packets{
        trace!("Received packet: {:?}", packet_skele);


        }


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
