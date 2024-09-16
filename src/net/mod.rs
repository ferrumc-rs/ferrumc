use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::io::{Cursor, Read, Write};
use std::sync::atomic::AtomicU32;
use std::sync::{atomic, Arc};
use std::time::Duration;

use dashmap::DashMap;
use ferrumc_codec::enc::{EncodeOption, NetEncode};
use ferrumc_codec::network_types::varint::VarInt;
use flate2::read::ZlibDecoder;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard};
use tracing::{debug, error, trace};

use ferrumc_macros::Component;

use crate::net::packets::{handle_packet, ConnectionId};
use crate::state::GlobalState;

use super::utils::config::get_global_config;
use super::utils::prelude::*;
pub mod utils;
// To allow implementing the `Component` trait for `Connection`. Since we can't implement a trait for a type defined in another crate.
#[derive(Component)]
pub struct ConnectionWrapper(pub Arc<RwLock<Connection>>);

impl Debug for ConnectionWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionWrapper")
    }
}

/// Implementing `Send` for `ConnectionWrapper` to allow sending it between threads.
/// This is safe because `ConnectionWrapper` is just a wrapper around `Arc<RwLock<Connection>>`, which is `Send`.
unsafe impl Send for ConnectionWrapper {}
unsafe impl Sync for ConnectionWrapper {}

pub mod packets;
pub mod systems;
mod test_ecs;
pub mod the_dimension_codec;

