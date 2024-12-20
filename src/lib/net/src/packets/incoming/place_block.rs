
use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode)]
#[packet(packet_id = 0x3C, state = "play")]
pub struct PlaceBlock {
}

impl IncomingPacket for PlaceBlock {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        todo!()
    }
}
