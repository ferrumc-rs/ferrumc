use bevy_ecs::prelude::{Component, Entity};

/// Offset added to Bevy entity indices to generate network entity IDs.
///
/// This prevents collisions with player short_uuid values (which are i32).
/// Bevy entity indices start at 0, so we offset them by 1,000,000 to ensure
/// they don't conflict with any player entity IDs.
const ENTITY_ID_OFFSET: i32 = 1_000_000;

/// Identity component for non-player entities.
///
/// Similar to PlayerIdentity but for mobs and other entities.
/// Contains a unique network ID derived from the Bevy Entity and a UUID.
///
/// The network entity ID is computed from the Bevy Entity index plus an offset,
/// ensuring it's unique and tied to the actual Bevy entity lifecycle.
///
/// # Examples
///
/// ```ignore
/// use ferrumc_core::identity::EntityIdentity;
/// use bevy_ecs::world::World;
///
/// let mut world = World::new();
/// let entity = world.spawn_empty().id();
/// let pig_identity = EntityIdentity::from_entity(entity);
/// assert!(pig_identity.entity_id >= 1_000_000);
/// ```
#[derive(Debug, Component, Clone)]
pub struct EntityIdentity {
    /// Network entity ID used in packets.
    /// Derived from the Bevy Entity index + offset.
    /// Must be unique across all entities in the server.
    pub entity_id: i32,

    /// Unique identifier for this entity.
    /// Generated randomly for each spawned entity.
    pub uuid: uuid::Uuid,
}

impl EntityIdentity {
    /// Creates a new entity identity from a Bevy Entity.
    ///
    /// The entity_id is derived from the Bevy entity's index plus an offset.
    /// This ensures the network ID is stable and tied to the Bevy entity.
    /// The UUID is randomly generated.
    ///
    /// # Arguments
    ///
    /// * `entity` - The Bevy entity this identity belongs to
    pub fn from_entity(entity: Entity) -> Self {
        Self {
            entity_id: (entity.index() as i32).saturating_add(ENTITY_ID_OFFSET),
            uuid: uuid::Uuid::new_v4(),
        }
    }

    /// Creates an entity identity with a specific UUID (for loading from disk).
    ///
    /// # Arguments
    ///
    /// * `entity` - The Bevy entity this identity belongs to
    /// * `uuid` - The UUID to use (e.g., loaded from save data)
    pub fn from_entity_with_uuid(entity: Entity, uuid: uuid::Uuid) -> Self {
        Self {
            entity_id: (entity.index() as i32).saturating_add(ENTITY_ID_OFFSET),
            uuid,
        }
    }
}
