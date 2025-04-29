use crate::errors::NetError;
use crate::packets::IncomingPacket;

use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode)]
#[packet(packet_id = "accept_teleportation", state = "play")]
pub struct ConfirmPlayerTeleport {
    pub teleport_id: VarInt,
}

impl IncomingPacket for ConfirmPlayerTeleport {
    fn handle(self, conn_id: usize, state: Arc<ServerState>) -> Result<(), NetError> {
        Ok(())
    }
}
