use crate::components::EntityType;
use bevy_ecs::prelude::{Entity, Event};
use ferrumc_core::transform::position::Position;

/// Event for asking entity spawn
#[derive(Event)]
pub struct SpawnEntityEvent {
    pub entity_type: EntityType,
    pub position: Position,
}

/// Event for dealing damage to an entity
#[derive(Event, Debug, Clone)]
pub struct DamageEvent {
    /// The entity receiving the damage
    pub target: Entity,
    /// The entity dealing the damage (if any)
    pub attacker: Option<Entity>,
    /// Amount of damage to deal
    pub damage: f32,
    /// Knockback direction (normalized vector)
    pub knockback_direction: Option<(f64, f64, f64)>,
    /// Knockback strength multiplier
    pub knockback_strength: f64,
}
