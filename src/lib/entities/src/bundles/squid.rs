use bevy_ecs::prelude::Bundle;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::transform::{
    grounded::OnGround, position::Position, rotation::Rotation, velocity::Velocity,
};
use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

use crate::components::{
    CombatProperties, EntityMetadata, LastSyncedPosition, PhysicalProperties, SpawnProperties,
};

#[derive(Bundle)]
pub struct SquidBundle {
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

impl SquidBundle {
    pub fn new(position: Position) -> Self {
        let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::SQUID);
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
    fn test_squid_bundle_creation() {
        let position = Position::new(0.0, 64.0, 0.0);
        let squid = SquidBundle::new(position);

        assert_eq!(squid.metadata.protocol_id(), 122);
        assert_eq!(squid.metadata.resource_name(), "squid");
        assert!(squid.metadata.is_mob());
        // WATER_CREATURE is persistent
        assert!(squid.spawn.is_persistent());
    }

    #[test]
    fn test_squid_bundle_with_rotation() {
        let position = Position::new(10.0, 70.0, 20.0);
        let rotation = Rotation { yaw: 90.0, pitch: 0.0 };
        let squid = SquidBundle::with_rotation(position, rotation);
        assert_eq!(squid.rotation.yaw, 90.0);
    }
}
