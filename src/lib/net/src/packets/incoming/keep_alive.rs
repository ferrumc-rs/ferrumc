use crate::packets::outgoing::keep_alive::OutgoingKeepAlivePacket;
use crate::packets::IncomingPacket;

use crate::errors::NetError;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;
use tracing::debug;
use typename::TypeName;

#[derive(TypeName, NetDecode)]
#[packet(packet_id = "keep_alive", state = "play")]
pub struct IncomingKeepAlivePacket {
    pub timestamp: i64,
}
