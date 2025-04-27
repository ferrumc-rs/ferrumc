use crate::conn_init::handle_handshake;
use crate::errors::NetError::HandshakeTimeout;
use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::packets::{AnyIncomingPacket, IncomingPacket};
use crate::utils::state::terminate_connection;
use crate::{handle_packet, NetResult};
use crossbeam_queue::SegQueue;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::Event;
use ferrumc_net_codec::encode::NetEncode;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::ServerState;
use parking_lot::RwLock;
use std::cmp::PartialEq;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::timeout;
use tracing::{debug, debug_span, error, trace, warn, Instrument};
use typename::TypeName;

/// The maximum time to wait for a handshake to complete
const MAX_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
#[derive(TypeName, Debug)]
pub struct ConnectionControl {
    pub should_disconnect: bool,
}
#[derive(TypeName, Debug)]
pub struct LocalPacketQueue {
    pub queue: SegQueue<AnyIncomingPacket>,
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
#[derive(TypeName)]
pub struct StreamReader {
    pub reader: Arc<RwLock<TcpStream>>,
}

impl StreamReader {
    pub fn new(reader: Arc<RwLock<TcpStream>>) -> Self {
        Self { reader }
    }
}

#[derive(TypeName)]
pub struct StreamWriter {
    sender: UnboundedSender<Vec<u8>>,
    running: Arc<AtomicBool>,
}
impl Drop for StreamWriter {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
impl StreamWriter {
    pub async fn new(mut writer: OwnedWriteHalf) -> Self {
        let (sender, mut receiver): (UnboundedSender<Vec<u8>>, UnboundedReceiver<Vec<u8>>) =
            tokio::sync::mpsc::unbounded_channel();
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        // Spawn a task to write to the writer using the channel
        tokio::spawn(async move {
            while running_clone.load(Ordering::Relaxed) {
                let Some(bytes) = receiver.recv().await else {
                    break;
                };

                if let Err(e) = writer.write_all(&bytes).await {
                    warn!("Failed to write to writer: {:?}", e);
                    running_clone.store(false, Ordering::Relaxed);
                    break;
                }
            }
        });

        Self { sender, running }
    }

    pub fn send_packet(
        &mut self,
        packet: impl NetEncode + Send,
        net_encode_opts: &NetEncodeOpts,
    ) -> NetResult<()> {
        let bytes = {
            let mut buffer = Vec::new();
            packet.encode(&mut buffer, net_encode_opts)?;
            buffer
        };
        self.sender.send(bytes).map_err(std::io::Error::other)?;
        Ok(())
    }
}
#[derive(TypeName)]
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
    let (mut tcp_reader, mut tcp_writer) = tcp_stream.into_split();

    let handshake_result = timeout(
        MAX_HANDSHAKE_TIMEOUT,
        handle_handshake(&mut tcp_reader, &mut tcp_writer, state.clone()),
    )
    .await;

    match handshake_result {
        Ok(res) => match res {
            Ok(false) => {
                debug!("Handshake successful");
            }
            Ok(true) => {
                debug!("Handshake successful, killing connection");
                return Ok(());
            }
            Err(err) => {
                error!("Handshake error: {:?}", err);
                return Err(err);
            }
        },
        Err(err) => {
            error!("Handshake timed out: {:?}", err);
            return Err(HandshakeTimeout);
        }
    }

    // The player has successfully connected, so we can start the connection properly
    let entity = state
        .universe
        .builder()
        .with(StreamWriter::new(tcp_writer).await)?
        .with(CompressionStatus::new())?
        .with(ConnectionControl::new())?
        .with(LocalPacketQueue {
            queue: SegQueue::new(),
        })?
        .with(ChunkReceiver::new())?
        .with(Position::default())?
        .with(OnGround(false))?
        .with(Rotation::default())?
        .build();

    {
        let chunk_recv = state.universe.get::<&mut ChunkReceiver>(entity)?;
    }

    'recv: loop {
        let compressed = state.universe.get::<CompressionStatus>(entity)?.enabled;
        let should_disconnect = state
            .universe
            .get::<ConnectionControl>(entity)?
            .should_disconnect;

        if should_disconnect {
            trace!("Conn for entity {:?} is marked for disconnection", entity);
            break 'recv;
        }

        if state.shut_down.load(Ordering::Relaxed) {
            break 'recv;
        }

        let mut packet_skele = match PacketSkeleton::new(&mut tcp_reader, compressed).await {
            Ok(packet_skele) => packet_skele,
            Err(err) => {
                // Handle the error
                debug!(
                    "Failed to read packet for entity {:?}, err: {:?}. Continuing to next iteration",
                    entity, err
                );
                continue 'recv;
            }
        };

        match handle_packet(
            packet_skele.id,
            entity,
            &mut packet_skele.data,
            Arc::clone(&state),
        )
        .instrument(debug_span!("eid", %entity))
        .into_inner()
        {
            Ok(Some(pak)) => {
                let queue = state.universe.get::<LocalPacketQueue>(entity)?;
                queue.queue.push(pak);
            }
            Ok(None) => {
                // No packet found for the given ID and state
                debug!("No packet found for ID: 0x{:02X}", packet_skele.id);
            }
            Err(e) => {
                // Failed to handle the packet
                debug!(
                    "Failed to handle packet: {:?}. packet_id: {:02X}",
                    e, packet_skele.id
                );

                disconnect(state.clone(), entity);
                break 'recv;
            }
        };
    }

    Ok(())
}

#[derive(Event)]
pub struct PlayerDisconnectEvent {
    pub entity_id: usize,
}

fn remove_all_components_blocking(state: Arc<ServerState>, entity: usize) -> NetResult<()> {
    let res = state.universe.remove_all_components(entity);

    Ok(res?)
}

fn disconnect(state: Arc<ServerState>, entity: usize) {
    debug!("Connection closed for entity: {:?}", entity);

    // Broadcast the leave server event

    _ = PlayerDisconnectEvent::trigger(PlayerDisconnectEvent { entity_id: entity }, state.clone());

    // Remove all components from the entity

    terminate_connection(state.clone(), entity, "Failed to handle packet".to_string())
        .expect("Failed to terminate connection");

    // Wait until anything that might be using the entity is done
    if let Err(e) = remove_all_components_blocking(state.clone(), entity) {
        warn!("Failed to remove all components from entity: {:?}", e);
    }

    trace!("Dropped all components from entity: {:?}", entity);
}
