#![feature(box_into_inner)]

use std::cmp::PartialEq;
use std::io::Cursor;
use std::sync::{Arc, atomic, OnceLock};
use std::sync::atomic::AtomicU32;

use dashmap::DashMap;
use ferrumc_utils::encoding::varint::{read_varint};
use ferrumc_utils::prelude::*;
use lariv::Lariv;
use log::{debug, error, info, trace};
use rand::random;
use tokio::io::{AsyncWriteExt};
use tokio::io::AsyncReadExt;
use tokio::sync::{RwLock};
use crate::packets::handshake::HandshakePacket;

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


#[derive(PartialEq)]
pub enum State {
    Unknown,
    Handshake,
    Status,
    Login,
    Play,
}

pub struct ConnectionList {
    // The connections, keyed with random values. The value also contains the connection id for ease of access.
    // pub connections: DashMap<u32, Connection>,
    pub connections: DashMap<u32, Connection>,
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
    // A queue of bytes to be sent to the client. Set up this way to allow for easy batching of packets & we can run the sender/receiver in parallel.
    pub send_queue: Vec<Vec<u8>>,
    // State
    pub state: State,
    // Notifier for the sender to send the packets.
    pub sender_notifier: tokio::sync::Notify,
}

pub async fn handle_connection(socket: tokio::net::TcpStream) -> Result<()> {
    let mut id = random();
    // check if we have a collision (1 in 4.2 billion chance) and if so, generate a new id
    while CONNECTIONS().connections.contains_key(&id) {
        id = random();
    }
    let conn = Connection {
        id,
        socket,
        player_uuid: None,
        send_queue: Vec::new(),
        state: State::Unknown,
        sender_notifier: tokio::sync::Notify::new(),
    };

    CONNECTIONS().connections.insert(id, conn);
    CONNECTIONS()
        .connection_count
        .fetch_add(1, atomic::Ordering::Relaxed);

    debug!("Connection established with id: {}, total count: {}", id, CONNECTIONS().connection_count.load(atomic::Ordering::Relaxed));

    let mut conn_ref = CONNECTIONS().connections.get_mut(&id).ok_or(Error::ConnectionNotFound(id))?;
    conn_ref.start_connection().await?;

    Ok(())
}

impl Connection {
    pub async fn start_connection(&mut self) -> Result<()> {
        self.state = State::Handshake;
        trace!("Starting connection with id: {}", self.id);

        let arc_id = Arc::new(RwLock::new(self.id));

        tokio::spawn(async move {
            let conn = CONNECTIONS().connections.get_mut(&*arc_id.read().await);

            let Some(mut conn) = conn else {
                error!("Connection not found for id: {}", *arc_id.read().await);
                return;
            };

            let res = conn.sender().await;

            if let Err(e) = res {
                trace!("Error in sender: {:?}", e);
            }
        });

        self.receiver().await?;


        Ok(())
    }

    pub async fn sender(&mut self) -> Result<()> {
        trace!("Starting sender for connection with addy: {:?}", self.socket.peer_addr()?);

        loop {
            self.sender_notifier.notified().await;

            while let Some(packet) = self.send_queue.pop() {
                trace!("Sent packet with len: {:?}", packet.len());
                self.socket.write_all(&packet).await?;
            }

            // TODO: Implement a way to break the loop when the connection is closed
/*            if ([State::Unknown, State::Handshake].contains(&self.state)) {
                trace!("Breaking the connection, state isn't unknown/handshake anymore");
                break;
            }
*/
        }
    }

    pub async fn receiver(&mut self) -> Result<()> {
        trace!("Starting receiver for the same addy: {:?}", self.socket.peer_addr()?);

        loop {
            let mut length_buffer = vec![0u8; 1];
            self.socket.read_exact(&mut length_buffer).await?;

            let length = length_buffer[0] as usize;

            let mut buffer = vec![0u8; length];

            self.socket.read_exact(&mut buffer).await?;

            let buffer = vec![length_buffer, buffer].concat();

            let mut cursor = Cursor::new(buffer);

            let packet_length = read_varint(&mut cursor).await?;
            let packet_id = read_varint(&mut cursor).await?;

            if packet_id.get_val() != 0x00 {
                return Err(Error::InvalidPacketId(packet_id.get_val() as u32));
            }

            trace!("Packet Length: {}", packet_length);
            trace!("Packet ID: {}", packet_id);

            let handshake_packet = HandshakePacket::decode(&mut cursor).await?;

            handshake_packet.test_method_to_handle_handshake_packet(self).await?;
        }

    }

    pub fn send_packet_to_conn(&mut self, packet: Vec<u8>) {
        self.send_queue.push(packet);
        self.sender_notifier.notify_one();
    }
}

