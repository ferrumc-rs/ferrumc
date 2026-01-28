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
pub struct SkeletonHorseBundle {
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

impl SkeletonHorseBundle {
    pub fn new(position: Position) -> Self {
        let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::SKELETON_HORSE);
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
    fn test_skeleton_horse_bundle_creation() {
        let position = Position::new(0.0, 64.0, 0.0);
        let skeleton_horse = SkeletonHorseBundle::new(position);

        assert_eq!(skeleton_horse.metadata.protocol_id(), 111);
        assert_eq!(skeleton_horse.metadata.resource_name(), "skeleton_horse");
        assert!(skeleton_horse.metadata.is_mob());
    }

    #[test]
    fn test_skeleton_horse_bundle_with_rotation() {
        let position = Position::new(10.0, 70.0, 20.0);
        let rotation = Rotation {
            yaw: 90.0,
            pitch: 0.0,
        };
        let skeleton_horse = SkeletonHorseBundle::with_rotation(position, rotation);
        assert_eq!(skeleton_horse.rotation.yaw, 90.0);
    }
}