#[derive(PartialEq, Debug, Clone)]
pub enum State {
    Unknown,
    Handshake,
    Status,
    Login,
    Play,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl State {
    pub fn as_str(&self) -> &str {
        match self {
            State::Unknown => "unknown",
            State::Handshake => "handshake",
            State::Status => "status",
            State::Login => "login",
            State::Play => "play",
        }
    }
}

/// A list of connections, with a counter for the number of connections.
///
/// In desperate need of reworking.
pub struct ConnectionList {
    // The connections, keyed with random values. The value also contains the connection id for ease of access.
    pub connections: DashMap<ConnectionId, Arc<RwLock<Connection>>>,
    // The number of connections.
    pub connection_count: AtomicU32,
}

impl ConnectionList {
    pub fn get_connection(&self, conn_id: impl TryInto<usize>) -> Result<Arc<RwLock<Connection>>> {
        let conn_id = conn_id.try_into().map_err(|_| Error::ConversionError)?;
        let conn = self
            .connections
            .get(&conn_id)
            .ok_or(Error::ConnectionNotFound(conn_id))?;

        Ok(conn.clone())
    }
}

/// A connection to a client.
///
/// - `id`: The numerical ID for the connection. Is also the key for it's [ConnectionList] entry.
/// - `socket`: The TCP socket for the connection ([tokio::net::TcpStream]).
/// - `player_uuid`: The UUID of the player, if the connection is authenticated ([uuid::Uuid]).
/// - `state`: The current state of the connection ([State]).
/// - `metadata`: Metadata for the connection ([ConnectionMetadata]).
/// - `drop`: Whether to drop and clean up the connection after this network tick.
pub struct Connection {
    pub id: usize,
    // pub socket: tokio::net::TcpStream,
    pub stream: NetStream,
    pub player_uuid: Option<uuid::Uuid>,
    pub state: State,
    pub metadata: ConnectionMetadata,
    pub drop: bool,
}

pub struct NetStream {
    pub in_stream: Mutex<tokio::net::tcp::OwnedReadHalf>,
    pub out_stream: Mutex<tokio::net::tcp::OwnedWriteHalf>,
}

/// Metadata for a connection.
///
/// - `protocol_version`: The protocol version of the connection.
/// - `entity`: The entity ID of the player.
/// - `compressed`: Whether the connection is compressed. Default is false, until the server sends a SetCompression packet.
#[derive(Debug, Default)]
pub struct ConnectionMetadata {
    pub protocol_version: i32,
    pub entity: usize,
    pub compressed: bool, // Default false, until server sends SetCompression
}

pub fn setup_tracer() {
    console_subscriber::init();
}

/// Handles a connection. This is the main entry point for a connection.
///
/// - `socket`: The TCP socket for the connection ([tokio::net::TcpStream]).
///
/// Creates a new [Connection] and adds it to the [ConnectionList]. Passes the connection to [manage_conn].
pub async fn init_connection(socket: tokio::net::TcpStream, state: GlobalState) -> Result<()> {
    let entity_id = state.world.create_entity().await.build();

    let (in_stream, out_stream) = socket.into_split();

    let conn = Connection {
        id: entity_id,
        stream: NetStream {
            in_stream: Mutex::new(in_stream),
            out_stream: Mutex::new(out_stream),
        },
        player_uuid: None,
        state: State::Handshake,
        metadata: ConnectionMetadata::default(),
        drop: false,
    };

    let conn = Arc::new(RwLock::new(conn));

    state
        .world
        .get_component_storage()
        .insert(entity_id, ConnectionWrapper(conn.clone()));

    // Doesn't matter if we clone, since actual value is not cloned
    state
        .connections
        .connections
        .insert(entity_id, conn.clone());
    state
        .connections
        .connection_count
        .fetch_add(1, atomic::Ordering::Relaxed);

    let current_amount = state
        .connections
        .connection_count
        .load(atomic::Ordering::Relaxed);

    debug!(
        "Connection established with id: {}. Current connection count: {}",
        entity_id, current_amount
    );

    let res = manage_conn(conn.clone(), state.clone()).await;

    if let Err(e) = res {
        error!(
            "Error occurred in {:?}: {:?}, dropping connection",
            entity_id, e
        );
        drop_conn(entity_id, state).await?;
    }

    Ok(())
}

/// Manages a connection. This is the main loop for a connection.
///
/// - `conn`: The connection to manage ([Arc<RwLock<Connection>>]).
///
/// Reads packets from the connection and passes them to [handle_packet]. The handle_packet function
/// is generated at compile time by [ferrumc_macros::bake_packet_registry].
pub async fn manage_conn(conn: Arc<RwLock<Connection>>, state: GlobalState) -> Result<()> {
    {
        let local_addr = conn
            .read()
            .await
            .stream
            .in_stream
            .lock()
            .await
            .peer_addr()?;
        debug!("Starting receiver for the addr: {:?}", local_addr);
    }

    let network_compression_threshold = get_global_config().network_compression_threshold;

    loop {
        // Get the length of the packet
        let conn_read = conn.read().await;

        trace!("Reading length buffer");

        let (packet_length, buffer) =
            get_packet_length_and_buffer(&conn_read, network_compression_threshold).await?;
        let (conn_id, conn_state, is_compressed) = (
            conn_read.id,
            conn_read.state.clone(),
            conn_read.metadata.compressed,
        );
        drop(conn_read); // Release the read lock

        trace!("Packet Length: {}", packet_length.get_val());

        let mut cursor = Cursor::new(buffer);

        if is_compressed {
            // If the packet is compressed, handle decompression
            let data_length = VarInt::read(&mut cursor).await?.get_val() as usize;

            if data_length != 0 {
                let mut z = ZlibDecoder::new(cursor);
                let mut decompressed_data = Vec::new();
                z.read_to_end(&mut decompressed_data)?;

                cursor = Cursor::new(decompressed_data); // Update cursor with decompressed data
            } else {
                trace!("Packet is not compressed (size below compression threshold)");
            }
        }

        // Get the packet id
        let packet_id = VarInt::read(&mut cursor).await?;
        trace!("Packet ID: {}", packet_id);

        let packet_id = packet_id.get_val() as u8;

        let state_clone = state.clone();
        tokio::spawn(async move {
            handle_packet(packet_id, conn_id, &conn_state, &mut cursor, state_clone).await
        });

        // Drop connection if flagged
        drop_conn_if_flagged(conn.clone(), state.clone()).await?;

        // Sleep based on network tick rate
        let tick_rate = get_global_config().network_tick_rate;
        let sleep_duration = Duration::from_millis(if tick_rate > 0 {
            1000 / tick_rate as u64
        } else {
            0
        });
        tokio::time::sleep(sleep_duration).await;
    }
    #[allow(unreachable_code)]
    Ok(())
}
async fn get_packet_length_and_buffer(
    conn: &RwLockReadGuard<'_, Connection>,
    network_compression_threshold: i32,
) -> Result<(VarInt, Vec<u8>)> {
    let compressed = conn.metadata.compressed;
    let mut conn = conn.get_in_stream().await;

    // Read length of packet
    let packet_length = VarInt::read(&mut *conn).await?;

    // Read packet data into buffer
    let mut buffer = vec![0u8; packet_length.get_val() as usize];
    conn.read_exact(&mut buffer).await?;

    // Handle cases when compression is enabled
    if compressed {
        // Decompress the packet
        let mut cursor = Cursor::new(&buffer);
        let data_length = VarInt::read(&mut cursor).await?;

        // If the data length is greater than or equal to the threshold, the data is compressed
        if data_length.get_val() >= network_compression_threshold {
            // Compressed packet. Need to decompress it.
            let compressed_data = &buffer[cursor.position() as usize..];
            let mut z = ZlibDecoder::new(Cursor::new(compressed_data));
            let mut decompressed_data = Vec::new();
            z.read_to_end(&mut decompressed_data)?;

            // return the decompressed data
            return Ok((data_length, decompressed_data));
        }
    }

    // Compression off or data length less than threshold. Return the buffer as is.
    Ok((packet_length, buffer))
}
async fn drop_conn_if_flagged(conn: Arc<RwLock<Connection>>, state: GlobalState) -> Result<()> {
    let read = conn.read().await;
    let do_drop = read.drop;
    let id = read.id;
    drop(read);

    if do_drop {
        drop_conn(id, state).await?;
    }

    Ok(())
}
pub async fn drop_conn(connection_id: usize, state: GlobalState) -> Result<()> {
    debug!("Dropping connection with id: {}", connection_id);
    let connection = state.connections.connections.remove(&connection_id);
    let Some((_, conn_arc)) = connection else {
        return Err(Error::ConnectionNotFound(connection_id));
    };
    state
        .connections
        .connection_count
        .fetch_sub(1, atomic::Ordering::Relaxed);

    {
        let read_lock = conn_arc.read().await;
        let entity_id = read_lock.id;
        state.world.delete_entity(entity_id).await?;
    }

    // drop the connection in the end, just in case it errors out
    let conn = conn_arc.read().await;
    let mut conn = conn.get_out_stream().await;
    conn.shutdown().await?;
    Ok(())
}

impl Connection {
    pub async fn send_packet(&self, packet: impl NetEncode) -> Result<()> {
        trace!("Sending packet");
        let mut out_stream = self.get_out_stream().await;

        // Is compression enabled?
        let compressed = self.metadata.compressed;
        if compressed {
            trace!("Compression is enabled");

            // Get the packet without length information
            let mut packet_data = Vec::new();
            packet
                .net_encode(&mut packet_data, &EncodeOption::AlwaysOmitSize)
                .await?;

            // Get the length of the data
            let data_length = VarInt::from(packet_data.len() as i32);

            let network_compression_threshold = get_global_config().network_compression_threshold;
            if data_length.get_val() >= network_compression_threshold {
                trace!("Compressing packet");
                // Compress the packet
                let mut compressed_data = Vec::new();
                let mut encoder = flate2::write::ZlibEncoder::new(
                    &mut compressed_data,
                    flate2::Compression::default(),
                );
                encoder.write_all(&packet_data)?;
                encoder.finish()?;

                // Compressed packet structure
                let compressed_length = compressed_data.len();
                let packet_length =
                    VarInt::from((data_length.get_len() + compressed_length) as i32);

                // Send the packet
                packet_length
                    .net_encode(&mut *out_stream, &EncodeOption::AlwaysOmitSize)
                    .await?;

                data_length
                    .net_encode(&mut *out_stream, &EncodeOption::AlwaysOmitSize)
                    .await?;

                out_stream.write_all(&compressed_data).await?; // Sending raw compressed data
            } else {
                trace!("Data length is less than threshold");
                // No compression applied, use a 0 length
                let zero_length = VarInt::from(0); // Indicate no compression

                let packet_length =
                    VarInt::from((zero_length.get_len() + packet_data.len()) as i32);

                // Send the packet length and uncompressed data
                packet_length
                    .net_encode(&mut *out_stream, &EncodeOption::AlwaysOmitSize)
                    .await?;

                zero_length
                    .net_encode(&mut *out_stream, &EncodeOption::AlwaysOmitSize)
                    .await?;

                out_stream.write_all(&packet_data).await?;
            }
        } else {
            trace!("Compression is disabled");
            // Compression is disabled
            // Send the packet with no compression format (Default EncodeOption)
            packet
                .net_encode(&mut *out_stream, &EncodeOption::Default)
                .await?;
        }
        Ok(())
    }

    /// Just exists so it doesn't seem weird when sending a packet_queue, since multiple packetS are sent.
    pub async fn send_packets(&self, packets: impl NetEncode) -> Result<()> {
        self.send_packet(packets).await
    }

    pub async fn get_in_stream(&self) -> MutexGuard<'_, tokio::net::tcp::OwnedReadHalf> {
        self.stream.in_stream.lock().await
    }

    pub async fn get_out_stream(&self) -> MutexGuard<'_, tokio::net::tcp::OwnedWriteHalf> {
        self.stream.out_stream.lock().await
    }

    pub async fn drop_connection(&self, state: GlobalState) -> Result<()> {
        drop_conn(self.id, state).await
    }
}
