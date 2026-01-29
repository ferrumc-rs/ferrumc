// Entity bundles for spawning in Bevy ECS
// Organized by behavior category

pub mod hostile;
pub mod neutral;
pub mod passive;

// Re-export all bundles for convenience
pub use hostile::*;
pub use neutral::*;
pub use passive::*;

/// Macro to define an entity bundle with all standard components.
///
/// This macro generates a bundle struct with identity, metadata, physical properties,
/// combat properties, spawn properties, position, rotation, velocity, ground state,
/// and last synced position components.
///
/// # Usage
/// ```ignore
/// define_entity_bundle!(PigBundle, PIG);
/// ```
///
/// This will create a `PigBundle` struct that uses `VanillaEntityType::PIG` for metadata.
#[macro_export]
macro_rules! define_entity_bundle {
    ($bundle_name:ident, $vanilla_type:ident) => {
        use bevy_ecs::prelude::Bundle;
        use ferrumc_core::identity::entity_identity::EntityIdentity;
        use ferrumc_core::transform::{
            grounded::OnGround, position::Position, rotation::Rotation, velocity::Velocity,
        };
        use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

        use crate::components::{
            CombatProperties, EntityMetadata, LastSyncedPosition, PhysicalProperties,
            SpawnProperties,
        };

        #[derive(Bundle)]
        pub struct $bundle_name {
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

        impl $bundle_name {
            pub fn new(position: Position) -> Self {
                let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::$vanilla_type);
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
    };
}
