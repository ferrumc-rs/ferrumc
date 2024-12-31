use crate::packets::IncomingPacket;
use crate::NetResult;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::ServerState;
use std::sync::Arc;
use tracing::debug;

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x38, state = "play")]
pub struct PlaceBlock {
    pub hand: VarInt,
    pub position: NetworkPosition,
    pub face: VarInt,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub cursor_z: f32,
    pub inside_block: bool,
    pub sequence: VarInt,
}

impl IncomingPacket for PlaceBlock {
    async fn handle(self, _conn_id: usize, _state: Arc<ServerState>) -> NetResult<()> {
        debug!("{:?}", self);
        Ok(())
    }
}
