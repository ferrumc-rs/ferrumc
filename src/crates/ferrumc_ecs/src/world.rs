use std::fmt::Display;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::components::{Component, ComponentStorage};
use crate::error;
use crate::error::DeallocationErrorType;

#[derive(Debug, PartialEq)]
pub struct Entity {
    id: u64,
    generation: u64,
}

impl Into<usize> for &Entity {
    fn into(self) -> usize {
        self.id as usize
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Entity {
    pub fn new(id: u64, generation: u64) -> Self {
        Entity { id, generation }
    }

    /// Returns the id of the entity.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns the generation of the entity.
    pub fn generation(&self) -> u64 {
        self.generation
    }
}

pub struct EntityAllocator {
    next_id: AtomicU64,
    // The generation of each entity, indexed by the entity id
    generations: Vec<u64>,
    free_ids: Vec<u64>,
}

impl EntityAllocator {
    pub fn new() -> Self {
        EntityAllocator {
            next_id: AtomicU64::new(0),
            generations: Vec::new(),
            free_ids: Vec::new(),
        }
    }

    /// Allocates a new entity.
    /// Returns a builder that can be used to add components to the entity.
    pub fn allocate<'a>(&mut self, component_storage: &'a mut ComponentStorage) -> EntityBuilder<'a> {
        let entity = self.allocate_entity();

        EntityBuilder {
            entity,
            component_storage,
        }
    }

    /// Simply allocates an entity without any components.
    pub fn allocate_entity(&mut self) -> Entity {
        if let Some(id) = self.free_ids.pop() {
            let generation = self.generations[id as usize];
            Entity::new(id, generation)
        } else {
            let id = self.next_id.fetch_add(1, Ordering::Relaxed);
            if id > self.generations.len() as u64 {
                self.generations.push(0);
            }
            Entity::new(id, 0)
        }
    }

    /// Deallocates an entity, making the id available for reuse.
    /// Returns the deallocated entity on success.
    pub fn deallocate(&mut self, entity: Entity) -> ferrumc_utils::prelude::Result<Entity> {
        let id = entity.id() as usize;

        if id >= self.generations.len() {
            // Invalid entity, since the id is out of bounds
            let error = error::Error::DeallocationError(DeallocationErrorType::EntityNotFound(entity));
            return Err(error.into());
        }

        if self.generations[id] != entity.generation() {
            // Invalid entity, since the generation does not match
            // return Err(error::Error::DeallocationError("DeallocErrorType::InvalidGeneration".to_string()).into());
            let error = error::Error::DeallocationError(DeallocationErrorType::InvalidGeneration(entity));
            return Err(error.into());
        }

        self.generations[id] += 1;
        self.free_ids.push(id as u64);

        Ok(entity)
    }

    pub fn total_entities(&self) -> usize {
        self.generations.len()
    }
}

pub struct EntityBuilder<'a> {
    entity: Entity,
    component_storage: &'a mut ComponentStorage,
}

impl<'a> EntityBuilder<'a> {
    pub fn with<T: Component>(self, component: T) -> Self {
        self.component_storage.insert(&self.entity, component);
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
    fn test_entity_creation() {
        let mut allocator = EntityAllocator::new();
        let e1 = allocator.allocate_entity();
        let e2 = allocator.allocate_entity();
        assert_ne!(e1, e2);
        assert_eq!(e1.id() + 1, e2.id());
    }

    #[test]
    fn test_entity_generation() {
        let mut allocator = EntityAllocator::new();
        let e1 = allocator.allocate_entity();
        let e1 = allocator.deallocate(e1).expect("Failed to deallocate entity");
        let e2 = allocator.allocate_entity();
        assert_ne!(e1, e2);
        assert_eq!(e1.id() + 1, e2.id());
        assert_eq!(e1.generation(), 0);
        assert_eq!(e2.generation(), 1);
    }
}