use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, PartialEq)]
struct Entity {
    id: u64,
    generation: u64,
}

impl Entity {
    fn new(id: u64, generation: u64) -> Self {
        Entity { id, generation }
    }

    /// Returns the id of the entity.
    fn id(&self) -> u64 {
        self.id
    }

    /// Returns the generation of the entity.
    fn generation(&self) -> u64 {
        self.generation
    }
}

struct EntityAllocator {
    next_id: AtomicU64,
}

impl EntityAllocator {
    fn new() -> Self {
        EntityAllocator {
            next_id: AtomicU64::new(0),
        }
    }

    fn allocate(&self, generation: u64) -> Entity {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        Entity::new(id, generation)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let allocator = EntityAllocator::new();
        let e1 = allocator.allocate(0);
        let e2 = allocator.allocate(0);
        assert_ne!(e1, e2);
        assert_eq!(e1.id() + 1, e2.id());
    }

    #[test]
    fn test_entity_generation() {
        let allocator = EntityAllocator::new();
        let e1 = allocator.allocate(0);
        let e2 = allocator.allocate(1);
        assert_ne!(e1, e2);
        assert_eq!(e1.id() + 1, e2.id());
        assert_eq!(e1.generation(), 0);
        assert_eq!(e2.generation(), 1);
    }
}