#![allow(dead_code)]

use std::sync::atomic::{AtomicUsize, Ordering};


/// Entity is a unique identifier for an entity in the ECS.
/// It is a simple usize.
/// Always incremented.
pub type Entity = usize;

pub struct EntityManager {
    new_entity_id: AtomicUsize,
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