use bevy_ecs::prelude::Component;
use ferrumc_macros::get_registry_entry;

#[derive(Component, Debug, Clone, Copy)]
pub struct EntityKind {
    /// The id of the entity kind. (Found in the registry under minecraft:entity_type)
    r#type: u64
}

impl EntityKind {
    /// Creates a new `EntityKind` with the given type id.
    pub fn new(r#type: u64) -> Self {
        Self { r#type }
    }

    /// Returns the type id of the entity kind.
    pub fn get_id(&self) -> u64 {
        self.r#type
    }
}

impl From<u64> for EntityKind {
    fn from(r#type: u64) -> Self {
        Self::new(r#type)
    }
}
