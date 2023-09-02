use std::fmt;
use std::fmt::Display;

use anyhow::Result;
use serde::Serialize;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Serialize)]
pub struct JsonResponse {
    version: Version,
    players: Players,
    description: Description,
}

#[derive(Serialize)]
pub struct Version {
    name: String,
    protocol: i32,
}

#[derive(Serialize)]
pub struct Players {
    max: i32,
    online: i32,
}

#[derive(Serialize)]
pub struct Description {
    text: String,
}

// A function to handle Status packets.
// pub async fn handle_status(socket: &mut TcpStream) -> Result<()> {
//     let response = JsonResponse {
//         version: Version {
//             name: "1.17.1".to_string(),
//             protocol: 756,
//         },
//         players: Players {
//             max: 100,
//             online: 0,
//         },
//         description: Description {
//             text: "Hello from the server!".to_string(),
//         },
//     };
//
//     // println!("Sending status response");
//     // let json_str = serde_json::to_string(&response)?;
//     let response = r#"{
//         "version": {
//             "name": "1.17.1",
//             "protocol": 756
//         },
//         "players": {
//             "max": 100,
//             "online": 0
//         },
//         "description": {
//             "text": "Hello from the server!"
//         }
//     }"#;
//
//     socket.write_all(response.as_bytes()).await?;
//     Ok(())
// }
// fn write_varint(value: i32, buf: &mut Vec<u8>) {
//     let mut value = value as u32;
//     loop {
//         let mut temp = (value & 0b01111111) as u8;
//         value >>= 7;
//         if value != 0 {
//             temp |= 0b10000000;
//         }
//         buf.push(temp);
//         if value == 0 {
//             break;
//         }
//     }
// }


pub async fn handle_status(socket: &mut TcpStream) -> Result<()> {
    handle_status_latest(socket).await
}

const LATEST_REQUEST: [u8; 127] = [
    0x7E,
    0x00,
    0x7B, 0x22, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x22, 0x3A, 0x7B, 0x22, 0x6E, 0x61, 0x6D, 0x65,
    0x22, 0x3A, 0x22, 0x31, 0x2E, 0x31, 0x37, 0x2E, 0x31, 0x22, 0x2C, 0x22, 0x70, 0x72, 0x6F, 0x74, 0x6F,
    0x63, 0x6F, 0x6C, 0x22, 0x3A, 0x37, 0x35, 0x36, 0x7D, 0x2C, 0x22, 0x70, 0x6C, 0x61, 0x79, 0x65, 0x72,
    0x73, 0x22, 0x3A, 0x7B, 0x22, 0x6D, 0x61, 0x78, 0x22, 0x3A, 0x31, 0x30, 0x30, 0x2C, 0x22, 0x6F, 0x6E,
    0x6C, 0x69, 0x6E, 0x65, 0x22, 0x3A, 0x30, 0x7D, 0x2C, 0x22, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70,
    0x74, 0x69, 0x6F, 0x6E, 0x22, 0x3A, 0x7B, 0x22, 0x74, 0x65, 0x78, 0x74, 0x22, 0x3A, 0x22, 0x48, 0x65,
    0x6C, 0x6C, 0x6F, 0x20, 0x66, 0x72, 0x6F, 0x6D, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x21, 0x22, 0x7D, 0x7D
];

// async fn handle_status_latest(stream: &mut TcpStream) -> Result<()> {
//     let json_str =
//     r#"{{
//     "version": {
//         "name": "1.19.4",
//         "protocol": 762
//     },
//     "players": {
//         "max": 100,
//         "online": 5,
//         "sample": [
//             {
//                 "name": "thinkofdeath",
//                 "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
//             }
//         ]
//     },
//     "description": {
//         "text": "Hello world"
//     },
//     "favicon": "data:image/png;base64,<data>",
//     "enforcesSecureChat": true,
//     "previewsChat": true
// }}"#;
//
//     let mut temp_buffer = vec![];
//
//     write_varint(&mut temp_buffer, 0x00).await;
//
//     temp_buffer.extend_from_slice(json_str.as_bytes());
//
//     let packet_length = temp_buffer.len() as i32;
//
//     let mut final_buffer = vec![];
//
//     write_varint(&mut final_buffer, packet_length).await;
//
//     println!("Final buffer: {:?}", &final_buffer);
//
//     final_buffer.extend_from_slice(&temp_buffer);
//
//     stream.write_all(&final_buffer).await?;
//     println!("Sent status response: {:?}", &final_buffer);
//
//     stream.flush().await?;
//
//     Ok(())
// }

