use crate::errors::NetError;
use crate::packets::IncomingPacket;
use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, Event, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode)]
#[packet(packet_id = "finish_configuration", state = "configuration")]
pub struct AckFinishConfigurationPacket {}

impl IncomingPacket for AckFinishConfigurationPacket {
    fn handle(self, conn_id: usize, state: Arc<ServerState>) -> Result<(), NetError> {
        let event = AckFinishConfigurationEvent::new(self, conn_id);

        AckFinishConfigurationEvent::trigger(event, state)?;

        Ok(())
    }
}

#[derive(Event)]
pub struct AckFinishConfigurationEvent {
    pub packet: AckFinishConfigurationPacket,
    pub conn_id: usize,
}

impl AckFinishConfigurationEvent {
    pub fn new(packet: AckFinishConfigurationPacket, conn_id: usize) -> Self {
        Self { packet, conn_id }
    }
}
