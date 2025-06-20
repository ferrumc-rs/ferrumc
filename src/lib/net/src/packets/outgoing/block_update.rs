use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "block_update", state = "play")]
pub struct BlockUpdate {
    pub location: NetworkPosition,
    pub block_id: VarInt,
}
