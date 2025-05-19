use crate::conn_init::handle_handshake;
use crate::errors::NetError;
use crate::errors::NetError::HandshakeTimeout;
use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::{handle_packet, PacketSender};
use bevy_ecs::prelude::{Component, Entity};
use crossbeam_channel::Sender;
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
use tracing::{debug, debug_span, error, trace, Instrument};
use typename::TypeName;

/// The maximum time to wait for a handshake to complete
const MAX_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(TypeName, Component)]
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
    pub async fn new(mut writer: OwnedWriteHalf, running: Arc<AtomicBool>) -> Self {
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

        Self { sender, running }
    }

    // Sends the packet to the client with the default options. You probably want to use this instead
    // of send_packet_with_opts()
    pub fn send_packet(
        &self,
        packet: impl NetEncode + Send,
    ) -> Result<(), NetError> {
        self.send_packet_with_opts(packet, &NetEncodeOpts::WithLength)
    }

    pub fn send_packet_with_opts(
        &self,
        packet: impl NetEncode + Send,
        net_encode_opts: &NetEncodeOpts,
    ) -> Result<(), NetError> {
        let bytes = {
            let mut buffer = Vec::new();
            packet.encode(&mut buffer, net_encode_opts)?;
            buffer
        };
        self.sender.send(bytes).map_err(std::io::Error::other)?;
        Ok(())
    }
}

pub struct NewConnection {
    pub stream: StreamWriter,
    pub entity_return: oneshot::Sender<Entity>,
}

pub async fn handle_connection(
    state: Arc<ServerState>,
    tcp_stream: TcpStream,
    packet_sender: Arc<PacketSender>,
    new_join_sender: Arc<Sender<NewConnection>>,
) -> Result<(), NetError> {
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

    let compressed = false;
    let running = Arc::new(AtomicBool::new(true));

    let stream = StreamWriter::new(tcp_writer, running.clone()).await;

    // Send the streamwriter to the main thread
    let (entity_return, entity_recv) = oneshot::channel();

    new_join_sender
        .send(NewConnection {
            stream,
            entity_return,
        })
        .map_err(|_| NetError::Misc("Failed to send new connection".to_string()))?;

    // Wait for the entity ID to be sent back, use timeout so we can't hang if nothing is sent
    // TODO: Make the delay scale based on the server tick rate since the entity ID is sent back
    // in a system which could run at less than 1 tps
    let entity = match timeout(Duration::from_secs(1), entity_recv).await {
        Ok(res) => match res {
            Ok(entity) => {
                debug!("Entity ID received: {:?}", entity);
                entity
            }
            Err(err) => {
                error!("Failed to receive entity ID: {:?}", err);
                return Err(NetError::Misc("Failed to receive entity ID".to_string()));
            }
        },
        Err(err) => {
            error!("Entity return timed out: {:?}", err);
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

        let mut packet_skele = match PacketSkeleton::new(&mut tcp_reader, compressed).await {
            Ok(packet_skele) => packet_skele,
            Err(err) => {
                error!("Failed to read packet skeleton: {:?}", err);
                debug!("Connection dropped for entity {:?}", entity);
                running.store(false, Ordering::Relaxed);
                break 'recv;
            }
        };

        debug!("Got a packet");

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
            Err(err) => {
                debug!("Failed to handle packet: {:?}", err);
                running.store(false, Ordering::Relaxed);
                break 'recv;
            }
        }
    }

    Ok(())
}

impl StreamWriter {
    /// Kills the connection and sends a disconnect packet to the client
    ///
    /// !!! This won't delete the entity, you should do that with the connection killer system
    pub fn kill(&self, reason: Option<String>) -> Result<(), NetError> {
        self.send_packet(
            crate::packets::outgoing::disconnect::DisconnectPacket {
                reason: ferrumc_text::TextComponent::from(reason.unwrap_or_else(|| "Disconnected".to_string()))
            }
        )?;
        self.running.store(false, Ordering::Relaxed);
        Ok(())
    }
}

// fn disconnect(state: Arc<ServerState>, entity: usize) {
//     debug!("Connection closed for entity: {:?}", entity);
//
//     // Broadcast the leave server event
//
//     _ = PlayerDisconnectEvent::trigger(PlayerDisconnectEvent { entity_id: entity }, state.clone());
//
//     // Remove all components from the entity
//
//     terminate_connection(state.clone(), entity, "Failed to handle packet".to_string())
//         .expect("Failed to terminate connection");
//
//     // Wait until anything that might be using the entity is done
//     if let Err(e) = remove_all_components_blocking(state.clone(), entity) {
//         warn!("Failed to remove all components from entity: {:?}", e);
//     }
//
//     trace!("Dropped all components from entity: {:?}", entity);
// }
