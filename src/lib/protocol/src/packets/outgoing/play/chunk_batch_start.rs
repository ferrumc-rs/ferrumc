use crate::ids;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_CHUNK_BATCH_START, state = "play")]
pub struct ChunkBatchStart {}
