use bevy_ecs::prelude::Component;
use std::collections::{HashSet, VecDeque};
use typename::TypeName;

#[derive(TypeName, Component)]
pub struct ChunkReceiver {
    pub loading: VecDeque<(i32, i32)>,
    pub dirty: VecDeque<(i32, i32)>,
    pub loaded: HashSet<(i32, i32)>,
    pub unloading: VecDeque<(i32, i32)>,
    pub chunks_per_tick: f32,
}

impl Default for ChunkReceiver {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkReceiver {
    pub fn new() -> Self {
        Self {
            loading: VecDeque::new(),
            loaded: HashSet::new(),
            unloading: VecDeque::new(),
            dirty: VecDeque::new(),
            chunks_per_tick: 128.0,
        }
    }
}
