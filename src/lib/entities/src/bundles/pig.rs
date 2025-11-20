use bevy_ecs::prelude::Bundle;
use ferrumc_core::transform::{grounded::OnGround, position::Position, rotation::Rotation};
use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

use crate::components::{CombatProperties, EntityMetadata, PhysicalProperties, SpawnProperties};
use crate::traits::EntityFactory;

/// Complete bundle to spawn a pig in Bevy ECS.
///
/// This bundle contain all the necessary composantsto represent a pig
/// in the world. It use Vanilla's data from ferrumc-data to correctly
/// initialize properties.
///
/// # Examples
///
/// ```ignore
/// use bevy_ecs::prelude::Commands;
/// use ferrumc_entities::bundles::PigBundle;
/// use ferrumc_core::transform::position::Position;
///
/// fn spawn_pig(mut commands: Commands) {
///     let position = Position::new(0.0, 64.0, 0.0);
///     commands.spawn(PigBundle::new(position));
/// }
/// ```
#[derive(Bundle)]
pub struct PigBundle {
    /// Immutable vanilla metadatas (protocol_id, resource_name, etc.)
    pub metadata: EntityMetadata,

    /// Physical properties (bounding box, eye_height, fire_immune)
    pub physical: PhysicalProperties,

    /// Combat properties (attackable, invulnerability)
    pub combat: CombatProperties,

    /// Spawn properties (category, saveable, limits)
    pub spawn: SpawnProperties,

    /// Actual entities position in the world
    pub position: Position,

    /// Actual rotation (yaw, pitch)
    pub rotation: Rotation,

    /// True if the entity is on the ground (needed for physics)
    pub on_ground: OnGround,
}

impl PigBundle {
    /// Create a new bundle for the pig for a gived position.
    ///
    /// Initialize all the components with correct vanilla's values
    /// from ferrumc-data.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ferrumc_entities::bundles::PigBundle;
    /// use ferrumc_core::transform::position::Position;
    ///
    /// let pig = PigBundle::new(Position::new(10.0, 64.0, 20.0));
    /// commands.spawn(pig);
    /// ```
    pub fn new(position: Position) -> Self {
        // Use EntityFactory to create the basics components from vanilla data
        let (metadata, physical, combat, spawn) =
            Self::create_base_components(&VanillaEntityType::PIG);

        Self {
            // Derived components from vanilla
            metadata,
            physical,
            combat,
            spawn,

            // Transformation state
            position,
            rotation: Rotation::default(),
            on_ground: OnGround(true), // Spawn on the ground
        }
    }

    /// Create a pig at the gived position with a custom rotation
    pub fn with_rotation(position: Position, rotation: Rotation) -> Self {
        let mut bundle = Self::new(position);
        bundle.rotation = rotation;
        bundle
    }
}

impl std::fmt::Debug for PigBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PigBundle")
            .field("metadata", &self.metadata)
            .field("physical", &self.physical)
            .field("position", &self.position)
            .field("rotation", &self.rotation)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pig_bundle_creation() {
        let position = Position::new(0.0, 64.0, 0.0);
        let pig = PigBundle::new(position);

        // Verify vanilla metadata
        assert_eq!(pig.metadata.protocol_id(), 95);
        assert_eq!(pig.metadata.resource_name(), "pig");
        assert!(pig.metadata.is_mob());

        // Verify physical properties
        assert_eq!(pig.physical.bounding_box.height, 0.9);
        assert_eq!(pig.physical.bounding_box.half_width, 0.45);
        assert_eq!(pig.physical.eye_height, 0.765);
        assert!(!pig.physical.fire_immune);

        // Verify combat properties
        assert!(pig.combat.attackable);
        assert_eq!(pig.combat.invulnerability_ticks, 0);

        // Verify spawn properties
        assert!(pig.spawn.saveable);
        assert_eq!(pig.spawn.limit_per_chunk, 4);
        assert!(pig.spawn.is_friendly());
        assert!(pig.spawn.is_persistent());
    }

    #[test]
    fn test_pig_bundle_with_rotation() {
        let position = Position::new(10.0, 70.0, 20.0);
        let rotation = Rotation {
            yaw: 90.0,
            pitch: 0.0,
        };
        let pig = PigBundle::with_rotation(position, rotation);

        assert_eq!(pig.rotation.yaw, 90.0);
        assert_eq!(pig.rotation.pitch, 0.0);
    }
}
