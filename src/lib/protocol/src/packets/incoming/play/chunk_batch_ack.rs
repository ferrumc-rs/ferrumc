use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_CHUNK_BATCH_RECEIVED, state = "play")]
pub struct ChunkBatchAck {
    pub chunks_per_tick: f32,
}
