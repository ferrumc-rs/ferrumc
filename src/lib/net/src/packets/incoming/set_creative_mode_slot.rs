use std::sync::Arc;

use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, Event, NetDecode};
use ferrumc_state::ServerState;

use crate::{
    packets::{outgoing::set_container_slot::NetworkSlot, IncomingPacket},
    NetResult,
};

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x32, state = "play")]
pub struct SetCreativeModeSlotPacket {
    pub slot: i16,
    pub clicked_item: NetworkSlot,
}

impl IncomingPacket for SetCreativeModeSlotPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let event = SetCreativeModeSlotEvent::new(conn_id, self.slot, self.clicked_item);
        SetCreativeModeSlotEvent::trigger(event, state).await?;
        Ok(())
    }
}

#[derive(Event, Debug)]
pub struct SetCreativeModeSlotEvent {
    pub conn_id: usize,
    pub slot: i16,
    pub clicked_item: NetworkSlot,
}

impl SetCreativeModeSlotEvent {
    pub fn new(conn_id: usize, slot: i16, clicked_item: NetworkSlot) -> Self {
        Self {
            conn_id,
            slot,
            clicked_item,
        }
    }
}
