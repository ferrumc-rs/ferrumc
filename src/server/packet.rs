use anyhow::Result;
use anyhow::Error;
use tokio::net::TcpStream;
use std::convert::TryInto;
use serde_json::to_string;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::PipeEnd::Server;

use crate::server::handshake::{handle_handshake, HandshakePacket};

pub enum Packet {
    Handshake(HandshakePacket),
    Pong,
    // TODO: Add more packets
}
const LEGACY_REQUEST: [u8; 35] = [
    0xfe, // 1st packet id: 0xfe for server list ping
    0x01, // payload: always 1
    0xfa, // 2nd packet id: 0xfa for plugin message
    0x00, 0x0b, // length of following string: always 11 as short,
    0x00, 0x4d, 0x00, 0x43, 0x00, 0x7c, 0x00, 0x50, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x67, 0x00, 0x48,
    0x00, 0x6f, 0x00, 0x73, 0x00, 0x74,
    // MC|PingHost as UTF16-BE
    7,    // length of the rest of the data: 7 + length of hostname
    0x4a, // protocol version: 0x4a for the last version
    0x00, 0x00, // length of hostname: 0 as short
    0x00, 0x00, 0x00, 0x00, // port: 0 as int
];
pub async fn decode_packet(data: &[u8]) -> Result<Packet> {
    let packet_id = data[1];
    // println!("Packet ID is {:?}", packet_id);

    match packet_id {
        0x00 => {
            HandshakePacket::from_bytes(&data[1..])
                .map(|handshake_packet| Packet::Handshake(handshake_packet))
                .map_err(Error::msg)
        },
        0x01 => {
            Ok(Packet::Pong)
        }

        // 0x01 => {
        //     // println!("Pong packet received: {:?}", &data[1..]);
        //     let response = PongPacket::construct_legacy_response()?;
        //     Ok(Packet::Pong(response))
        //     // PongPacket::new(&data[1..])
        //     //     .map(|pong_packet| Packet::Pong(pong_packet))
        //     //     .map_err(Error::msg)
        // },
        _ => Err(Error::msg("Unknown packet ID: ".to_string() + &packet_id.to_string())),
    }
}

pub async fn handle_packet(packet: Packet, stream: &mut TcpStream) -> Result<()> {
    match packet {
        Packet::Handshake(handshake_packet) => {
            handle_handshake(handshake_packet, stream).await?;
        },
        Packet::Pong => {
            // println!("Pong packet received: {:?}", &data[1..]);
            // println!("Handling Pong packet");
            // stream.write_all(&LEGACY_REQUEST).await?;
            // stream.flush().await?;

            // let mut buffer = Vec::new();
            // stream.read_to_end(&mut buffer).await?;
            //
            // println!("Pong packet received: {:?}", &buffer);

            // Ok(Packet::Pong(response))
            // PongPacket::new(&data[1..])
            //     .map(|pong_packet| Packet::Pong(pong_packet))
            //     .map_err(Error::msg)
        }
        // Packet::Pong(pong_bytes) => {
        //     // println!("Pong packet received: {:?}", pong_bytes);
        //     // PongPacket::construct_and_send_legacy_response(socket).await?;
        //     socket.write_all(&pong_bytes).await?;
        //     // socket.write_all(&pong_packet.to_bytes()).await?;
        // },
    }
    Ok(())
}

pub struct PongPacket;

impl PongPacket {
    pub fn construct_legacy_response() -> Result<Vec<u8>> {
        let mut response: Vec<u8> = Vec::new();

        // Packet Identifier (0xFF kick packet)
        response.push(0xFF);

        let protocol_version = "47";
        let minecraft_server_version = "1.4.2";
        let message_of_the_day = "A Minecraft Server";
        let current_player_count = "0";
        let max_players = "20";

        let data_string = format!(
            "\u{00A7}1\u{0000}{}\u{0000}{}\u{0000}{}\u{0000}{}\u{0000}{}",
            protocol_version,
            minecraft_server_version,
            message_of_the_day,
            current_player_count,
            max_players,
        );

        // Convert to UTF-16BE bytes
        let data_utf16: Vec<u8> = data_string.encode_utf16().flat_map(|unit| vec![(unit >> 8) as u8, (unit & 0xFF) as u8]).collect();

        // Write length (Note: this assumes the total length fits in a u16, which should be the case)
        let length = (data_utf16.len() / 2) as u16;
        response.extend_from_slice(&length.to_be_bytes());

        // Write the UTF-16BE encoded string
        response.extend(data_utf16);

        Ok(response)
    }
}