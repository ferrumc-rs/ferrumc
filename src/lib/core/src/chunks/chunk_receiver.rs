use ferrumc_world::chunk_format::Chunk;
use std::collections::HashSet;
use std::sync::atomic::AtomicBool;
use typename::TypeName;

pub const VIEW_DISTANCE: i32 = 8;

#[derive(TypeName)]
pub struct ChunkReceiver {
    pub needs_reload: HashSet<(i32, i32, String)>,
    pub seen: HashSet<(i32, i32, String)>,
    pub last_chunk: (i32, i32, String),
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
            needs_reload: HashSet::new(),
            seen: HashSet::new(),
            last_chunk: (0, 0, "overworld".to_string()),
            chunks_per_tick: 0.0,
            has_loaded: AtomicBool::new(false),
        }
    }
}
