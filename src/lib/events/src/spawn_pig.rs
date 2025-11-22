use bevy_ecs::prelude::Event;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::EntityType;

/// Fired when an entity should be spawned.
/// This is triggered by spawn commands after processing the spawn queue.
#[derive(Event)]
pub struct SpawnEntityEvent {
    pub entity_type: EntityType,
    pub position: Position,
}
