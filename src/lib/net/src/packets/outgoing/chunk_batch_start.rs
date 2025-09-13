use ferrumc_macros::{packet, NetEncode};

#[derive(NetEncode)]
#[packet(packet_id = "chunk_batch_start", state = "play")]
pub struct ChunkBatchStart {}
