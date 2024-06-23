use tokio::io::{AsyncRead, AsyncSeek, AsyncWriteExt};
use std::fmt::Display;
use ferrumc_macros::{Decode, Encode};
use ferrumc_utils::encoding::varint::VarInt;

use ferrumc_utils::prelude::*;
use ferrumc_utils::type_impls::Encode;
use log::{trace};
use crate::Connection;

#[derive(Decode, Debug)]
pub struct HandshakePacket {
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

impl HandshakePacket {
    pub async fn test_method_to_handle_handshake_packet(self, conn: &mut Connection) -> Result<()> {
        trace!("Handling handshake packet");

        let test_json = r#"{
            "version": {
                "name": "1.19.4",
                "protocol": 762
            },
            "players": {
                "max": 100,
                "online": 5,
                "sample": [
                    {
                        "name": "thinkofdeath",
                        "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                    }
                ]
            },
            "description": {
                "text": "no way! it actually works LMAOOO WWWWW??"
            },
            "favicon": "data:image/png;base64,<data>",
            "enforcesSecureChat": false,
            "previewsChat": false
        }"#;


        let packet = ClientBoundHandshakePacket {
            packet_id: VarInt::new(0x00),
            res_json: test_json.to_string(),
        };

        let serialized = packet.encode().await?;

        conn.add_to_send_queue(serialized);

        Ok(())
    }
}

#[derive(Encode, Debug)]
struct ClientBoundHandshakePacket {
    packet_id: VarInt,
    res_json: String,
}