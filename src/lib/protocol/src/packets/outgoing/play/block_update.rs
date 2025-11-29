use crate::codec::net_types::network_position::NetworkPosition;
use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_BLOCK_UPDATE, state = "play")]
pub struct BlockUpdate {
    pub location: NetworkPosition,
    pub block_state_id: VarInt,
}
