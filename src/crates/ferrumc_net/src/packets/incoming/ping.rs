use log::info;
use ferrumc_macros::{Decode, packet};
use ferrumc_utils::encoding::varint::VarInt;
use crate::Connection;
use crate::packets::IncomingPacket;
use crate::packets::outgoing::ping::OutgoingPing;

#[derive(Decode)]
#[packet(packet_id = 0x01, state = "status")]
pub struct Ping {
    pub payload: i64,
}

impl IncomingPacket for Ping {
    async fn handle(&self, conn: &mut Connection) -> Result<(), ferrumc_utils::error::Error> {
        info!("Handling ping packet");

        let response = OutgoingPing {
            packet_id: VarInt::from(0x01),
            payload: self.payload,
        };

        let response = response.encode().await?;

        conn.send_packet(response).await
    }
}