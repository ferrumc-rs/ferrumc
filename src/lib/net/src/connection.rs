use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::packets::IncomingPacket;
use crate::utils::state::terminate_connection;
use crate::{handle_packet, NetResult};
use crossbeam_channel::{Receiver, Sender};
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::Event;
use ferrumc_net_codec::encode::NetEncode;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::ServerState;
use parking_lot::RwLock;
use std::io::Write;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
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
    sender: Sender<Vec<u8>>,
    running: Arc<AtomicBool>,
}
impl Drop for StreamWriter {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
impl StreamWriter {
    pub fn new(mut writer: TcpStream) -> Self {
        let (sender, receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) =
            crossbeam_channel::unbounded();
        let running = Arc::new(AtomicBool::new(true));

        // Spawn a task to write to the writer using the channel
        thread::spawn({
            let running = Arc::clone(&running);
            move || {
                while running.load(Ordering::Relaxed) {
                    let Ok(bytes) = receiver.recv() else {
                        break;
                    };

                    if let Err(e) = writer.write_all(&bytes) {
                        warn!("Failed to write to writer: {:?}", e);
                        break;
                    }
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

pub fn handle_connection(
    state: Arc<ServerState>,
    tcp_stream: TcpStream,
    packet_queue: Arc<Mutex<Vec<(Box<dyn IncomingPacket + Send + 'static>, usize)>>>,
) -> NetResult<()> {
    let entity = state
        .universe
        .builder()
        .with(StreamWriter::new(tcp_stream.try_clone()?))?
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
        let (tx, rx) = crossbeam_channel::bounded(1);
        thread::spawn({
            let mut tcp_stream = tcp_stream.try_clone().expect("Unable to clone tcp stream");
            move || tx.send(PacketSkeleton::new(&mut tcp_stream, compressed))
        });
        let res = rx.recv_timeout(read_timeout);

        if let Err(err) = res {
            trace!(
                "failed to read packet within {:?} for entity {:?}, err: {:?}
            continuing to next iteration",
                read_timeout,
                entity,
                err
            );
            continue;
        }

        let Ok(Ok(mut packet_skele)) = res else {
            trace!("Failed to read packet. Possibly connection closed. Breaking out of connection loop");
            break 'recv;
        };

        // Log the packet if the environment variable is set (this env variable is set at compile time not runtime!)
        if option_env!("FERRUMC_LOG_PACKETS").is_some() {
            trace!("Received packet: {:?}", packet_skele);
        }

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
                // Kick the player (when implemented).
                terminate_connection(state.clone(), entity, "Failed to handle packet".to_string())?;
                break 'recv;
            }
        };
    }

    debug!("Connection closed for entity: {:?}", entity);

    // Broadcast the leave server event

    let _ =
        PlayerDisconnectEvent::trigger(PlayerDisconnectEvent { entity_id: entity }, state.clone());

    // Remove all components from the entity

    // Wait until anything that might be using the entity is done
    if let Err(e) = remove_all_components_blocking(state.clone(), entity) {
        warn!("Failed to remove all components from entity: {:?}", e);
    }

    trace!("Dropped all components from entity: {:?}", entity);

    Ok(())
}

#[derive(Event)]
pub struct PlayerDisconnectEvent {
    pub entity_id: usize,
}

/// Since parking_lot is single-threaded, we use spawn_blocking to remove all components from the entity asynchronously (on another thread).
fn remove_all_components_blocking(state: Arc<ServerState>, entity: usize) -> NetResult<()> {
    let res = state.universe.remove_all_components(entity);

    Ok(res?)
}
