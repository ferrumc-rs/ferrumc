use bevy_ecs::prelude::{Entity, Message};

// Fired when a player crosses a chunk boundary. Assumes dimensions are the same
#[derive(Message)]
pub struct ChunkBoundaryCrossed {
    pub player: Entity,
    pub old_chunk: (i32, i32),
    pub new_chunk: (i32, i32),
}
