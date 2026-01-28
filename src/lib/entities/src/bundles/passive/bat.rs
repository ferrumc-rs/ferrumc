use bevy_ecs::prelude::Bundle;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::transform::{
    grounded::OnGround, position::Position, rotation::Rotation, velocity::Velocity,
};
use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

use crate::components::{
    CombatProperties, EntityMetadata, LastSyncedPosition, PhysicalProperties, SpawnProperties,
};

/// Complete bundle to spawn a bat in Bevy ECS.
///
/// This bundle contains all the necessary components to represent a bat
/// in the world. It uses Vanilla's data from ferrumc-data to correctly
/// initialize properties.
#[derive(Bundle)]
pub struct BatBundle {
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

impl BatBundle {
    pub fn new(position: Position) -> Self {
        let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::BAT);
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
    fn test_bat_bundle_creation() {
        const EPSILON_F32: f32 = 1e-6;
        const EPSILON_F64: f64 = 1e-6;

        let position = Position::new(0.0, 64.0, 0.0);
        let bat = BatBundle::new(position);

        // Verify vanilla metadata
        assert_eq!(bat.metadata.protocol_id(), 10);
        assert_eq!(bat.metadata.resource_name(), "bat");
        assert!(bat.metadata.is_mob());

        // Verify physical properties (using epsilon for floating point comparison)
        // Bat dimensions: [width=0.5, height=0.9]
        assert!((bat.physical.bounding_box.height() - 0.9).abs() < EPSILON_F64);
        assert!((bat.physical.bounding_box.width() - 0.5).abs() < EPSILON_F64);
        assert!((bat.physical.eye_height - 0.45).abs() < EPSILON_F32);
        assert!(!bat.physical.fire_immune);

        // Verify combat properties
        assert!(bat.combat.attackable);
        assert_eq!(bat.combat.invulnerability_ticks, 0);

        // Verify spawn properties
        assert!(bat.spawn.saveable);
        assert_eq!(bat.spawn.limit_per_chunk, 4);
        assert!(bat.spawn.is_friendly());
        assert!(!bat.spawn.is_persistent());
    }

    #[test]
    fn test_bat_bundle_with_rotation() {
        let position = Position::new(10.0, 70.0, 20.0);
        let rotation = Rotation {
            yaw: 90.0,
            pitch: 0.0,
        };
        let bat = BatBundle::with_rotation(position, rotation);

        assert_eq!(bat.rotation.yaw, 90.0);
        assert_eq!(bat.rotation.pitch, 0.0);
    }
}
