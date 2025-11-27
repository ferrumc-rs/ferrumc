use bevy_ecs::prelude::Component;
use std::sync::atomic::{AtomicI32, Ordering};

/// Global entity ID counter for non-player entities.
///
/// Each spawned entity (pig, cow, arrow, etc.) needs a unique network ID
/// to be identified in packets. This counter generates sequential IDs
/// starting from a high number to avoid collisions with player short_uuid.
static ENTITY_ID_COUNTER: AtomicI32 = AtomicI32::new(1_000_000);

/// Identity component for non-player entities.
///
/// Similar to PlayerIdentity but for mobs and other entities.
/// Contains a unique network ID and UUID for the entity.
///
/// # Examples
///
/// ```ignore
/// use ferrumc_core::identity::EntityIdentity;
///
/// let pig_identity = EntityIdentity::new();
/// assert!(pig_identity.entity_id >= 1_000_000);
/// ```
#[derive(Debug, Component, Clone)]
pub struct EntityIdentity {
    /// Network entity ID used in packets.
    /// Must be unique across all entities in the server.
    pub entity_id: i32,

    /// Unique identifier for this entity.
    /// Generated randomly for each spawned entity.
    pub uuid: uuid::Uuid,
}

impl EntityIdentity {
    /// Creates a new entity identity with a unique ID and UUID.
    ///
    /// The entity_id is generated from an atomic counter to ensure uniqueness.
    /// The UUID is randomly generated.
    pub fn new() -> Self {
        Self {
            entity_id: ENTITY_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            uuid: uuid::Uuid::new_v4(),
        }
    }

    /// Creates an entity identity with a specific UUID (for loading from disk).
    pub fn with_uuid(uuid: uuid::Uuid) -> Self {
        Self {
            entity_id: ENTITY_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            uuid,
        }
    }
}

impl Default for EntityIdentity {
    fn default() -> Self {
        Self::new()
    }
}
