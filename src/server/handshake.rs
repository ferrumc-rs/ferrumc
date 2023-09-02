use std::io::{Cursor, Read};

use anyhow::Error;
use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};
use tokio::net::TcpStream;

use crate::server::status::handle_status;

pub async fn handle_handshake(handshake_packet: HandshakePacket, socket: &mut TcpStream) -> Result<()> {
    println!("Handshake packet received: {:?}", handshake_packet);

    if handshake_packet.get_next_state() == 1 {
        handle_status(socket).await?;
    }

    Ok(())
}

/* PACKET FORMAT

 : [15, 0, 47, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]

 Example:
 15 - Varint - length of the packet
 0  - Varint - packet ID - Handshake
 47 - VarInt - protocol version
      Following bytes are length prefixed server address in UTF-8:
 9  - Lenght
 108 - l
 111 - o
 99  - c
 97  - a
 108 - l
 104 - h
 111 - o
 115 - s
 116 - t
      Following 2 bytes are server port in unsigned short format (I guess 7777)
 30
 97
 1  - Varint - Next state - 1 for status, 2 for login
 */

fn read_varint<R: Read>(mut reader: R) -> Result<i32> {
    let mut num_read = 0;
    let mut result = 0;
    let mut read = 0x80; // Dummy value to start the loop

    while (read & 0x80) != 0 {
        read = reader.read_u8()?; // Read one byte
        let val = read & 0x7F; // Take the last 7 bits of the byte
        result |= (val as i32) << (7 * num_read); // Shift the 7 bits to their proper place

        num_read += 1;

        if num_read > 5 {
            return Err(Error::msg("VarInt is too large"));
        }
    }

    Ok(result)
}

#[derive(Clone, Debug)]
pub struct HandshakePacket {
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    next_state: i32,
}

impl HandshakePacket {
    pub fn new(protocol_version: i32, server_address: String, server_port: u16, next_state: i32) -> Self {
        Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        }
    }

    pub fn get_protocol_version(&self) -> i32 {
        self.protocol_version
    }

    pub fn get_server_address(&self) -> &str {
        &self.server_address
    }

    pub fn get_server_port(&self) -> u16 {
        self.server_port
    }

    pub fn get_next_state(&self) -> i32 {
        self.next_state
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&self.protocol_version.to_be_bytes());
        bytes.extend_from_slice(&self.server_address.as_bytes());
        bytes.extend_from_slice(&self.server_port.to_be_bytes());
        bytes.extend_from_slice(&self.next_state.to_be_bytes());

        bytes
    }

    pub fn from_bytes(data: &[u8]) -> Result<HandshakePacket> {
        let mut cursor = Cursor::new(data);

        // Read packet length
        // let _packet_length = read_varint(&mut cursor)?;

        // Read Packet ID (should be 0 for a Handshake packet)
        let packet_id = read_varint(&mut cursor)?;
        if packet_id != 0 {
            println!("Packet ID is {:?}", packet_id);
            return Err(Error::msg("Invalid packet ID"));
        }

        // Read protocol version
        let protocol_version = read_varint(&mut cursor)?;

        // Read server address
        let address_length = read_varint(&mut cursor)? as usize;
        let mut address_bytes = vec![0u8; address_length];
        cursor.read_exact(&mut address_bytes)?;
        let server_address = String::from_utf8(address_bytes)?;

        // Read server port
        let server_port = cursor.read_u16::<BigEndian>()?;

        // Read next state
        let next_state = read_varint(&mut cursor)?;

        Ok(HandshakePacket {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}
