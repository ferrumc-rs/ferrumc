use bevy_ecs::prelude::Bundle;
use ferrumc_core::transform::{grounded::OnGround, position::Position, rotation::Rotation};
use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

use crate::components::{CombatProperties, EntityMetadata, PhysicalProperties, SpawnProperties};

/// Complete bundle to spawn a pig in Bevy ECS.
///
/// This bundle contain all the necessary composantsto represent a pig
/// in the world. It use Vanilla's data from ferrumc-data to correctly
/// initialize properties.
///
/// Note: The EntityIdentity component must be added separately after spawning
/// since it needs the Bevy Entity ID.
///
/// # Examples
///
/// ```ignore
/// use bevy_ecs::prelude::Commands;
/// use ferrumc_entities::bundles::PigBundle;
/// use ferrumc_core::{identity::entity_identity::EntityIdentity, transform::position::Position};
///
/// fn spawn_pig(mut commands: Commands) {
///     let position = Position::new(0.0, 64.0, 0.0);
///     let entity = commands.spawn(PigBundle::new(position)).id();
///     commands.entity(entity).insert(EntityIdentity::from_entity(entity));
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
    /// Note: Don't forget to add EntityIdentity after spawning!
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ferrumc_entities::bundles::PigBundle;
    /// use ferrumc_core::{identity::entity_identity::EntityIdentity, transform::position::Position};
    ///
    /// let entity = commands.spawn(PigBundle::new(Position::new(10.0, 64.0, 20.0))).id();
    /// commands.entity(entity).insert(EntityIdentity::from_entity(entity));
    /// ```
    pub fn new(position: Position) -> Self {
        // Create metadata from vanilla data
        let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::PIG);

        // Create other components from metadata
        let physical = PhysicalProperties::from_metadata(&metadata);
        let combat = CombatProperties::from_metadata(&metadata);
        let spawn = SpawnProperties::from_metadata(&metadata);

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
        const EPSILON_F32: f32 = 1e-6;
        const EPSILON_F64: f64 = 1e-6;

        let position = Position::new(0.0, 64.0, 0.0);
        let pig = PigBundle::new(position);

        // Verify vanilla metadata
        assert_eq!(pig.metadata.protocol_id(), 95);
        assert_eq!(pig.metadata.resource_name(), "pig");
        assert!(pig.metadata.is_mob());

        // Verify physical properties (using epsilon for floating point comparison)
        assert!((pig.physical.bounding_box.height - 0.9).abs() < EPSILON_F64);
        assert!((pig.physical.bounding_box.half_width - 0.45).abs() < EPSILON_F64);
        assert!((pig.physical.eye_height - 0.765).abs() < EPSILON_F32);
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
