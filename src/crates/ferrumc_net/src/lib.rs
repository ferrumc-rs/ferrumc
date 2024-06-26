#![feature(box_into_inner)]

use std::cmp::PartialEq;
use std::fmt::Display;
use std::io::Cursor;
use std::sync::{Arc, atomic, OnceLock};
use std::sync::atomic::AtomicU32;

use dashmap::DashMap;
use ferrumc_utils::encoding::varint::read_varint;
use ferrumc_utils::prelude::*;
use lariv::Lariv;
use lazy_static::lazy_static;
use log::{debug, trace};
use rand::random;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;

use crate::packets::{handle_packet};

mod packets;

#[allow(non_snake_case)]
pub fn CONNECTIONS() -> &'static ConnectionList {
    static CONNECTIONS: OnceLock<ConnectionList> = OnceLock::new();
    CONNECTIONS.get_or_init(|| ConnectionList {
        connections: DashMap::new(),
        connection_count: AtomicU32::new(0),
        purge_queue: Lariv::new(1024),
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

pub struct ConnectionList {
    // The connections, keyed with random values. The value also contains the connection id for ease of access.
    // pub connections: DashMap<u32, Connection>,
    pub connections: DashMap<u32, Arc<RwLock<Connection>>>,
    // The number of connections.
    pub connection_count: AtomicU32,
    // The queue of connections to be purged. This is used to store the connections to be dropped at the end of every tick.
    pub purge_queue: Lariv<u32>,
}

#[derive()]
pub struct Connection {
    // The connection id.
    pub id: u32,
    // The socket.
    pub socket: tokio::net::TcpStream,
    // The player uuid, if the connection is authenticated.
    pub player_uuid: Option<uuid::Uuid>,
    // State
    pub state: State,
    // Metadata
    pub metadata: ConnectionMetadata,
}

#[derive(Debug, Default)]
pub struct ConnectionMetadata {
    pub protocol_version: i32,
}

pub fn setup_tracer() {
    console_subscriber::init();
}

pub async fn handle_connection(socket: tokio::net::TcpStream) -> Result<()> {
    let mut id = random();
    // check if we have a collision (1 in 4.2 billion chance) and if so, generate a new id
    while CONNECTIONS().connections.contains_key(&id) {
        id = random();
    }
    let conn = Arc::new(RwLock::new(
        Connection {
        id,
        socket,
        player_uuid: None,
        state: State::Handshake,
        metadata: ConnectionMetadata::default(),
    }));

    CONNECTIONS().connections.insert(id, conn);
    CONNECTIONS()
        .connection_count
        .fetch_add(1, atomic::Ordering::Relaxed);

    debug!("Connection established with id: {}. Current connection count: {}", id, CONNECTIONS().connection_count.load(atomic::Ordering::Relaxed));

    let mut conn_ref = CONNECTIONS().connections.view(&id, |_k, v| {v.clone()}).unwrap();
    manage_conn(&mut conn_ref).await?;

    Ok(())
}

    pub async fn manage_conn(conn: &mut Arc<RwLock<Connection>>) -> Result<()> {
        debug!("Starting receiver for the same addr: {:?}", conn.read().await.socket.peer_addr()?);

        loop {
            debug!("1");
            let mut length_buffer = vec![0u8; 1];
            {
                let mut conn_write = conn.write().await;
                conn_write.socket.read_exact(&mut length_buffer).await?;
            }
            debug!("2");
            let length = length_buffer[0] as usize;

            let mut buffer = vec![0u8; length];

            {
                let mut conn_write = conn.write().await;
                conn_write.socket.read_exact(&mut buffer).await?;
            }
            debug!("3");
            let buffer = vec![length_buffer, buffer].concat();

            let mut cursor = Cursor::new(buffer);

            let packet_length = read_varint(&mut cursor).await?;
            let packet_id = read_varint(&mut cursor).await?;
            debug!("4");

            trace!("Packet Length: {}", packet_length);
            trace!("Packet ID: {}", packet_id);
            debug!("5");

            handle_packet(packet_id.get_val() as u8, conn, &mut cursor).await?;

            debug!("6");

            // TODO: Check if we need to drop the connection
        }
        #[allow(unreachable_code)]
        Ok(())
    }

    #[allow(dead_code)]
    async fn drop_conn(connection: &mut Connection) -> Result<()> {
        trace!("Dropping connection with id: {}", connection.id);
        let id = connection.id;
        CONNECTIONS().connections.remove(&id);
        CONNECTIONS().connection_count.fetch_sub(1, atomic::Ordering::Relaxed);
        Ok(())
    }



