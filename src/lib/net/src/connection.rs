use std::io::{Cursor, Read};
use crate::conn_init::handle_handshake;
use crate::errors::NetError;
use crate::errors::NetError::HandshakeTimeout;
use crate::errors::PacketError::InvalidPacket;
use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::{handle_packet, PacketSender};
use bevy_ecs::prelude::{Component, Entity};
use crossbeam_channel::Sender;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net_codec::encode::NetEncode;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::ServerState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use tokio::time::timeout;
use tracing::{debug_span, error, trace, warn, Instrument};
use typename::TypeName;
use yazi::{CompressionLevel, Format};
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use crate::ConnState::Play;

/// The maximum time to wait for a handshake to complete
const MAX_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(TypeName, Component)]
pub struct StreamWriter {
    sender: UnboundedSender<Vec<u8>>,
    pub running: Arc<AtomicBool>,
    pub compress: Arc<AtomicBool>,
}
impl Drop for StreamWriter {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

fn get_id(packet: &(impl NetEncode + Send)) -> Result<VarInt, NetError> {
    let mut buffer = Vec::new();
    packet.encode(&mut buffer, &NetEncodeOpts::None)?;
    let id = VarInt::decode(&mut Cursor::new(buffer), &NetDecodeOpts::None)?;
    Ok(id)
}
impl StreamWriter {
    pub async fn new(mut writer: OwnedWriteHalf, running: Arc<AtomicBool>) -> Self {
        let compress = Arc::new(AtomicBool::new(false)); // Default to no compression
        let (sender, mut receiver): (UnboundedSender<Vec<u8>>, UnboundedReceiver<Vec<u8>>) =
            tokio::sync::mpsc::unbounded_channel();
        let running_clone = running.clone();

        // Spawn a task to write to the writer using the channel
        tokio::spawn(async move {
            while running_clone.load(Ordering::Relaxed) {
                let Some(bytes) = receiver.recv().await else {
                    break;
                };

                if let Err(e) = writer.write_all(&bytes).await {
                    error!("Failed to write to writer: {:?}", e);
                    running_clone.store(false, Ordering::Relaxed);
                    break;
                }
            }
        });

        Self { sender, running, compress }
    }

    // Sends the packet to the client with the default options. You probably want to use this instead
    // of send_packet_with_opts()
    pub fn send_packet(&self, packet: impl NetEncode + Send) -> Result<(), NetError> {
        self.send_packet_with_opts(&packet, &NetEncodeOpts::WithLength)
    }

    pub fn send_packet_ref(
        &self,
        packet: &(impl NetEncode + Send),
    ) -> Result<(), NetError> {
        self.send_packet_with_opts(packet, &NetEncodeOpts::WithLength)
    }

