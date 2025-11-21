use crate::compression::compress_packet;
use crate::conn_init::handle_handshake;
use crate::errors::CompressionError::GenericCompressionError;
use crate::errors::NetError;
use crate::errors::NetError::HandshakeTimeout;
use crate::errors::PacketError::InvalidPacket;
use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::ConnState::Play;
use crate::{handle_packet, PacketSender};
use bevy_ecs::prelude::{Component, Entity};
use crossbeam_channel::Sender;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net_codec::encode::NetEncode;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_encryption::read::EncryptedReader;
use ferrumc_net_encryption::write::EncryptedWriter;
use ferrumc_state::ServerState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use tokio::time::timeout;
use tracing::{debug, debug_span, error, trace, warn, Instrument};
use typename::TypeName;

/// The maximum time allowed for a client to complete its initial handshake.
/// Connections exceeding this duration will be dropped to avoid resource hogging.
const MAX_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);

/// StreamWriter manages asynchronous writes to a client's TCP connection.
///
/// It:
/// - Buffers outgoing packets via a Tokio `mpsc` channel.
/// - Runs a background task that writes packets to the underlying socket.
/// - Supports toggling compression dynamically.
/// - Gracefully handles disconnection when dropped.
#[derive(TypeName, Component)]
pub struct StreamWriter {
    sender: UnboundedSender<WriterCommand>,
    pub running: Arc<AtomicBool>,
    pub compress: Arc<AtomicBool>,
    pub state: Arc<ServerState>,
    pub entity: Arc<Mutex<Option<Entity>>>,
}

impl Drop for StreamWriter {
    /// When the writer is dropped, mark the connection as no longer active.
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

impl StreamWriter {
    /// Creates a new StreamWriter for the connection's write half.
    ///
    /// Spawns a background task that continuously reads from the channel
    /// and writes bytes to the network socket.
    pub async fn new(
        mut writer: EncryptedWriter<OwnedWriteHalf>,
        running: Arc<AtomicBool>,
        state: Arc<ServerState>,
        entity: Arc<Mutex<Option<Entity>>>,
    ) -> Self {
        let compress = Arc::new(AtomicBool::new(false)); // Default: no compression
        let (sender, mut receiver): (
            UnboundedSender<WriterCommand>,
            UnboundedReceiver<WriterCommand>,
        ) = tokio::sync::mpsc::unbounded_channel();
        let running_clone = running.clone();
        let entity_clone = entity.clone();
        let state_clone = state.clone();

        // Task: forward packets from channel to socket
        tokio::spawn(async move {
            while running_clone.load(Ordering::Relaxed) {
                let Some(cmd) = receiver.recv().await else {
                    break;
                };

                match cmd {
                    WriterCommand::SendPacket(bytes) => {
                        // This handles ONLY if there was a writing error to the client.
                        if let Err(e) = writer.write_all(&bytes).await {
                            error!("Failed to write to client: {:?}", e);
                            running_clone.store(false, Ordering::Relaxed);
                            if let Some(entity_id) = *entity_clone.lock().unwrap() {
                                state_clone.players.disconnect(entity_id, None);
                            }
                            break;
                        }
                    }
                    WriterCommand::CipherKey(new_key) => {
                        writer.update_cipher(&new_key);
                    }
                }
            }

            // This handles cases where the channel closes without a write error
            if let Some(entity_id) = *entity_clone.lock().unwrap() {
                trace!(
                    "Write task ending for entity {:?}, ensuring disconnect",
                    entity_id
                );
                state_clone.players.disconnect(entity_id, None);
            }
        });

        Self {
            sender,
            running,
            compress,
            state,
            entity,
        }
    }

    /// Sets the entity ID for this stream writer.
    /// This should be called after the entity is created in the ECS.
    pub fn set_entity(&self, entity: Entity) {
        *self.entity.lock().unwrap() = Some(entity);
    }

    /// Sends a packet to the client using the default `WithLength` encoding.
    pub fn send_packet(&self, packet: impl NetEncode + Send) -> Result<(), NetError> {
        self.send_packet_with_opts(&packet, &NetEncodeOpts::WithLength)
    }

    /// Sends a packet reference using the default `WithLength` encoding.
    pub fn send_packet_ref(&self, packet: &(impl NetEncode + Send)) -> Result<(), NetError> {
        self.send_packet_with_opts(packet, &NetEncodeOpts::WithLength)
    }

