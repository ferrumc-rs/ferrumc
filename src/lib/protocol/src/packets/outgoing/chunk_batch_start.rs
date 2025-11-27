use ferrumc_macros::{packet, NetEncode};
use ferrumc_protocol::ids;

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_CHUNK_BATCH_START, state = "play")]
pub struct ChunkBatchStart {}
