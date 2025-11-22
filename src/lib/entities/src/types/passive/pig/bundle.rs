use crate::components::*;
use crate::types::passive::pig::data::PigData;
use bevy_ecs::prelude::*;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_data::generated::entities::EntityType as EntityTypeData;
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
    pub last_synced_position: LastSyncedPosition,

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
            last_synced_position: crate::components::LastSyncedPosition::from_position(&position),
            position,
            rotation: Rotation::default(),
            velocity: Velocity::zero(),
            health: Health::new(EntityTypeData::PIG.max_health.unwrap()), // Pig max health from vanilla data (10.0)
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
