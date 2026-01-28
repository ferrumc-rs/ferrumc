use bevy_ecs::prelude::Bundle;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::transform::{
    grounded::OnGround, position::Position, rotation::Rotation, velocity::Velocity,
};
use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

use crate::components::{
    CombatProperties, EntityMetadata, LastSyncedPosition, PhysicalProperties, SpawnProperties,
};

/// Complete bundle to spawn a fox in Bevy ECS.
///
/// This bundle contains all the necessary components to represent a fox
/// in the world. It uses Vanilla's data from ferrumc-data to correctly
/// initialize properties.
#[derive(Bundle)]
pub struct FoxBundle {
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

impl FoxBundle {
    pub fn new(position: Position) -> Self {
        let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::FOX);
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
    fn test_fox_bundle_creation() {
        const EPSILON_F32: f32 = 1e-6;
        const EPSILON_F64: f64 = 1e-6;

        let position = Position::new(0.0, 64.0, 0.0);
        let fox = FoxBundle::new(position);

        // Verify vanilla metadata
        assert_eq!(fox.metadata.protocol_id(), 52);
        assert_eq!(fox.metadata.resource_name(), "fox");
        assert!(fox.metadata.is_mob());

        // Verify physical properties (using epsilon for floating point comparison)
        // Fox dimensions: [width=0.6, height=0.7]
        assert!((fox.physical.bounding_box.height() - 0.7).abs() < EPSILON_F64);
        assert!((fox.physical.bounding_box.width() - 0.6).abs() < EPSILON_F64);
        assert!((fox.physical.eye_height - 0.4).abs() < EPSILON_F32);
        assert!(!fox.physical.fire_immune);

        // Verify combat properties
        assert!(fox.combat.attackable);
        assert_eq!(fox.combat.invulnerability_ticks, 0);

        // Verify spawn properties (CREATURE category - persistent)
        assert!(fox.spawn.saveable);
        assert_eq!(fox.spawn.limit_per_chunk, 4);
        assert!(fox.spawn.is_friendly());
        assert!(fox.spawn.is_persistent());
    }

    #[test]
    fn test_fox_bundle_with_rotation() {
        let position = Position::new(10.0, 70.0, 20.0);
        let rotation = Rotation {
            yaw: 90.0,
            pitch: 0.0,
        };
        let fox = FoxBundle::with_rotation(position, rotation);

        assert_eq!(fox.rotation.yaw, 90.0);
        assert_eq!(fox.rotation.pitch, 0.0);
    }
}
