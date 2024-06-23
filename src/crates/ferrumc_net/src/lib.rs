#![feature(box_into_inner)]

use std::cmp::PartialEq;
use std::fmt::Display;
use std::io::Cursor;
use std::sync::{Arc, atomic, OnceLock};
use std::sync::atomic::AtomicU32;
use std::task::Poll;

use dashmap::DashMap;
use ferrumc_macros::Decode;
use ferrumc_utils::encoding::varint::{read_varint, VarInt};
use ferrumc_utils::prelude::*;
use ferrumc_utils::type_impls::Decode;
use lariv::Lariv;
use lazy_static::lazy_static;
use log::{debug, trace};
use rand::random;
use tokio::io::{AsyncRead, AsyncWriteExt};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncSeek;
use tokio::sync::{RwLock, Mutex};

use crate::State::Handshake;

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
    pub connections: DashMap<u32, Arc<Mutex<Connection>>>,
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
    };
    let conn = Arc::new(Mutex::new(conn));

    Connection::start_connection(conn.clone()).await?;

    CONNECTIONS().connections.insert(id, conn);
    CONNECTIONS()
        .connection_count
        .fetch_add(1, atomic::Ordering::Relaxed);

    trace!("Connection established with id: {}", id);

    /*    let conn_ref = CONNECTIONS().connections.get(&id).ok_or(Error::ConnectionNotFound(id))?;
        let mut conn_write = conn_ref.write_owned().await;

        conn_write.start_connection().await?;*/

    Ok(())
}

impl Connection {
    pub async fn start_connection(conn_arc: Arc<Mutex<Self>>) -> Result<()> {
        let conn = conn_arc.clone();
        let mut conn = conn.lock().await;
        conn.state = State::Handshake;
        debug!("Starting connection with id: {}", conn.id);
        drop(conn);

        // let self_clone = conn_arc.clone();

        // self_clone.lock().await.receiver().await?;

        let conn = conn_arc.clone();
        tokio::spawn(async move {
            let mut conn = conn.lock().await;
            if let Err(e) = conn.sender().await {
                trace!("Error in sender: {:?}", e);
            }
        });

        let conn = conn_arc.clone();
        tokio::spawn(async move {
            let mut conn = conn.lock().await;
            if let Err(e) = conn.receiver().await {
                trace!("Error in receiver: {:?}", e);
            }
        });


        Ok(())
    }

    pub async fn sender(&mut self) -> Result<()> {
        debug!("Starting sender for connection with addy: {:?}", self.socket.peer_addr()?);

        loop {
            for packet in self.send_queue.iter() {
                self.socket.write_all(&packet).await?;
            }

            // TODO: Implement a way to break the loop when the connection is closed
            if ([State::Unknown, State::Handshake].contains(&self.state)) {
                trace!("Breaking the connection, state isn't unknown/handshake anymore");
                break;
            }

            // TODO: Implement a proper tick system
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }

        Ok(())
    }

    pub async fn receiver(&mut self) -> Result<()> {
        debug!("Starting receiver for the same addy: {:?}", self.socket.peer_addr()?);

        loop {
            let mut length_buffer = vec![0u8; 1];
            self.socket.read_exact(&mut length_buffer).await?;

            let length = length_buffer[0] as usize;

            let mut buffer = vec![0u8; length];

            self.socket.read_exact(&mut buffer).await?;

            let mut buffer = vec![length_buffer, buffer].concat();

            let mut cursor = Cursor::new(buffer);

            let packet_length = read_varint(&mut cursor).await?;
            let packet_id = read_varint(&mut cursor).await?;

            // trace!("Packet Length: {}", packet_length);
            // trace!("Packet ID: {}", packet_id);

            if (packet_id.get_val() != 0x00) {
                return Err(Error::InvalidPacketId(packet_id.get_val() as u32));
            }

            let handshake_packet = HandshakePacket::decode(&mut cursor).await?;

            trace!("{}", handshake_packet);
        }

        Ok(())
    }

    /*    pub async fn start_connection(&mut self) -> Result<()> {
            trace!("Starting connection with id: {}", self.id);
            self.state = State::Handshake;

            tokio::spawn(async move {
                let res = self.sender().await;

                if let Err(e) = res {
                    trace!("Error in sender: {:?}", e);
                }
            });

            loop {

                let mut length_buffer = vec![0u8; 1];
                self.socket.read_exact(&mut length_buffer).await?;

                let length = length_buffer[0] as usize;

                let mut buffer = vec![0u8; length];

                self.socket.read_exact(&mut buffer).await?;

                let mut buffer = vec![length_buffer, buffer].concat();

                let mut cursor = Cursor::new(buffer);

                let packet_length = read_varint(&mut cursor).await?;
                let packet_id = read_varint(&mut cursor).await?;

                // trace!("Packet Length: {}", packet_length);
                // trace!("Packet ID: {}", packet_id);

                if (packet_id.get_val() != 0x00) {
                    return Err(Error::InvalidPacketId(packet_id.get_val() as u32));
                }

                let handshake_packet = HandshakePacket::decode(&mut cursor).await?;

                trace!("{}", handshake_packet);
            }
        }
    */
    /*  pub async fn sender(&mut self) -> Result<()> {
          loop {
              for packet in self.send_queue.iter() {
                  self.socket.write_all(&packet).await?;
              }

              // TODO: Implement a way to break the loop when the connection is closed
              if ([State::Unknown, State::Handshake].contains(&self.state)) {
                  trace!("Breaking the connection, state isn't unknown/handshake anymore");
                  break;
              }

              // TODO: Implement a proper tick system
              tokio::time::sleep(std::time::Duration::from_millis(50)).await;
          }

          Ok(())
      }*/
}

#[derive(Decode, Debug)]
struct HandshakePacket {
    protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    next_state: VarInt,
}

impl Display for HandshakePacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handshake Packet: Protocol Version: {}, Server Address: {}, Server Port: {}, Next State: {}", self.protocol_version, self.server_address, self.server_port, self.next_state)
    }
}