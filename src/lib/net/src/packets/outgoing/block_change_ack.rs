use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "block_changed_ack", state = "play")]
pub struct BlockChangeAck {
    pub sequence: VarInt,
}
