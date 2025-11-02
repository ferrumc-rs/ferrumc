use crate::components::EntityType;
use bevy_ecs::prelude::Event;
use ferrumc_core::transform::position::Position;

/// Event for asking entity spawn
#[derive(Event)]
pub struct SpawnEntityEvent {
    pub entity_type: EntityType,
    pub position: Position,
}
