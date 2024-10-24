use std::sync::atomic::{AtomicUsize, Ordering};

pub type EntityId = usize;


pub struct EntityAllocator {
    counter: AtomicUsize
}

impl EntityAllocator {
    pub fn new() -> Self {
        EntityAllocator {
            counter: AtomicUsize::new(0)
        }
    }

    pub fn create(&self) -> EntityId {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}