    pub fn send_packet_with_opts(
        &self,
        packet: &(impl NetEncode + Send),
        net_encode_opts: &NetEncodeOpts,
    ) -> Result<(), NetError> {
        use std::io::{Cursor, Read};
        use std::sync::atomic::Ordering;
        use yazi::{compress, CompressionLevel, Format};

        if !self.running.load(Ordering::Relaxed) {
            #[cfg(debug_assertions)]
            warn!("StreamWriter is not running, not sending packet");
            return Err(NetError::ConnectionDropped);
        }

        // Helper: encode full frame (outer length + id + body), then split into (id_varint, body)
        fn encode_id_and_body(pkt: &(impl NetEncode + Send)) -> Result<(VarInt, Vec<u8>), NetError> {
            // Authoritative path: whatever "normal" is for your packets on the wire.
            let mut full = Vec::new();
            // This MUST be the same mode your plain/uncompressed path uses.
            pkt.encode(&mut full, &NetEncodeOpts::WithLength)?;

            // Strip outer length
            let mut cur = Cursor::new(full);
            let _outer_len = VarInt::read(&mut cur)?; // discard

            // Read ID (keep the actual VarInt for length calculations if you need)
            let id_vi = VarInt::read(&mut cur)?;

            // Remaining = body
            let mut body = Vec::new();
            cur.read_to_end(&mut body)?;
            Ok((id_vi, body))
        }

        let raw_bytes = if self.compress.load(Ordering::Relaxed) {
            // Build a canonical uncompressed frame (ID + BODY) from the authoritative encode
            let (id_vi, body) = encode_id_and_body(packet)?;

            let mut uncompressed_frame = Vec::with_capacity(id_vi.len() + body.len());
            id_vi.encode(&mut uncompressed_frame, &NetEncodeOpts::None)?;
            uncompressed_frame.extend_from_slice(&body);

            // Compare against threshold using the *uncompressed frame length* (ID+BODY)
            let threshold = get_global_config().network_compression_threshold as usize;

            // Inner payload = VarInt(uncompressed_len or 0) + (compressed or uncompressed_frame)
            let mut inner = Vec::new();
            if uncompressed_frame.len() >= threshold {
                // compress ID+BODY
                let compressed = compress(
                    &uncompressed_frame,
                    Format::Zlib,
                    CompressionLevel::Default,
                )
                    .map_err(|err| {
                        error!("Failed to compress packet: {:?}", err);
                        NetError::CompressionError(format!("Failed to compress packet: {:?}", err))
                    })?;

                // write uncompressed_length (as VarInt), then compressed bytes
                VarInt::new(uncompressed_frame.len() as i32)
                    .encode(&mut inner, &NetEncodeOpts::None)?;
                inner.extend_from_slice(&compressed);
            } else {
                // below threshold: uncompressed; write 0 then ID+BODY
                VarInt::new(0).encode(&mut inner, &NetEncodeOpts::None)?;
                inner.extend_from_slice(&uncompressed_frame);
            }

            // Outer = VarInt(total inner len) + inner
            let mut final_data = Vec::with_capacity(inner.len() + 5);
            VarInt::new(inner.len() as i32).encode(&mut final_data, &NetEncodeOpts::None)?;
            final_data.extend_from_slice(&inner);
            final_data
        } else {
            // No compression path: just do whatever caller asked (likely WithLength).
            let mut buffer = Vec::new();
            packet.encode(&mut buffer, net_encode_opts)?;
            buffer
        };

        self.sender.send(raw_bytes).map_err(std::io::Error::other)?;
        Ok(())
    }


}

pub struct NewConnection {
    pub stream: StreamWriter,
    pub player_identity: PlayerIdentity,
    pub entity_return: oneshot::Sender<Entity>,
}

pub async fn handle_connection(
    state: Arc<ServerState>,
    tcp_stream: TcpStream,
    packet_sender: Arc<PacketSender>,
    new_join_sender: Arc<Sender<NewConnection>>,
) -> Result<(), NetError> {
    let (mut tcp_reader, tcp_writer) = tcp_stream.into_split();


    let running = Arc::new(AtomicBool::new(true));

    let stream = StreamWriter::new(tcp_writer, running.clone()).await;

    let handshake_result = timeout(
        MAX_HANDSHAKE_TIMEOUT,
        handle_handshake(&mut tcp_reader, &stream, state.clone()),
    )
        .await;

    let mut player_identity = PlayerIdentity::default();

    match handshake_result {
        Ok(res) => match res {
            Ok((false, returned_player_identity)) => {
                trace!("Handshake successful");
                match returned_player_identity {
                    Some(returned_player_identity) => {
                        trace!("Player identity: {:?}", returned_player_identity);
                        player_identity = returned_player_identity;
                    }
                    None => {
                        error!("Player identity not found");
                    }
                }
            }
            Ok((true, _)) => {
                trace!("Handshake successful, killing connection");
                return Ok(());
            }
            Err(err) => {
                match &err {
                    NetError::MismatchedProtocolVersion(client_version, server_version) => {
                        warn!(
                            "Client connected with incompatible protocol version {} (server supports {})",
                            client_version, server_version
                        );
                    }
                    NetError::InvalidState(state) => {
                        warn!("Client sent invalid handshake state: {}", state);
                    }
                    _ => {
                        error!("Handshake error: {}", err);
                    }
                }
                return Err(err);
            }
        },
        Err(err) => {
            error!("Handshake timed out: {:?}", err);
            return Err(HandshakeTimeout);
        }
    }

    // The player has successfully connected, so we can start the connection properly

    let compressed = false;

    // Send the streamwriter to the main thread
    let (entity_return, entity_recv) = oneshot::channel();

    new_join_sender
        .send(NewConnection {
            stream,
            player_identity,
            entity_return,
        })
        .map_err(|_| NetError::Misc("Failed to send new connection".to_string()))?;

    // Wait for the entity ID to be sent back
    let entity = match entity_recv.await {
        Ok(entity) => entity,
        Err(err) => {
            error!("Failed to receive entity ID: {:?}", err);
            return Err(NetError::Misc("Failed to receive entity ID".to_string()));
        }
    };

    'recv: loop {
        if !running.load(Ordering::Relaxed) {
            trace!("Conn for entity {:?} is marked for disconnection", entity);
            break 'recv;
        }

        if state.shut_down.load(Ordering::Relaxed) {
            break 'recv;
        }

        let mut packet_skele = match PacketSkeleton::new(&mut tcp_reader, compressed, Play).await {
            Ok(packet_skele) => packet_skele,
            Err(err) => {
                if let NetError::ConnectionDropped = err {
                    trace!("Connection dropped for entity {:?}", entity);
                    running.store(false, Ordering::Relaxed);
                    break 'recv;
                }
                error!("Failed to read packet skeleton: {:?} for {:?}", err, entity);
                running.store(false, Ordering::Relaxed);
                break 'recv;
            }
        };

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
                    "Packet {:02X} handled for entity {:?}",
                    packet_skele.id,
                    entity
                );
            }
            Err(err) => match &err {
                NetError::Packet(InvalidPacket(id)) => {
                    trace!("Packet 0x{:02X} received, no handler implemented yet", id);
                }
                _ => {
                    warn!("Failed to handle packet: {:?}", err);
                    running.store(false, Ordering::Relaxed);
                    break 'recv;
                }
            },
        }
    }

    Ok(())
}

impl StreamWriter {
    /// Kills the connection and sends a disconnect packet to the client
    ///
    /// !!! This won't delete the entity, you should do that with the connection killer system
    pub fn kill(&self, reason: Option<String>) -> Result<(), NetError> {
        self.running.store(false, Ordering::Relaxed);
        if let Err(err) = self.send_packet(crate::packets::outgoing::disconnect::DisconnectPacket {
            reason: ferrumc_text::TextComponent::from(reason.unwrap_or("Disconnected".to_string())),
        }) {
            if matches!(err, NetError::ConnectionDropped) {
                trace!("Connection already dropped, not sending disconnect packet");
            } else {
                error!("Failed to send disconnect packet: {:?}", err);
                return Err(err);
            }
        }
        Ok(())
    }
}
