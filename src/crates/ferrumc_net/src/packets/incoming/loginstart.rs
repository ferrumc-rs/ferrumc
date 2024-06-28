use std::sync::Arc;
use log::debug;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use ferrumc_macros::{Decode, packet};
use ferrumc_utils::encoding::varint::VarInt;
use tokio::sync::RwLock;
use crate::Connection;
use crate::packets::IncomingPacket;

#[derive(Decode)]
#[packet(packet_id = 0x00, state = "login")]
pub struct LoginStart {
    pub username: String,
    pub _uuid_pad: bool,
    pub uuid: u128,
}

impl IncomingPacket for LoginStart {
    async fn handle(&self, conn: & mut Connection) -> Result<(), Error> {
        debug!("LoginStart packet received");
        debug!("Username: {}", self.username);
        debug!("UUID: {:X}", self.uuid);
        let namespace_uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, "OfflinePlayer".as_bytes());

        let uuid = Uuid::new_v3(&namespace_uuid, self.username.as_bytes());
        let response = crate::packets::outgoing::login_success::LoginSuccess {
            packet_id: VarInt::from(0x02),
            uuid: uuid.as_u128(),
            username: "OfflinePlayer".to_string(),
            property_count: VarInt::from(0),
        };
        conn.socket.write_all(response.encode().await?.as_slice()).await?;
        Ok(())
    }
}