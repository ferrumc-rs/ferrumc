use bevy_ecs::prelude::{Entity, Event};

// Fired when a player crosses a chunk boundary. Assumes dimensions are the same
#[derive(Event)]
pub struct CrossChunkBoundaryEvent {
    pub player: Entity,
    pub old_chunk: (i32, i32),
    pub new_chunk: (i32, i32),
}