    /// Sends a packet with custom encoding options (e.g., with or without length prefix).
    ///
    /// Handles optional compression based on `self.compress` flag.
    pub fn send_packet_with_opts(
        &self,
        packet: &(impl NetEncode + Send),
        net_encode_opts: &NetEncodeOpts,
    ) -> Result<(), NetError> {
        if !self.running.load(Ordering::Relaxed) {
            #[cfg(debug_assertions)]
            warn!("Attempted to send packet on closed connection");
            return Err(NetError::ConnectionDropped);
        }

        let raw_bytes = compress_packet(
            packet,
            self.compress.load(Ordering::Relaxed),
            net_encode_opts,
        )
        .map_err(|err| {
            error!("Failed to compress packet: {:?}", err);
            NetError::CompressionError(GenericCompressionError(format!(
                "Failed to compress packet: {:?}",
                err
            )))
        })?;

        self.sender
            .send(WriterCommand::SendPacket(raw_bytes))
            .map_err(std::io::Error::other)?;
        Ok(())
    }

    /// Sends pre-encoded raw bytes to the client without additional processing.
    pub fn send_raw_packet(&self, raw_bytes: Vec<u8>) -> Result<(), NetError> {
        if !self.running.load(Ordering::Relaxed) {
            #[cfg(debug_assertions)]
            warn!("Attempted to send raw bytes on closed connection");
            return Err(NetError::ConnectionDropped);
        }

        self.sender
            .send(WriterCommand::SendPacket(raw_bytes))
            .map_err(std::io::Error::other)?;
        Ok(())
    }

