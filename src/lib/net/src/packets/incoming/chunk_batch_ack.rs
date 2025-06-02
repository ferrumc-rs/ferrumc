use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode)]
#[packet(packet_id = "chunk_batch_received", state = "play")]
pub struct ChunkBatchAck {
    pub chunks_per_tick: f32,
}
