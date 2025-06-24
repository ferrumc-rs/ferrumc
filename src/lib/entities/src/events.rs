use bevy_ecs::prelude::Event;
use ferrumc_core::transform::position::Position;

#[derive(Clone, Event)]
pub struct SpawnZombieEvent {
    pub position: Position,
}
