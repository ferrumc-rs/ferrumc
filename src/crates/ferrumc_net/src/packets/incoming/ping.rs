use ferrumc_macros::{Decode, packet};
use ferrumc_utils::encoding::varint::VarInt;
use ferrumc_utils::prelude::*;
use ferrumc_utils::type_impls::Encode;
use tokio::io::AsyncWriteExt;
use tracing::info;
use crate::Connection;
use crate::packets::IncomingPacket;
use crate::packets::outgoing::ping::OutgoingPing;

/// This ping packet is sent by the client to the server to request a pong.
/// 
/// The payload is a random number that the server should return in the pong.
/// For some reason, seems to be required for the client to acknowledge the server's status response.
#[derive(Decode)]
#[packet(packet_id = 0x01, state = "status")]
pub struct Ping {
    pub payload: i64,
}

impl IncomingPacket for Ping {
    async fn handle(&self, conn: &mut Connection) -> Result<()> {
        info!("Handling ping packet");

        // tokio::io::AsyncWriteExt::write_all()
        let response = OutgoingPing {
            packet_id: VarInt::from(0x01),
            payload: self.payload,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        response.encode(&mut cursor).await?;
        let response = cursor.into_inner();

        conn.drop = true;

        conn.socket.write_all(&*response).await.map_err(|e| e.into())
    }
}