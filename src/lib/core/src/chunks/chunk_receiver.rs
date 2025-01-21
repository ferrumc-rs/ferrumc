use ferrumc_world::chunk_format::Chunk;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::AtomicBool;
use tokio::time::Instant;

const VIEW_DISTANCE: i32 = 8;
pub struct ChunkReceiver {
    pub needed_chunks: HashMap<(i32, i32, String), Option<Chunk>>,
    pub can_see: HashSet<(i32, i32, String)>,
    pub last_update: Instant,
    pub last_chunk: Option<(i32, i32, String)>,
    pub chunks_per_tick: f32,
    pub has_loaded: AtomicBool,
}

impl Default for ChunkReceiver {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkReceiver {
    pub fn new() -> Self {
        Self {
            needed_chunks: HashMap::new(),
            can_see: HashSet::new(),
            last_update: Instant::now(),
            last_chunk: None,
            chunks_per_tick: 0.0,
            has_loaded: AtomicBool::new(false),
        }
    }
    
    pub fn queue_chunk_resend(&mut self, x: i32, z: i32, dimension: String) {
        self.needed_chunks.insert((x, z, dimension), None);
    }
}

impl ChunkReceiver {
    pub async fn calculate_chunks(&mut self) {
        if let Some(last_chunk) = &self.last_chunk {
            let mut new_can_see = HashSet::new();
            for x in last_chunk.0 - VIEW_DISTANCE..=last_chunk.0 + VIEW_DISTANCE {
                for z in last_chunk.1 - VIEW_DISTANCE..=last_chunk.1 + VIEW_DISTANCE {
                    if !self.can_see.contains(&(x, z, last_chunk.2.clone())) {
                        self.needed_chunks
                            .insert((x, z, last_chunk.2.clone()), None);
                    }
                    new_can_see.insert((x, z, last_chunk.2.clone()));
                }
            }
            self.can_see = new_can_see;
        }
    }
}
