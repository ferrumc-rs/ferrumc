use bevy_ecs::prelude::Bundle;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::transform::{
    grounded::OnGround, position::Position, rotation::Rotation, velocity::Velocity,
};
use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

use crate::components::{
    CombatProperties, EntityMetadata, LastSyncedPosition, PhysicalProperties, SpawnProperties,
};

/// Complete bundle to spawn an armadillo in Bevy ECS.
///
/// This bundle contains all the necessary components to represent an armadillo
/// in the world. It uses Vanilla's data from ferrumc-data to correctly
/// initialize properties.
#[derive(Bundle)]
pub struct ArmadilloBundle {
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

impl ArmadilloBundle {
    pub fn new(position: Position) -> Self {
        let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::ARMADILLO);
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
    fn test_armadillo_bundle_creation() {
        const EPSILON_F32: f32 = 1e-6;
        const EPSILON_F64: f64 = 1e-6;

        let position = Position::new(0.0, 64.0, 0.0);
        let armadillo = ArmadilloBundle::new(position);

        // Verify vanilla metadata
        assert_eq!(armadillo.metadata.protocol_id(), 4);
        assert_eq!(armadillo.metadata.resource_name(), "armadillo");
        assert!(armadillo.metadata.is_mob());

        // Verify physical properties (using epsilon for floating point comparison)
        // Armadillo dimensions: [width=0.7, height=0.65]
        assert!((armadillo.physical.bounding_box.height() - 0.65).abs() < EPSILON_F64);
        assert!((armadillo.physical.bounding_box.width() - 0.7).abs() < EPSILON_F64);
        assert!((armadillo.physical.eye_height - 0.26).abs() < EPSILON_F32);
        assert!(!armadillo.physical.fire_immune);

        // Verify combat properties
        assert!(armadillo.combat.attackable);
        assert_eq!(armadillo.combat.invulnerability_ticks, 0);

        // Verify spawn properties
        assert!(armadillo.spawn.saveable);
        assert_eq!(armadillo.spawn.limit_per_chunk, 4);
        assert!(armadillo.spawn.is_friendly());
        assert!(armadillo.spawn.is_persistent());
    }

    #[test]
    fn test_armadillo_bundle_with_rotation() {
        let position = Position::new(10.0, 70.0, 20.0);
        let rotation = Rotation {
            yaw: 90.0,
            pitch: 0.0,
        };
        let armadillo = ArmadilloBundle::with_rotation(position, rotation);

        assert_eq!(armadillo.rotation.yaw, 90.0);
        assert_eq!(armadillo.rotation.pitch, 0.0);
    }
}
