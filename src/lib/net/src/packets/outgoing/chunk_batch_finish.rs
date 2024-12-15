use ferrumc_macros::{packet, NetEncode};
use std::io::Write;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
#[packet(packet_id = 0x0C)]
pub struct ChunkBatchFinish {
    pub batch_size: VarInt
}
