#![allow(dead_code)]

use crate::components::{ComponentManager, ComponentStorage};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::components::storage::Component;

/// Entity is a unique identifier for an entity in the ECS.
/// It is a simple usize.
/// Always incremented.
pub type Entity = usize;

pub struct EntityManager {
    new_entity_id: AtomicUsize,
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            new_entity_id: AtomicUsize::new(0),
        }
    }

    pub fn create_entity(&self) -> Entity {
        let id = self.new_entity_id.load(Ordering::Relaxed);
        self.new_entity_id.fetch_add(1, Ordering::Relaxed);
        id as Entity
    }
    
    pub fn builder<'a>(&'a self, component_storage: &'a ComponentManager) -> EntityBuilder<'a> {
        EntityBuilder::new(self.create_entity(), component_storage)
    }
}

pub struct EntityBuilder<'a> {
    entity: Entity,
    component_storage: &'a ComponentManager,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(entity: Entity, component_storage: &'a ComponentManager) -> Self {
        EntityBuilder { entity, component_storage }
    }

    pub fn with<T: Component>(self, component: T) -> Self {
        self.component_storage.insert(self.entity, component);
        self
    }

    pub fn build(self) -> Entity {
        self.entity
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_entity() {
        let manager = EntityManager::new();
        let entity = manager.create_entity();
        assert_eq!(entity, 0);
    }

    #[test]
    fn test_create_multiple_entities() {
        let manager = EntityManager::new();
        let entity1 = manager.create_entity();
        let entity2 = manager.create_entity();
        assert_eq!(entity1, 0);
        assert_eq!(entity2, 1);
    }
}
