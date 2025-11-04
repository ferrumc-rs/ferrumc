use bevy_ecs::prelude::{Commands, Component};
use ferrumc_core::transform::position::Position;
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
    pub fn protocol_id(&self) -> i32 {
        use ferrumc_registry::lookup;
        use simd_json::prelude::ValueAsScalar;

        let path = match self {
            EntityType::Pig => "minecraft:entity_type/entries/minecraft:pig/protocol_id",
            EntityType::Cow => "minecraft:entity_type/entries/minecraft:cow/protocol_id",
            EntityType::Sheep => "minecraft:entity_type/entries/minecraft:sheep/protocol_id",
            EntityType::Chicken => "minecraft:entity_type/entries/minecraft:chicken/protocol_id",
            EntityType::Zombie => "minecraft:entity_type/entries/minecraft:zombie/protocol_id",
            EntityType::Spider => "minecraft:entity_type/entries/minecraft:spider/protocol_id",
            EntityType::Creeper => "minecraft:entity_type/entries/minecraft:creeper/protocol_id",
            EntityType::Skeleton => "minecraft:entity_type/entries/minecraft:skeleton/protocol_id",
            EntityType::ItemEntity => "minecraft:entity_type/entries/minecraft:item/protocol_id",
            EntityType::ExperienceOrb => {
                "minecraft:entity_type/entries/minecraft:experience_orb/protocol_id"
            }
        };

        lookup(path)
            .and_then(|v| v.as_i64())
            .expect("Entity type not found in the registry") as i32
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
    pub fn spawn(&self, commands: &mut Commands, entity_id: i64, position: &Position) {
        use crate::components::SyncedToPlayers;
        use crate::types::passive::pig::PigBundle;

        match self {
            EntityType::Pig => {
                let pig =
                    PigBundle::new(entity_id, Position::new(position.x, position.y, position.z));
                commands.spawn((pig, SyncedToPlayers::default()));
            }
            _ => {
                tracing::warn!("Entity type {:?} not yet implemented for spawning", self);
            }
        }
    }
}
