use log::info;
use ferrumc_macros::Decode;
use ferrumc_utils::encoding::varint::VarInt;
use crate::Connection;
use crate::packets::IncomingPacket;
use crate::packets::outgoing::ping::OutgoingPing;

#[derive(Decode)]
pub struct IncomingPing {
    pub payload: i64,
}

impl IncomingPacket for IncomingPing {
    async fn handle(&self, _: &mut Connection) -> Result<Option<Vec<u8>>, ferrumc_utils::error::Error> {
        info!("Handling ping packet");

        let response = OutgoingPing {
            packet_id: VarInt::from(0x01),
            payload: self.payload,
        };

        Ok(Some(response.encode().await?))
    }
}