use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::{handle_packet, NetResult};
use ferrumc_state::ServerState;
use ferrumc_net_codec::encode::NetEncode;
use ferrumc_net_codec::encode::NetEncodeOpts;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tracing::{debug, debug_span, trace, warn, Instrument};

#[derive(Debug)]
pub struct ConnectionControl {
    pub should_disconnect: bool,
}

impl ConnectionControl {
    pub fn new() -> Self {
        Self {
            should_disconnect: false,
        }
    }
}

impl Default for ConnectionControl {
    fn default() -> Self {
        Self::new()
    }
}
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
    pub reader: OwnedReadHalf,
}

impl StreamReader {
    pub fn new(reader: OwnedReadHalf) -> Self {
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
    let (mut reader, writer) = tcp_stream.into_split();

    let entity = state
        .universe
        .builder()
        .with(StreamWriter::new(writer))?
        .with(ConnectionState::Handshaking)?
        .with(CompressionStatus::new())?
        .with(ConnectionControl::new())?
        .build();

    'recv: loop {
        let compressed = state.universe.get::<CompressionStatus>(entity)?.enabled;
        let should_disconnect = state
            .universe
            .get::<ConnectionControl>(entity)?
            .should_disconnect;

        if should_disconnect {
            debug!(
                "should_disconnect is true for entity: {}, breaking out of connection loop.",
                entity
            );
            break 'recv;
        }

        let read_timeout = Duration::from_secs(2);
        let packet_task = timeout(read_timeout, PacketSkeleton::new(&mut reader, compressed)).await;

        if let Err(err) = packet_task {
            trace!(
                "failed to read packet within {:?} for entity {:?}, err: {:?}
            continuing to next iteration",
                read_timeout,
                entity,
                err
            );
            continue;
        }

        let Ok(Ok(mut packet_skele)) = packet_task else {
            trace!("Failed to read packet. Possibly connection closed. Breaking out of connection loop");
            break 'recv;
        };

        // Log the packet if the environment variable is set (this env variable is set at compile time not runtime!)
        if option_env!("FERRUMC_LOG_PACKETS").is_some() {
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
        .instrument(debug_span!("eid", %entity))
        .inner()
        {
            warn!(
                "Failed to handle packet: {:?}. packet_id: {:02X}; conn_state: {}",
                e,
                packet_skele.id,
                conn_state.as_str()
            );
            // Kick the player (when implemented).
            terminate_connection(state.clone(), entity, "Failed to handle packet".to_string())
                .await?;
            break 'recv;
        };
    }

    debug!("Connection closed for entity: {:?}", entity);

    // Remove all components from the entity

    // Wait until anything that might be using the entity is done
    if let Err(e) = remove_all_components_blocking(state.clone(), entity).await {
        warn!("Failed to remove all components from entity: {:?}", e);
    }

    trace!("Dropped all components from entity: {:?}", entity);

    Ok(())
}

/// Since parking_lot is single-threaded, we use spawn_blocking to remove all components from the entity asynchronously (on another thread).
async fn remove_all_components_blocking(state: Arc<ServerState>, entity: usize) -> NetResult<()> {
    let res =
        tokio::task::spawn_blocking(move || state.universe.remove_all_components(entity)).await?;

    Ok(res?)
}