    /// Sends a message to the outgoing packet writer to update its encryption keys
    pub fn update_encryption_cipher(&self, new_key: &[u8]) -> Result<(), NetError> {
        if !self.running.load(Ordering::Relaxed) {
            #[cfg(debug_assertions)]
            warn!("Attempted to update encryption cipher on closed connection");
            return Err(NetError::ConnectionDropped);
        }

        self.sender
            .send(WriterCommand::CipherKey(new_key.to_vec()))
            .map_err(std::io::Error::other)?;
        Ok(())
    }
}

/// Contains information about a newly established connection that
/// needs to be registered with the game world.
pub struct NewConnection {
    pub stream: StreamWriter,
    pub player_identity: PlayerIdentity,
    pub entity_return: oneshot::Sender<Entity>,
    pub disconnect_handle: oneshot::Sender<()>,
}

#[derive(Component)]
pub struct DisconnectHandle {
    pub sender: Option<oneshot::Sender<()>>,
}

/// Handles a new incoming client connection.
///
/// Responsibilities:
/// 1. Perform the initial handshake (with timeout protection).
/// 2. Validate and register the player.
/// 3. Transfer the connection to the ECS world and packet dispatcher.
/// 4. Enter the packet receive loop for ongoing gameplay communication.
///
/// # Parameters
/// - `state`: Shared global server state.
/// - `tcp_stream`: The accepted client socket.
/// - `packet_sender`: Channel to the packet handling system.
/// - `new_join_sender`: Channel to register the new connection in the ECS.
///
/// # Errors
/// Returns `NetError` on handshake failure, timeout, or network IO issues.
pub async fn handle_connection(
    state: Arc<ServerState>,
    tcp_stream: TcpStream,
    packet_sender: Arc<PacketSender>,
    new_join_sender: Arc<Sender<NewConnection>>,
) -> Result<(), NetError> {
    let (tcp_reader, tcp_writer) = tcp_stream.into_split();

    let mut tcp_reader = EncryptedReader::from(tcp_reader);
    let tcp_writer = EncryptedWriter::from(tcp_writer);

    let running = Arc::new(AtomicBool::new(true));

    let entity_holder: Arc<Mutex<Option<Entity>>> = Arc::new(Mutex::new(None));

    let stream = StreamWriter::new(
        tcp_writer,
        running.clone(),
        state.clone(),
        entity_holder.clone(),
    )
    .await;

    let handshake_result = timeout(
        MAX_HANDSHAKE_TIMEOUT,
        handle_handshake(&mut tcp_reader, &stream, state.clone()),
    )
    .await;

    let login_result = match handshake_result {
        // Handshake completed within timeout
        Ok(res) => match res {
            // Normal login flow (keep connection alive)
            Ok((false, login_result)) => {
                trace!("Handshake successful");
                match &login_result.player_identity {
                    Some(given_player_identity) => {
                        trace!("Player identity: {:?}", given_player_identity);
                    }
                    None => {
                        error!("Player identity not found after handshake");
                        return Err(NetError::Misc("Player identity not found".to_string()));
                    }
                }
                login_result
            }
            // Connection should be closed after handshake
            Ok((true, _)) => {
                trace!("Handshake successful but connection will be closed");
                return Ok(());
            }
            // Handshake returned an error
            Err(err) => {
                match &err {
                    NetError::MismatchedProtocolVersion(client_version, server_version) => {
                        warn!(
                            "Client version mismatch: {} (server {})",
                            client_version, server_version
                        );
                    }
                    NetError::InvalidState(state) => {
                        warn!("Client sent invalid handshake state: {}", state);
                    }
                    _ => {
                        error!("Unhandled handshake error: {}", err);
                    }
                }
                return Err(err);
            }
        },
        // Timeout expired before handshake completed
        Err(err) => {
            error!("Handshake timed out: {:?}", err);
            return Err(HandshakeTimeout);
        }
    };

    // Send the new connection data to ECS world
    let (entity_return, entity_recv) = oneshot::channel();
    let (disconnect_return, mut disconnect_receiver) = oneshot::channel();

    new_join_sender
        .send(NewConnection {
            stream,
            player_identity: login_result.player_identity.unwrap_or_default(),
            entity_return,
            disconnect_handle: disconnect_return,
        })
        .map_err(|_| NetError::Misc("Failed to register new connection".to_string()))?;

    // Await the entity ID assigned by ECS
    let entity = match entity_recv.await {
        Ok(entity) => entity,
        Err(err) => {
            error!("Failed to receive entity ID: {:?}", err);
            return Err(NetError::Misc("Failed to receive entity ID".to_string()));
        }
    };

    // Sets the entity for the stream writer.
    *entity_holder.lock().unwrap() = Some(entity);

    trace!("Entity {:?} assigned to connection", entity);

    // ---- Packet receive loop ----
    'recv: loop {
        if !running.load(Ordering::Relaxed) {
            trace!("Entity {:?} connection flagged for disconnect", entity);
            break 'recv;
        }

        if state.shut_down.load(Ordering::Relaxed) {
            break 'recv;
        }

        // Read next packet
        let mut packet_skele;
        tokio::select! {
            packet_result = PacketSkeleton::new(&mut tcp_reader, login_result.compression, Play) => {
                match packet_result {
                    Ok(packet) => {
                        packet_skele = packet;
                    },
                    Err(err) => {
                        if let NetError::ConnectionDropped = err {
                            trace!("Connection dropped for entity {:?}", entity);
                            running.store(false, Ordering::Relaxed);
                            state.players.disconnect(entity, None);
                        } else {
                            error!("Failed to read packet skeleton: {:?} for {:?}", err, entity);
                            running.store(false, Ordering::Relaxed);
                            state.players.disconnect(entity, None);
                        }
                        break 'recv;
                    }
                }
            }

            _ = &mut disconnect_receiver => {
                debug!("Received disconnect signal for entity {:?}", entity);
                running.store(false, Ordering::Relaxed);
                break 'recv;
            }
        }

        // Dispatch packet to handler
        match handle_packet(
            packet_skele.id,
            entity,
            &mut packet_skele.data,
            packet_sender.clone(),
        )
        .instrument(debug_span!("eid", %entity))
        .into_inner()
        {
            Ok(()) => {
                trace!(
                    "Packet {:02X} successfully handled for entity {:?}",
                    packet_skele.id,
                    entity
                );
            }
            Err(err) => match &err {
                NetError::Packet(InvalidPacket(id)) => {
                    trace!(
                        "Unimplemented packet 0x{:02X} received for entity {:?}",
                        id,
                        entity
                    );
                }
                _ => {
                    warn!("Error handling packet for {:?}: {:?}", entity, err);
                    running.store(false, Ordering::Relaxed);
                    state.players.disconnect(entity, None);
                    break 'recv;
                }
            },
        }
    }

    Ok(())
}

enum WriterCommand {
    SendPacket(Vec<u8>),
    CipherKey(Vec<u8>),
}
