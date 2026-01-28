use bevy_ecs::prelude::Bundle;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::transform::{
    grounded::OnGround, position::Position, rotation::Rotation, velocity::Velocity,
};
use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

use crate::components::{
    CombatProperties, EntityMetadata, LastSyncedPosition, PhysicalProperties, SpawnProperties,
};

/// Complete bundle to spawn a bee in Bevy ECS.
///
/// This bundle contains all the necessary components to represent a bee
/// in the world. It uses Vanilla's data from ferrumc-data to correctly
/// initialize properties.
#[derive(Bundle)]
pub struct BeeBundle {
    pub identity: EntityIdentity,
    pub metadata: EntityMetadata,
    pub physical: PhysicalProperties,
    pub combat: CombatProperties,
    pub spawn: SpawnProperties,
    pub position: Position,
    pub rotation: Rotation,
    pub velocity: Velocity,
    pub on_ground: OnGround,
    pub last_synced_position: LastSyncedPosition,
}

impl BeeBundle {
    pub fn new(position: Position) -> Self {
        let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::BEE);
        let physical = PhysicalProperties::from_metadata(&metadata);
        let combat = CombatProperties::from_metadata(&metadata);
        let spawn = SpawnProperties::from_metadata(&metadata);

        Self {
            identity: EntityIdentity::new(),
            metadata,
            physical,
            combat,
            spawn,
            rotation: Rotation::default(),
            velocity: Velocity::zero(),
            on_ground: OnGround(false),
            last_synced_position: LastSyncedPosition::from_position(&position),
            position,
        }
    }

    pub fn with_rotation(position: Position, rotation: Rotation) -> Self {
        let mut bundle = Self::new(position);
        bundle.rotation = rotation;
        bundle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bee_bundle_creation() {
        const EPSILON_F32: f32 = 1e-6;
        const EPSILON_F64: f64 = 1e-6;

        let position = Position::new(0.0, 64.0, 0.0);
        let bee = BeeBundle::new(position);

        // Verify vanilla metadata
        assert_eq!(bee.metadata.protocol_id(), 11);
        assert_eq!(bee.metadata.resource_name(), "bee");
        assert!(bee.metadata.is_mob());

        // Verify physical properties (using epsilon for floating point comparison)
        // Bee dimensions: [width=0.7, height=0.6]
        assert!((bee.physical.bounding_box.height() - 0.6).abs() < EPSILON_F64);
        assert!((bee.physical.bounding_box.width() - 0.7).abs() < EPSILON_F64);
        assert!((bee.physical.eye_height - 0.3).abs() < EPSILON_F32);
        assert!(!bee.physical.fire_immune);

        // Verify combat properties
        assert!(bee.combat.attackable);
        assert_eq!(bee.combat.invulnerability_ticks, 0);

        // Verify spawn properties
        assert!(bee.spawn.saveable);
        assert_eq!(bee.spawn.limit_per_chunk, 4);
        assert!(bee.spawn.is_friendly());
        assert!(bee.spawn.is_persistent());
    }

    #[test]
    fn test_bee_bundle_with_rotation() {
        let position = Position::new(10.0, 70.0, 20.0);
        let rotation = Rotation {
            yaw: 90.0,
            pitch: 0.0,
        };
        let bee = BeeBundle::with_rotation(position, rotation);

        assert_eq!(bee.rotation.yaw, 90.0);
        assert_eq!(bee.rotation.pitch, 0.0);
    }
}
