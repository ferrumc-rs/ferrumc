use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::packets::AnyIncomingPacket;
use crate::utils::state::terminate_connection;
use crate::{handle_packet, NetResult};
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::Event;
use ferrumc_net_codec::encode::NetEncode;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::ServerState;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
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
    pub reader: Arc<RwLock<TcpStream>>,
}

impl StreamReader {
    pub fn new(reader: Arc<RwLock<TcpStream>>) -> Self {
        Self { reader }
    }
}

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
                debug!("Got bytes from remote");

                if let Err(e) = writer.write_all(&bytes).await {
                    warn!("Failed to write to writer: {:?}", e);
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
        debug!("Sending packet to recv task");
        self.sender.send(bytes).map_err(std::io::Error::other)?;
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

pub async fn handle_connection(
    state: Arc<ServerState>,
    tcp_stream: TcpStream,
    packet_queue: Arc<Mutex<Vec<(AnyIncomingPacket, usize)>>>,
) -> NetResult<()> {
    let (mut tcp_reader, tcp_writer) = tcp_stream.into_split();

    let entity = state
        .universe
        .builder()
        .with(StreamWriter::new(tcp_writer))?
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
            trace!("Conn for entity {:?} is marked for disconnection", entity);
            break 'recv;
        }

        if state.shut_down.load(Ordering::Relaxed) {
            break 'recv;
        }

        let mut packet_skele = match PacketSkeleton::new(&mut tcp_reader, compressed).await {
            Ok(packet_skele) => {
                // Log the packet if the environment variable is set (this env variable is set at compile time not runtime!)
                packet_skele
            }
            Err(err) => {
                // Handle the error
                debug!(
                    "Failed to read packet for entity {:?}, err: {:?}. Continuing to next iteration",
                    entity, err
                );
                continue 'recv;
            }
        };

        let conn_state = state.universe.get::<ConnectionState>(entity)?.clone();
        match handle_packet(
            packet_skele.id,
            entity,
            &conn_state,
            &mut packet_skele.data,
            Arc::clone(&state),
        )
        .instrument(debug_span!("eid", %entity))
        .into_inner()
        {
            Ok(Some(pak)) => packet_queue.lock().unwrap().push((pak, entity)),
            Ok(None) => {
                // No packet found for the given ID and state
                debug!(
                    "No packet found for ID: 0x{:02X} in state: {}",
                    packet_skele.id,
                    conn_state.as_str()
                );
            }
            Err(e) => {
                // Failed to handle the packet
                debug!(
                    "Failed to handle packet: {:?}. packet_id: {:02X}; conn_state: {}",
                    e,
                    packet_skele.id,
                    conn_state.as_str()
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

    let _ =
        PlayerDisconnectEvent::trigger(PlayerDisconnectEvent { entity_id: entity }, state.clone());

    // Remove all components from the entity

    terminate_connection(state.clone(), entity, "Failed to handle packet".to_string())
        .expect("Failed to terminate connection");

    // Wait until anything that might be using the entity is done
    if let Err(e) = remove_all_components_blocking(state.clone(), entity) {
        warn!("Failed to remove all components from entity: {:?}", e);
    }

    trace!("Dropped all components from entity: {:?}", entity);
}