async fn handle_status_latest(stream: &mut TcpStream) -> Result<()> {
    let json_str =
        r#"{
        "version": {
            "name": "1.17.1",
            "protocol": 756
        },
        "players": {
            "max": 10,
            "online": 0
        },
        "description": {
            "text": "\u00A7a             SkyItems Network \u00A7c[1.8-1.19]\n\u00A76\u00A7l    SKYBLOCK REMAKE 0.1\u00A7r \u00A7f|\u00A7r \u00A7b\u00A7lDUNGEONS\u00A7r\u00A7c\u00A7l (dev)\u00A7r"
        },
        "previewsChat": true
    }"#;

    let json_bytes = json_str.as_bytes();

    let mut temp_buffer = vec![];

    // Write Packet ID (0x00)
    write_varint(&mut temp_buffer, 0x00).await;

    // Write the length of the JSON string as VarInt
    write_varint(&mut temp_buffer, json_bytes.len() as i32).await;

    // Write JSON string bytes
    temp_buffer.extend_from_slice(json_bytes);

    let packet_length = temp_buffer.len() as i32;

    let mut final_buffer = vec![];

    // Write the total packet length as VarInt
    write_varint(&mut final_buffer, packet_length).await;

    final_buffer.extend_from_slice(&temp_buffer);

    // Send the packet
    stream.write_all(&final_buffer).await?;
    // println!("Sent status response: {:?}", &final_buffer);

    stream.flush().await?;

    Ok(())
}


// used in read_varint implemenetation
const LAST_SEVEN_BITS: i32 = 0b0111_1111;
const NEXT_BYTE_EXISTS: u8 = 0b1000_0000;

// bit mask to remove remaining 7 MSB's after right shift
const SEVEN_BITS_SHIFT_MASK: i32 = 0x01_ff_ff_ff;

async fn write_varint(sink: &mut Vec<u8>, mut value: i32) {
    loop {
        let mut temp = (value & LAST_SEVEN_BITS) as u8;
        // i32 right shift is arithmetic shift (preserves MSB)
        value >>= 7;
        value &= SEVEN_BITS_SHIFT_MASK;
        if value != 0 {
            temp |= NEXT_BYTE_EXISTS;
        }
        sink.push(temp);
        if value == 0 {
            break;
        }
    }
}
//
//
// async fn write_varint(buf: &mut Vec<u8>, mut value: i32) {
//     while (value & -0x80) != 0 {
//         buf.push(((value & 0x7F) | 0x80).try_into().unwrap());
//         value = ((value >> 7) & 0x01FFFFFF);  // Only keep the 25 least significant bits
//     }
//     buf.push(value as u8);
// }


async fn read_varint<Stream>(stream: &mut Stream) -> Result<i32>
    where
        Stream: AsyncRead + Unpin,
{
    let mut buffer = [0u8];
    let mut result = 0;
    let mut read_count = 0u32;
    loop {
        stream.read_exact(&mut buffer).await?;
        result |= (buffer[0] as i32 & LAST_SEVEN_BITS).checked_shl(7 * read_count)
            .ok_or(Error::UnsupportedProtocol)?;

        read_count += 1;
        if read_count > 5 {
            break Err(Error::UnsupportedProtocol.into());
        } else if (buffer[0] & NEXT_BYTE_EXISTS) == 0 {
            break Ok(result);
        }
    }
}

#[derive(Debug)]
pub enum Error {
    UnsupportedProtocol,
    // ... other error variants can be added here
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnsupportedProtocol => write!(f, "Unsupported protocol version"),
            // ... other error variants can be added here
        }
    }
}

impl std::error::Error for Error {}