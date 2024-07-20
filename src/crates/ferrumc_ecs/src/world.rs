use std::fmt::Display;
use std::sync::atomic::{AtomicU64, Ordering};
use crate::error;
use crate::error::DeallocationErrorType;

#[derive(Debug, PartialEq)]
pub struct Entity {
    id: u64,
    generation: u64,
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

    pub fn allocate(&mut self) -> Entity {
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let mut allocator = EntityAllocator::new();
        let e1 = allocator.allocate();
        let e2 = allocator.allocate();
        assert_ne!(e1, e2);
        assert_eq!(e1.id() + 1, e2.id());
    }

    #[test]
    fn test_entity_generation() {
        let mut allocator = EntityAllocator::new();
        let e1 = allocator.allocate();
        let e1 = allocator.deallocate(e1).expect("Failed to deallocate entity");
        let e2 = allocator.allocate();
        assert_ne!(e1, e2);
        assert_eq!(e1.id() + 1, e2.id());
        assert_eq!(e1.generation(), 0);
        assert_eq!(e2.generation(), 1);
    }
}