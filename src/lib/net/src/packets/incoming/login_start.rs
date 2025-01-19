use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, Event, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(Debug, NetDecode)]
#[packet(packet_id = "hello", state = "login")]
pub struct LoginStartPacket {
    pub username: String,
    pub uuid: u128,
}

impl IncomingPacket for LoginStartPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        LoginStartEvent::trigger(LoginStartEvent::new(self, conn_id), state).await?;
        Ok(())
    }
}

#[derive(Event)]
pub struct LoginStartEvent {
    pub login_start_packet: LoginStartPacket,
    pub conn_id: usize,
}

impl LoginStartEvent {
    pub fn new(login_start_packet: LoginStartPacket, conn_id: usize) -> Self {
        Self {
            login_start_packet,
            conn_id,
        }
    }
}
