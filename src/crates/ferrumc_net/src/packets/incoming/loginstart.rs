use log::debug;
use tokio::io::AsyncWriteExt;
use ferrumc_macros::{Decode, packet};
use crate::Connection;
use crate::packets::IncomingPacket;

#[derive(Decode)]
#[packet(packet_id = 0x00, state = "login")]
pub struct LoginStart {
    pub username: String,
    pub uuid: u128,
}

impl IncomingPacket for LoginStart {
    async fn handle(&self, conn: &mut tokio::sync::RwLockWriteGuard<'_, Connection>) -> Result<(), Error> {
        debug!("LoginStart packet received");
        debug!("Username: {}", self.username);
        debug!("UUID: {:X}", self.uuid);
        conn.socket.write_all(crate::packets::outgoing::login_disconnect::LoginDisconnect {
            packet_id: Default::default(),
            reason: "Login not yet implemented!".to_string()
        }.encode().await?.as_slice()).await?;
        Ok(())
    }
}