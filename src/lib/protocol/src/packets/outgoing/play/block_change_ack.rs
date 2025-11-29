use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_BLOCK_CHANGED_ACK, state = "play")]
pub struct BlockChangeAck {
    pub sequence: VarInt,
}
