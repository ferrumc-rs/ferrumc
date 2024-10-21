use crate::packets::IncomingPacket;
use crate::{NetResult, ServerState};
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, Event, NetDecode};
use std::sync::Arc;

#[derive(NetDecode)]
#[packet(packet_id = 0x1A, state = "play")]
pub struct SetPlayerPositionPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub on_ground: bool,
}

impl IncomingPacket for SetPlayerPositionPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let event = SetPlayerPositionEvent::new(self, conn_id);
        tokio::spawn(SetPlayerPositionEvent::trigger(event, state));

        Ok(())
    }
}

#[derive(Event)]
pub struct SetPlayerPositionEvent {
    pub data: SetPlayerPositionPacket,
    pub conn_id: usize,
}

impl SetPlayerPositionEvent {
    pub fn new(data: SetPlayerPositionPacket, conn_id: usize) -> Self {
        Self { data, conn_id }
    }
}
