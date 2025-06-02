use std::sync::Arc;

use bevy_ecs::prelude::Event;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_state::ServerState;

use crate::{errors::NetError, packets::IncomingPacket, NetResult};

#[derive(NetDecode, Debug, Clone)]
#[packet(packet_id = "chat_command", state = "play")]
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

