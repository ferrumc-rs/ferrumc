#![feature(box_into_inner)]

use std::cmp::PartialEq;
use std::fmt::Display;
use std::io::Cursor;
use std::ops::DerefMut;
use std::sync::{Arc, atomic, OnceLock};
use std::sync::atomic::AtomicU32;
use std::time::Duration;

use dashmap::DashMap;
use ferrumc_ecs::components::{Component};
use ferrumc_ecs::world::{Entity, World};
use ferrumc_utils::config::get_global_config;
use ferrumc_utils::encoding::varint::read_varint;
use ferrumc_utils::prelude::*;
use ferrumc_utils::type_impls::Encode;
use rand::random;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tracing::{debug, error, trace};

use crate::packets::{handle_packet};

// To allow implementing the `Component` trait for `Connection`. Since we can't implement a trait for a type defined in another crate.
pub struct ConnectionWrapper(pub Arc<RwLock<Connection>>);
/// Implementing `Send` for `ConnectionWrapper` to allow sending it between threads.
/// This is safe because `ConnectionWrapper` is just a wrapper around `Arc<RwLock<Connection>>`, which is `Send`.
unsafe impl Send for ConnectionWrapper {}
impl Component for ConnectionWrapper {}

pub mod packets;
pub mod the_dimension_codec;
mod test_ecs;

#[allow(non_snake_case)]
pub fn GET_WORLD() -> &'static RwLock<World> {
    static WORLD: OnceLock<RwLock<World>> = OnceLock::new();
    WORLD.get_or_init(|| RwLock::new(World::new()))
}

#[allow(non_snake_case)]
pub fn CONNECTIONS() -> &'static ConnectionList {
    static CONNECTIONS: OnceLock<ConnectionList> = OnceLock::new();
    CONNECTIONS.get_or_init(|| ConnectionList {
        connections: DashMap::new(),
        connection_count: AtomicU32::new(0),
    })
}

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
    pub entity: Entity,
}

pub fn setup_tracer() {
    console_subscriber::init();
}

/// Handles a connection. This is the main entry point for a connection.
///
/// - `socket`: The TCP socket for the connection ([tokio::net::TcpStream]).
///
/// Creates a new [Connection] and adds it to the [ConnectionList]. Passes the connection to [manage_conn].
pub async fn init_connection(socket: tokio::net::TcpStream) -> Result<()> {
    let mut id = random();
    while CONNECTIONS().connections.contains_key(&id) {
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

    let mut world = GET_WORLD().write().await;
    let entity = world.create_entity()
        .with(ConnectionWrapper(conn.clone()))
        .build();
    drop(world);

    {
        let mut conn = conn.write().await;
        conn.metadata.entity = entity;
        drop(conn);
    }

    // Doesn't matter if we clone, since actual value is not cloned
    CONNECTIONS().connections.insert(id, conn.clone());
    CONNECTIONS()
        .connection_count
        .fetch_add(1, atomic::Ordering::Relaxed);

    let current_amount = CONNECTIONS()
        .connection_count
        .load(atomic::Ordering::Relaxed);

    debug!(
        "Connection established with id: {}. Current connection count: {}",
        id, current_amount
    );


    let res = manage_conn(conn.clone()).await;

    if let Err(e) = res {
        error!("Error occurred in {:?}: {:?}, dropping connection", id, e);
        let mut world = GET_WORLD().write().await;
        let entity_id = &conn.read().await.metadata.entity;
        world.delete_entity(entity_id)?;
        drop(world);
        drop_conn(id).await?;
    }

    Ok(())
}

/// Manages a connection. This is the main loop for a connection.
///
/// - `conn`: The connection to manage ([Arc<RwLock<Connection>>]).
///
/// Reads packets from the connection and passes them to [handle_packet]. The handle_packet function
/// is generated at compile time by [ferrumc_macros::bake_packet_registry].
pub async fn manage_conn(conn: Arc<RwLock<Connection>>) -> Result<()> {
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
        handle_packet(packet_id, actual_connection, &mut cursor).await?;

        // drop the handle to the write lock. to allow other tasks to write/read
        drop(conn_write);

        let read = conn.read().await;

        // drop if the connection is marked for drop
        let do_drop = read.drop;
        let id = read.id;

        drop(read);

        if do_drop {
            drop_conn(id).await?;
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

async fn drop_conn(connection_id: u32) -> Result<()> {
    debug!("Dropping connection with id: {}", connection_id);
    let connection = CONNECTIONS().connections.remove(&connection_id);
    let Some((_, conn_arc)) = connection else {
        return Err(Error::ConnectionNotFound(connection_id));
    };
    CONNECTIONS()
        .connection_count
        .fetch_sub(1, atomic::Ordering::Relaxed);
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
}