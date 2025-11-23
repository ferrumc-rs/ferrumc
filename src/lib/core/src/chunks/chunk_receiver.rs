use bevy_ecs::prelude::Component;
use std::collections::HashSet;
use std::sync::atomic::AtomicBool;
use typename::TypeName;

pub const VIEW_DISTANCE: i32 = 8;

#[derive(TypeName, Component)]
pub struct ChunkReceiver {
    pub loading: Vec<(i32, i32)>,
    pub loaded: HashSet<(i32, i32)>,
    pub unloading: HashSet<(i32, i32)>,
    pub dirty: Vec<(i32, i32)>,
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
            loading: Vec::new(),
            loaded: HashSet::new(),
            unloading: HashSet::new(),
            dirty: Vec::new(),
            chunks_per_tick: 64.0,
        }
    }
}
