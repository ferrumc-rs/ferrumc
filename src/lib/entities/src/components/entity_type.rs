use bevy_ecs::prelude::{Commands, Component, Entity};
use ferrumc_core::transform::position::Position;
use ferrumc_macros::get_registry_entry;
use typename::TypeName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, TypeName)]
pub enum EntityType {
    // Passive mobs
    Pig,
    Cow,
    Sheep,
    Chicken,

    // Hostiles mobs
    Zombie,
    Spider,
    Creeper,
    Skeleton,

    // Other
    ItemEntity, // Dropped item
    ExperienceOrb,
}

impl EntityType {
    /// Returns the protocol ID for this entity type.
    /// Uses compile-time registry lookups for zero-cost abstraction.
    pub fn protocol_id(&self) -> i32 {
        match self {
            EntityType::Pig => {
                get_registry_entry!("minecraft:entity_type.entries.minecraft:pig") as i32
            }
            EntityType::Cow => {
                get_registry_entry!("minecraft:entity_type.entries.minecraft:cow") as i32
            }
            EntityType::Sheep => {
                get_registry_entry!("minecraft:entity_type.entries.minecraft:sheep") as i32
            }
            EntityType::Chicken => {
                get_registry_entry!("minecraft:entity_type.entries.minecraft:chicken") as i32
            }
            EntityType::Zombie => {
                get_registry_entry!("minecraft:entity_type.entries.minecraft:zombie") as i32
            }
            EntityType::Spider => {
                get_registry_entry!("minecraft:entity_type.entries.minecraft:spider") as i32
            }
            EntityType::Creeper => {
                get_registry_entry!("minecraft:entity_type.entries.minecraft:creeper") as i32
            }
            EntityType::Skeleton => {
                get_registry_entry!("minecraft:entity_type.entries.minecraft:skeleton") as i32
            }
            EntityType::ItemEntity => {
                get_registry_entry!("minecraft:entity_type.entries.minecraft:item") as i32
            }
            EntityType::ExperienceOrb => {
                get_registry_entry!("minecraft:entity_type.entries.minecraft:experience_orb") as i32
            }
        }
    }

    pub fn is_hostile(&self) -> bool {
        matches!(
            self,
            EntityType::Zombie | EntityType::Spider | EntityType::Creeper | EntityType::Skeleton
        )
    }

    pub fn is_passive(&self) -> bool {
        matches!(
            self,
            EntityType::Pig | EntityType::Cow | EntityType::Sheep | EntityType::Chicken
        )
    }

    /// Spawns this entity type with the given ID and position
    /// Spawn an entity and return its ECS Entity ID for lookup indexing
    pub fn spawn(
        &self,
        commands: &mut Commands,
        entity_id: i64,
        position: &Position,
    ) -> Option<Entity> {
        use crate::components::SyncedToPlayers;
        use crate::types::passive::pig::PigBundle;

        match self {
            EntityType::Pig => {
                let pig =
                    PigBundle::new(entity_id, Position::new(position.x, position.y, position.z));
                let entity = commands.spawn((pig, SyncedToPlayers::default())).id();
                Some(entity)
            }
            _ => {
                tracing::warn!("Entity type {:?} not yet implemented for spawning", self);
                None
            }
        }
    }
}
