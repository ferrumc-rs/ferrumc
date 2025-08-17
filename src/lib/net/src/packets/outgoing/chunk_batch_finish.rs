use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
#[packet(packet_id = "chunk_batch_finished", state = "play")]
pub struct ChunkBatchFinish {
    pub batch_size: VarInt,
}
