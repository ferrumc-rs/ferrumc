use std::sync::Arc;

use ferrumc_events::infrastructure::Event;
use ferrumc_macros::{packet, Event, NetDecode};
use ferrumc_state::ServerState;

use crate::{
    packets::IncomingPacket,
    NetResult,
};

#[derive(NetDecode, Debug, Clone)]
#[packet(packet_id = 0x04, state = "play")]
pub struct ChatCommandPacket {
    command: String,
}

#[derive(Event)]
pub struct CommandDispatchEvent {
    pub command: String,
    pub conn_id: usize,
}

impl CommandDispatchEvent {
    pub fn new(command: String, conn_id: usize) -> Self {
        Self { command, conn_id }
    }
}

impl IncomingPacket for ChatCommandPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        CommandDispatchEvent::trigger(CommandDispatchEvent::new(self.command, conn_id), state).await
    }
}
