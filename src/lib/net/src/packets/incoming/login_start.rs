use crate::packets::IncomingPacket;

use crate::errors::NetError;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, NetDecode)]
#[packet(packet_id = "hello", state = "login")]
pub struct LoginStartPacket {
    pub username: String,
    pub uuid: u128,
}

impl IncomingPacket for LoginStartPacket {
    fn handle(self, conn_id: usize, state: Arc<ServerState>) -> Result<(), NetError> {
        debug!("{} Is logging in with username {}", conn_id, self.username);
        Ok(())
    }
}
