use log::debug;
use tokio::io::AsyncWriteExt;
use ferrumc_macros::{Decode, packet};
use ferrumc_utils::encoding::varint::VarInt;
use ferrumc_utils::type_impls::Encode;
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
    async fn handle(&self, conn: &mut tokio::sync::RwLockWriteGuard<'_, Connection>) -> Result<(), Error> {
        debug!("LoginStart packet received");
        debug!("Username: {}", self.username);
        debug!("UUID: {:X}", self.uuid);
        let response = crate::packets::outgoing::login_success::LoginSuccess {
            packet_id: VarInt::from(0x02),
            uuid: self.uuid,
            username: self.username.clone(),
            property_count: VarInt::from(0),
        };
        conn.socket.write_all(response.encode().await?.as_slice()).await?;
        Ok(())
    }
}