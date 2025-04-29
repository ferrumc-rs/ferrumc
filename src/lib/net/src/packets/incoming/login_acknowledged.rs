use crate::packets::IncomingPacket;

use crate::errors::NetError;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, Event, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(Debug, NetDecode)]
#[packet(packet_id = "login_acknowledged", state = "login")]
pub struct LoginAcknowledgedPacket {}

#[derive(Event)]
pub struct LoginAcknowledgedEvent {
    pub login_acknowledged_packet: LoginAcknowledgedPacket,
    pub conn_id: usize,
}
impl IncomingPacket for LoginAcknowledgedPacket {
    fn handle(self, conn_id: usize, state: Arc<ServerState>) -> Result<(), NetError> {
        LoginAcknowledgedEvent::trigger(LoginAcknowledgedEvent::new(self, conn_id), state)?;
        Ok(())
    }
}

impl LoginAcknowledgedEvent {
    pub fn new(login_acknowledged_packet: LoginAcknowledgedPacket, conn_id: usize) -> Self {
        Self {
            login_acknowledged_packet,
            conn_id,
        }
    }
}
