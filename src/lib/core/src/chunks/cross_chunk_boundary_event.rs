use bevy_ecs::prelude::{Entity, Event};
use bevy_math::IVec2;

// Fired when a player crosses a chunk boundary. Assumes dimensions are the same
#[derive(Event)]
pub struct CrossChunkBoundaryEvent {
    pub player: Entity,
    pub old_chunk: IVec2,
    pub new_chunk: IVec2,
}
