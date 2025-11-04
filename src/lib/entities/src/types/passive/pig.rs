use crate::components::*;
use crate::types::passive::pig_data::PigData;
use bevy_ecs::prelude::*;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use uuid::Uuid;

/// Complete bundle for spawning a pig entity
///
/// Includes both generic components (position, health) and pig-specific data (PigData).
/// By default, pigs are marked as Persisted to be saved to the database.
#[derive(Bundle)]
pub struct PigBundle {
    // Generic components (shared across all entities)
    pub entity_type: EntityType,
    pub entity_id: EntityId,
    pub position: Position,
    pub rotation: Rotation,
    pub velocity: Velocity,
    pub health: Health,
    pub age: Age,
    pub on_ground: OnGround,
    pub uuid: EntityUuid,

    // Pig-specific data (implements GameEntity)
    pub pig_data: PigData,

    // Persistence marker (save to database on chunk unload)
    pub persisted: Persisted,
}

/// entity UUID
#[derive(Component)]
pub struct EntityUuid(pub Uuid);

impl PigBundle {
    pub fn new(entity_id: i64, position: Position) -> Self {
        Self {
            // Generic components
            entity_type: EntityType::Pig,
            entity_id: EntityId::new(entity_id),
            position,
            rotation: Rotation::default(),
            velocity: Velocity::zero(),
            health: Health::new(10.0), // Pigs have 10 HP
            age: Age::new(),
            on_ground: OnGround(true), // Spawn on ground to prevent falling before sync
            uuid: EntityUuid(Uuid::new_v4()),

            // Pig-specific data
            pig_data: PigData::default(),

            // Persistence marker
            persisted: Persisted,
        }
    }
}
