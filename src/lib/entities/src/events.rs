use bevy_ecs::prelude::Event;
use ferrumc_core::transform::position::Position;
use ferrumc_core::entities::entity_kind::EntityKind;

#[derive(Clone, Event)]
pub struct SpawnEntityEvent {
    pub entity_kind: EntityKind,
    pub position: Position,
}
