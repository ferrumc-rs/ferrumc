use std::sync::Arc;

use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, Event, NetDecode};
use ferrumc_state::ServerState;

use crate::{packets::IncomingPacket, NetResult};

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x2F, state = "play")]
pub struct IncomingSetHeldItemPacket {
    pub slot: u16,
}

impl IncomingPacket for IncomingSetHeldItemPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let event = ChangeSlotEvent::new(conn_id, self.slot);
        ChangeSlotEvent::trigger(event, state).await?;

        Ok(())
    }
}

#[derive(Event, Debug)]
pub struct ChangeSlotEvent {
    pub conn_id: usize,
    pub slot: u16,
}

impl ChangeSlotEvent {
    pub fn new(conn_id: usize, slot: u16) -> Self {
        Self { conn_id, slot }
    }
}
