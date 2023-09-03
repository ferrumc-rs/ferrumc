use std::io::{Cursor, Read};

use anyhow::Result;
use async_trait::async_trait;
use byteorder::{BigEndian, ReadBytesExt};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::network::packet::outbound::packet_play_out_status::PacketPlayOutStatus;
use crate::utils::read_varint;
use crate::utils::write_varint;

use super::super::Packet;

pub struct PacketPlayInHandshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: i32,
}

#[async_trait]
impl Packet for PacketPlayInHandshake {
    fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    fn deserialize(data: Vec<u8>) -> Result<Self, anyhow::Error> where Self: Sized {
        let mut cursor = Cursor::new(data);
        // Read packet length
        let _packet_length = read_varint(&mut cursor)?;

        let packet_id = read_varint(&mut cursor)?;
        if packet_id != 0 {
            println!("Packet ID is {:?}", packet_id);
            panic!("Invalid packet ID")
        }

        // Read protocol version ? (should be 756 for 1.17.1)
        let protocol_version = read_varint(&mut cursor)?;

        let address_length = read_varint(&mut cursor)? as usize;
        let mut address_bytes = vec![0u8; address_length];
        cursor.read_exact((&mut address_bytes).as_mut())?;
        let server_address = String::from_utf8(address_bytes)?;

        let server_port = cursor.read_u16::<BigEndian>()?;

        // Next state :: 1 for status, 2 for login
        let next_state = read_varint(&mut cursor)?;

        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }

    fn get_id(&self) -> u32 {
        0x00
    }

    fn get_name(&self) -> String {
        todo!()
    }

    async fn handle(&self, stream: &mut TcpStream) {
        // log all the data

        // println!("Protocol Version: {}", self.protocol_version);
        // println!("Server Address: {}", self.server_address);
        // println!("Server Port: {}", self.server_port);
        // println!("Next State: {}", self.next_state);

        // send back a response with Out: Status packet.

        let status_packet = PacketPlayOutStatus {
            motd: "Hello, this is a Minecraft server made in Rust.".to_string(),
        };

        if let Ok(serialized_data) = status_packet.specialized_serialize(stream).await {
            let _ = stream.write_all((&serialized_data).as_ref()).await;
            // println!("sent data: {:?}", serialized_data);
        }

        // send a new buffer with [1,0] for ping
        let mut ping_buffer = vec![];
        write_varint(&mut ping_buffer, 0x01).await;
        write_varint(&mut ping_buffer, 0x00).await;

        let _ = stream.write_all((&ping_buffer).as_ref()).await;
    }
}