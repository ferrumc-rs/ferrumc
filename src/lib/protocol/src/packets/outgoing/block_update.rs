use ferrumc_macros::{packet, NetEncode};
use ferrumc_protocol::codec::net_types::network_position::NetworkPosition;
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_BLOCK_UPDATE, state = "play")]
pub struct BlockUpdate {
    pub location: NetworkPosition,
    pub block_state_id: VarInt,
}
