use ferrumc_world::chunk_format::Chunk;
use tokio::time::Instant;
use whirlwind::{ShardMap, ShardSet};

pub struct ChunkReceiver {
    pub needed_chunks: ShardMap<(i32, i32, String), Option<Chunk>>,
    pub can_see: ShardSet<(i32, i32, String)>,
    pub last_update: Instant,
    pub last_chunk: Option<(i32, i32, String)>,
}

impl Default for ChunkReceiver {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkReceiver {
    pub fn new() -> Self {
        Self {
            needed_chunks: ShardMap::new(),
            can_see: ShardSet::new(),
            last_update: Instant::now(),
            last_chunk: None,
        }
    }
}
