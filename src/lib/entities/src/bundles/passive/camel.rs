use bevy_ecs::prelude::Bundle;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::transform::{
    grounded::OnGround, position::Position, rotation::Rotation, velocity::Velocity,
};
use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

use crate::components::{
    CombatProperties, EntityMetadata, LastSyncedPosition, PhysicalProperties, SpawnProperties,
};

/// Complete bundle to spawn a camel in Bevy ECS.
///
/// This bundle contains all the necessary components to represent a camel
/// in the world. It uses Vanilla's data from ferrumc-data to correctly
/// initialize properties.
#[derive(Bundle)]
pub struct CamelBundle {
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

impl CamelBundle {
    pub fn new(position: Position) -> Self {
        let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::CAMEL);
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
    fn test_camel_bundle_creation() {
        const EPSILON_F32: f32 = 1e-6;
        const EPSILON_F64: f64 = 1e-6;

        let position = Position::new(0.0, 64.0, 0.0);
        let camel = CamelBundle::new(position);

        // Verify vanilla metadata
        assert_eq!(camel.metadata.protocol_id(), 19);
        assert_eq!(camel.metadata.resource_name(), "camel");
        assert!(camel.metadata.is_mob());

        // Verify physical properties (using epsilon for floating point comparison)
        // Camel dimensions: [width=1.7, height=2.375]
        assert!((camel.physical.bounding_box.height() - 2.375).abs() < EPSILON_F64);
        assert!((camel.physical.bounding_box.width() - 1.7).abs() < EPSILON_F64);
        assert!((camel.physical.eye_height - 2.275).abs() < EPSILON_F32);
        assert!(!camel.physical.fire_immune);

        // Verify combat properties
        assert!(camel.combat.attackable);
        assert_eq!(camel.combat.invulnerability_ticks, 0);

        // Verify spawn properties
        assert!(camel.spawn.saveable);
        assert_eq!(camel.spawn.limit_per_chunk, 6);
        assert!(camel.spawn.is_friendly());
        assert!(camel.spawn.is_persistent());
    }

    #[test]
    fn test_camel_bundle_with_rotation() {
        let position = Position::new(10.0, 70.0, 20.0);
        let rotation = Rotation {
            yaw: 90.0,
            pitch: 0.0,
        };
        let camel = CamelBundle::with_rotation(position, rotation);

        assert_eq!(camel.rotation.yaw, 90.0);
        assert_eq!(camel.rotation.pitch, 0.0);
    }
}
