use bevy_ecs::prelude::{Entity, Resource};
use std::collections::HashMap;

/// Fast O(1) lookup index for entities by their network ID
///
/// This avoids O(n) linear scans when looking up entities from packets.
/// Maintained by entity spawn/death systems.
#[derive(Resource, Default)]
pub struct EntityNetworkIdIndex {
    map: HashMap<i32, Entity>,
}

impl EntityNetworkIdIndex {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Register an entity with its network ID
    pub fn insert(&mut self, network_id: i32, entity: Entity) {
        self.map.insert(network_id, entity);
    }

    /// Lookup an entity by its network ID
    pub fn get(&self, network_id: i32) -> Option<Entity> {
        self.map.get(&network_id).copied()
    }

    /// Remove an entity from the index (called on despawn)
    pub fn remove(&mut self, network_id: i32) {
        self.map.remove(&network_id);
    }

    /// Get the number of entities in the index
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if the index is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
