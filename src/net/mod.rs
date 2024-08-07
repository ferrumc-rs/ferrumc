use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::io::Cursor;
use std::ops::DerefMut;
use std::sync::{Arc, atomic};
use std::sync::atomic::AtomicU32;
use std::time::Duration;

use dashmap::DashMap;
use rand::random;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tracing::{debug, error, trace};

use ferrumc_macros::Component;

use crate::net::packets::handle_packet;
use crate::state::GlobalState;

use super::utils::config::get_global_config;
use super::utils::encoding::varint::read_varint;
use super::utils::prelude::*;
use super::utils::type_impls::Encode;

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

#[derive(PartialEq, Debug)]
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
    // pub connections: DashMap<u32, Connection>,
    pub connections: DashMap<u32, Arc<RwLock<Connection>>>,
    // The number of connections.
    pub connection_count: AtomicU32,
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
    pub id: u32,
    pub socket: tokio::net::TcpStream,
    pub player_uuid: Option<uuid::Uuid>,
    pub state: State,
    pub metadata: ConnectionMetadata,
    pub drop: bool,
}

#[derive(Debug, Default)]
pub struct ConnectionMetadata {
    pub protocol_version: i32,
    pub entity: usize,
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
    let mut id = random();
    while state.connections.connections.contains_key(&id) {
        id = random();
    }

    let conn = Connection {
        id,
        socket,
        player_uuid: None,
        state: State::Handshake,
        metadata: ConnectionMetadata::default(),
        drop: false,
    };
    let conn = Arc::new(RwLock::new(conn));

    let entity = state
        .world
        .create_entity()
        .await
        .with(ConnectionWrapper(conn.clone()))
        .build();

    {
        let mut conn = conn.write().await;
        conn.metadata.entity = entity;
        drop(conn);
    }

    // Doesn't matter if we clone, since actual value is not cloned
    state.connections.connections.insert(id, conn.clone());
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
        id, current_amount
    );

    let res = manage_conn(conn.clone(), state.clone()).await;

    if let Err(e) = res {
        error!("Error occurred in {:?}: {:?}, dropping connection", id, e);
        let entity_id = conn.read().await.metadata.entity;
        state.world.delete_entity(entity_id).await?;
        drop_conn(id, state).await?;
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
    debug!(
        "Starting receiver for the same addr: {:?}",
        conn.read().await.socket.peer_addr()?
    );

    loop {
        // Get the length of the packet
        let mut length_buffer = vec![0u8; 1];

        trace!("Reading length buffer");

        let mut conn_write = conn.write().await;
        conn_write.socket.read_exact(&mut length_buffer).await?;

        trace!("Length buffer: {:?}", length_buffer);

        let length = length_buffer[0] as usize;

        // Get the rest of the packet
        let mut buffer = vec![0u8; length];

        conn_write.socket.read_exact(&mut buffer).await?;

        let buffer = vec![length_buffer, buffer].concat();

        let mut cursor = Cursor::new(buffer);

        // Get the packet length and id
        let packet_length = read_varint(&mut cursor).await?;
        let packet_id = read_varint(&mut cursor).await?;

        trace!("Packet Length: {}", packet_length);
        trace!("Packet ID: {}", packet_id);

        let packet_id = packet_id.get_val() as u8;
        let actual_connection = conn_write.deref_mut();
        // Handle the packet
        handle_packet(packet_id, actual_connection, &mut cursor, state.clone()).await?;

        // drop the handle to the write lock. to allow other tasks to write/read
        drop(conn_write);

        let read = conn.read().await;

        // drop if the connection is marked for drop
        let do_drop = read.drop;
        let id = read.id;

        drop(read);

        if do_drop {
            drop_conn(id, state).await?;
            break;
        }

        let tick_rate = get_global_config().network_tick_rate;
        let sleep_duration_millis: u64 = if tick_rate > 0 {
            1000 / tick_rate as u64
        } else {
            0
        };

        tokio::time::sleep(Duration::from_millis(sleep_duration_millis)).await;
    }
    #[allow(unreachable_code)]
    Ok(())
}

pub async fn drop_conn(connection_id: u32, state: GlobalState) -> Result<()> {
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
        let entity_id = read_lock.metadata.entity;
        state.world.delete_entity(entity_id).await?;
    }

    // drop the connection in the end, just in case it errors out
    let mut conn = conn_arc.write().await;
    conn.socket.shutdown().await?;
    Ok(())
}

impl Connection {
    pub async fn send_packet(&mut self, packet: impl Encode) -> Result<()> {
        let mut cursor = Cursor::new(Vec::new());
        packet.encode(&mut cursor).await?;
        let packet = cursor.into_inner();
        self.socket.write_all(&*packet).await?;
        Ok(())
    }

    pub async fn drop_connection(&self, state: GlobalState) -> Result<()> {
        Ok(drop_conn(self.id, state).await?)
    }
}
