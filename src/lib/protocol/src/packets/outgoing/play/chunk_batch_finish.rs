use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_CHUNK_BATCH_FINISHED, state = "play")]
pub struct ChunkBatchFinish {
    pub batch_size: VarInt,
}
