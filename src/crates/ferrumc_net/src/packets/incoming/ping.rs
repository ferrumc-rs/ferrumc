use ferrumc_macros::{Decode, packet};
use ferrumc_utils::encoding::varint::VarInt;
use ferrumc_utils::prelude::*;
use log::info;
use tokio::io::AsyncWriteExt;
use crate::Connection;
use crate::packets::IncomingPacket;
use crate::packets::outgoing::ping::OutgoingPing;

#[derive(Decode)]
#[packet(packet_id = 0x01, state = "status")]
pub struct Ping {
    pub payload: i64,
}

impl IncomingPacket for Ping {
    async fn handle(&self, conn: &mut Connection) -> Result<()> {
        info!("Handling ping packet");

        let response = OutgoingPing {
            packet_id: VarInt::from(0x01),
            payload: self.payload,
        };

        let response = response.encode().await?;

        conn.drop = true;

        conn.socket.write_all(&response).await.map_err(|e| e.into())
    }
}