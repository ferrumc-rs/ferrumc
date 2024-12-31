use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, Event, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x0F, state = "play")]
pub struct IncomingCloseContainerPacket {
    pub window_id: u8,
}

impl IncomingCloseContainerPacket {
    pub fn new(window_id: u8) -> Self {
        IncomingCloseContainerPacket { window_id }
    }
}

impl IncomingPacket for IncomingCloseContainerPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let event = InventoryCloseEvent::new(conn_id, self.window_id);
        InventoryCloseEvent::trigger(event, state).await?;
        Ok(())
    }
}

#[derive(Event, Debug)]
pub struct InventoryCloseEvent {
    pub conn_id: usize,
    pub window_id: u8,
}

impl InventoryCloseEvent {
    pub fn new(conn_id: usize, window_id: u8) -> Self {
        InventoryCloseEvent { conn_id, window_id }
    }
}
