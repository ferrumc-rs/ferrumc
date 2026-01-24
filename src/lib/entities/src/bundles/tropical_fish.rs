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
pub struct TropicalFishBundle {
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

impl TropicalFishBundle {
    pub fn new(position: Position) -> Self {
        let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::TROPICAL_FISH);
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
    fn test_tropical_fish_bundle_creation() {
        let position = Position::new(0.0, 64.0, 0.0);
        let tropical_fish = TropicalFishBundle::new(position);

        assert_eq!(tropical_fish.metadata.protocol_id(), 131);
        assert_eq!(tropical_fish.metadata.resource_name(), "tropical_fish");
        assert!(tropical_fish.metadata.is_mob());
        // WATER_AMBIENT is not persistent
        assert!(!tropical_fish.spawn.is_persistent());
    }

    #[test]
    fn test_tropical_fish_bundle_with_rotation() {
        let position = Position::new(10.0, 70.0, 20.0);
        let rotation = Rotation { yaw: 90.0, pitch: 0.0 };
        let tropical_fish = TropicalFishBundle::with_rotation(position, rotation);
        assert_eq!(tropical_fish.rotation.yaw, 90.0);
    }
}
