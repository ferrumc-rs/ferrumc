use std::collections::HashMap;

use async_trait::async_trait;
use serde::Serialize;
use tokio::net::TcpStream;

use crate::utils::write_varint;

use super::super::Packet;

pub struct PacketPlayOutStatus {
    pub motd: String,
}

#[async_trait]
impl Packet for PacketPlayOutStatus {
    fn serialize(&self) -> Vec<u8> {
        todo!()
        //     Not used, but required by the trait, Should probably fix it 💀
    }

    fn deserialize(_bytes: Vec<u8>) -> Result<Self, anyhow::Error> where Self: Sized {
        todo!()
    }

    fn get_id(&self) -> u32 {
        0x00
    }

    fn get_name(&self) -> String {
        todo!()
    }

    async fn handle(&self, _stream: &mut TcpStream) {
        todo!()
    }
}

impl PacketPlayOutStatus {
    pub async fn specialized_serialize(&self, _stream: &mut TcpStream) -> Result<Vec<u8>, anyhow::Error> {
        let mut sample = HashMap::new();

        sample.insert("name".to_string(), "thinkofdeath".to_string());
        sample.insert("id".to_string(), "4566e69f-c907-48ee-8d71-d7ba5aa00d20".to_string());

        let payload = JsonResponse {
            version: Version {
                name: "Uhh, What's up? I think you are on the wrong version lmao bro get on the right version XDDDD. Skill issue bro :: gitgud lil bro skull emoji moment".to_string(),
                protocol: 756,
            },
            players: Players {
                max: 100,
                online: 0,
                sample,
            },
            description: Description {
                // text: "Hello from the server!".to_string(),
                text: self.motd.clone(),
            },
        };

        let json_bytes = serde_json::to_vec(&payload)?;

        let mut temp_buffer = vec![];

        // Write Packet ID (0x00)
        write_varint(&mut temp_buffer, 0x00).await;

        // Write the length of the JSON string as VarInt
        write_varint(&mut temp_buffer, json_bytes.len() as i32).await;

        // Write JSON string bytes
        temp_buffer.extend_from_slice(&*json_bytes);

        let packet_length = temp_buffer.len() as i32;

        let mut final_buffer = vec![];

        // Write the total packet length as VarInt
        write_varint(&mut final_buffer, packet_length).await;

        final_buffer.extend_from_slice((&temp_buffer).as_ref());

        Ok(final_buffer)
    }
}

// Clone trait to allow self.clone() in specialized_serialize
impl Clone for PacketPlayOutStatus {
    fn clone(&self) -> Self {
        Self {
            motd: self.motd.clone(),
        }
    }
}

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
    sample: HashMap<String, String>,
}

#[derive(Serialize)]
pub struct Description {
    text: String,
}