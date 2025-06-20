use ferrumc_macros::{NetEncode, packet};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "chunk_batch_start", state = "play")]
pub struct ChunkBatchStart {}
