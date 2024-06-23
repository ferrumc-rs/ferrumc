#![feature(box_into_inner)]

use std::cmp::PartialEq;
use std::io::Cursor;
use std::sync::{Arc, atomic, OnceLock};
use std::sync::atomic::AtomicU32;

use dashmap::DashMap;
use ferrumc_utils::encoding::varint::{read_varint};
use ferrumc_utils::prelude::*;
use lariv::Lariv;
use log::{debug, error, trace};
use rand::random;
use tokio::io::{AsyncWriteExt};
use tokio::io::AsyncReadExt;
use tokio::sync::{RwLock};
use crate::packets::incoming::handshake;
use crate::packets::IncomingPacket;

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
        state: State::Unknown,
    };

    CONNECTIONS().connections.insert(id, conn);
    CONNECTIONS()
        .connection_count
        .fetch_add(1, atomic::Ordering::Relaxed);

    trace!("Connection established with id: {}", id);

    let mut conn_ref = CONNECTIONS().connections.get_mut(&id).ok_or(Error::ConnectionNotFound(id))?;
    conn_ref.start_connection().await?;

    /*    let conn_ref = CONNECTIONS().connections.get(&id).ok_or(Error::ConnectionNotFound(id))?;
        let mut conn_write = conn_ref.write_owned().await;

        conn_write.start_connection().await?;*/

    Ok(())
}

impl Connection {
    pub async fn start_connection(&mut self) -> Result<()> {
        self.state = State::Handshake;
        debug!("Starting connection with id: {}", self.id);

        self.manage_conn().await?;


        Ok(())
    }

    pub async fn manage_conn(&mut self) -> Result<()> {
        debug!("Starting receiver for the same addy: {:?}", self.socket.peer_addr()?);

        loop {
            let mut drop_connection: bool = false;
            let mut length_buffer = vec![0u8; 1];
            self.socket.read_exact(&mut length_buffer).await?;

            let length = length_buffer[0] as usize;

            let mut buffer = vec![0u8; length];

            self.socket.read_exact(&mut buffer).await?;

            let buffer = vec![length_buffer, buffer].concat();

            let mut cursor = Cursor::new(buffer);

            let packet_length = read_varint(&mut cursor).await?;
            let packet_id = read_varint(&mut cursor).await?;

            trace!("Packet Length: {}", packet_length);
            trace!("Packet ID: {}", packet_id);

            let response = match self.state {
                State::Handshake => {
                    match packet_id.get_val() {
                        0x00 => {
                            let handshake_packet = handshake::Handshake::decode(&mut cursor).await?;
                            handshake_packet.handle(self).await
                        }
                        _ => {
                            error!("Invalid packet id: {}", packet_id);
                            Ok(None)
                        }
                    }
                }
                State::Status => {
                    match packet_id.get_val() {
                        0x00 => {
                            let request_packet = packets::incoming::status::IncomingStatusRequest::decode(&mut cursor).await?;
                            drop_connection = true;
                            request_packet.handle(self).await
                        }
                        _ => {
                            error!("Invalid packet id: {}", packet_id);
                            drop_connection = true;
                            Ok(None)
                        }
                    }
                }
                
                _ => {
                    error!("Invalid state: {:?}", self.state);
                    drop_connection = true;
                    Ok(None)
                }
            };

            match response {
                Ok(response) => {
                    if let Some(response) = response {
                        self.socket.write_all(&response).await?;
                    }
                }
                Err(e) => {
                    error!("Error handling packet: {:?}", e);
                }
            }
            if drop_connection {
                Self::drop_conn(self).await?;
                break;
            }

        }
        Ok(())
    }

    async fn drop_conn(connection: &mut Connection) -> Result<()> {
        trace!("Dropping connection with id: {}", connection.id);
        let id = connection.id;
        CONNECTIONS().connections.remove(&id);
        CONNECTIONS().connection_count.fetch_sub(1, atomic::Ordering::Relaxed);
        Ok(())
    }
}


