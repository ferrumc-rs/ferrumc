use std::sync::Arc;

use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use bevy_ecs::prelude::Event;

use crate::{errors::NetError, packets::IncomingPacket};

#[derive(NetDecode, Debug, Clone)]
#[packet(packet_id = "chat", state = "play")]
pub struct ChatMessagePacket {
    pub message: String,
    pub timestamp: u64,
    pub salt: u64,
    pub has_signature: bool,
    pub signature: Option<Vec<u64>>,
    pub message_count: VarInt,
    pub acknowledged: Vec<u8>,
}

#[derive(Debug, Event, Clone)]
pub struct ChatMessageEvent {
    pub player_conn_id: usize,
    pub message: String,
}

impl ChatMessageEvent {
    pub fn new(player_conn_id: usize, message: String) -> Self {
        Self {
            player_conn_id,
            message,
        }
    }
}
