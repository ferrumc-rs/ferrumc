use bevy_ecs::prelude::{Component, Entity};
use ferrumc_core::world::dimension::{DimensionTypeData, WorldName};
use std::ops::Deref;
pub mod chunk_storage_component;

/// Component attached to a World Entity.
/// Defines what "kind" of world it is.
#[derive(Component, Debug, Clone, Copy)]
pub struct DimensionType(pub DimensionTypeData);

/// Component attached to a World Entity.
/// Defines the human-readable name (e.g., "lobby").
#[derive(Component, Debug, Clone)]
pub struct Name(pub WorldName);

/// Component attached to a **Player Entity**.
/// Points to the World Entity the player is currently in.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub world_entity: Entity,
}

// Deref to Entity for easy usage
impl Deref for Location {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.world_entity
    }
}
