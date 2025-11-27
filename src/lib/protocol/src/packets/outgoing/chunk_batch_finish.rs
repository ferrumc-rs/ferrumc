use ferrumc_macros::{packet, NetEncode};
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_CHUNK_BATCH_FINISHED, state = "play")]
pub struct ChunkBatchFinish {
    pub batch_size: VarInt,
}
