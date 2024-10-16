use std::sync::Arc;
use ferrumc_macros::{Event, NetDecode, packet};
use crate::packets::incoming::login_start::{LoginStartEvent, LoginStartPacket};
use crate::packets::IncomingPacket;
use crate::{NetResult, ServerState};

#[derive(Debug, NetDecode)]
#[packet(packet_id = 0x03, state = "login")]
pub struct LoginAcknowledgedPacket {
}

#[derive(Event)]
pub struct LoginAcknowledgedEvent {
    pub login_acknowledged_packet: LoginAcknowledgedPacket,
    pub conn_id: usize,
}

impl IncomingPacket for LoginAcknowledgedPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        LoginAcknowledgedEvent::trigger(LoginAcknowledgedEvent::new(self, conn_id), state).await?;
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