use crate::packets::IncomingPacket;

use crate::errors::NetError;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(Debug, NetDecode)]
#[packet(packet_id = "login_acknowledged", state = "login")]
pub struct LoginAcknowledgedPacket {}
impl IncomingPacket for LoginAcknowledgedPacket {
    fn handle(self, conn_id: usize, state: Arc<ServerState>) -> Result<(), NetError> {
        Ok(())
    }
}
