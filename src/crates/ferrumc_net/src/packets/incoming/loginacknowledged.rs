use log::debug;
use ferrumc_macros::{Decode, packet};
use crate::Connection;
use crate::packets::IncomingPacket;

#[derive(Decode)]
#[packet(packet_id = 0x03, state = "login")]
pub struct LoginAcknowledged;

impl IncomingPacket for LoginAcknowledged {
    async fn handle(&self, conn: &mut tokio::sync::RwLockWriteGuard<'_, Connection>) -> Result<(), Error> {
        debug!("LoginAcknowledged packet received");
        Ok(())
    }
}