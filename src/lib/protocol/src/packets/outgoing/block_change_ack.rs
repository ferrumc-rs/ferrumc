use ferrumc_macros::{packet, NetEncode};
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_BLOCK_CHANGED_ACK, state = "play")]
pub struct BlockChangeAck {
    pub sequence: VarInt,
}
