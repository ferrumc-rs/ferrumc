use std::marker::PhantomData;

use crate::components::{Component, ComponentStorage};

pub trait QueryFilter: 'static {
    type Item<'a>;
    fn filter_fetch<'a>(storage: &'a ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>>;
}
impl<T: Component> QueryFilter for T {
    type Item<'a> = &'a T;

    fn filter_fetch<'a>(storage: &'a ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>> {
        storage.get::<T>(entity_id)
    }
}


pub struct Query<'a, F: QueryFilter> {
    storage: &'a ComponentStorage,
    _marker: PhantomData<F>,
}

impl<'a, F: QueryFilter> Query<'a, F> {
    pub fn new(storage: &'a ComponentStorage) -> Self {
        Query {
            storage,
            _marker: PhantomData,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, F::Item<'a>)> + '_ {
        (0..=self.storage.max_entity_id())
            .filter_map(|entity_id| {
                F::filter_fetch(self.storage, entity_id)
                    .map(|item| (entity_id, item))
            })
    }
}

// You can add more implementations for tuples of 3, 4, etc. components if needed

pub trait System {
    fn run(&mut self, storage: &ComponentStorage);
}

mod tests {
    use crate::components::{ComponentStorage, Position, Velocity};
    use crate::query::System;
    use crate::world::EntityAllocator;

    pub struct PhysicsSystem;

    impl System for PhysicsSystem {
        fn run(&mut self, storage: &ComponentStorage) {
            let query = storage.query::<Position>();
            for (entity_id, position) in query.iter() {
                if let Some(velocity) = storage.get::<Velocity>(entity_id) {
                    println!("Entity {}: Position {:?}, Velocity {:?}", entity_id, position, velocity);
                }
            }
        }
    }

    #[test]
    fn test_physics_system() {
        let mut storage = ComponentStorage::new();
        let mut allocator = EntityAllocator::new();


        allocator.allocate(&mut storage)
            .with(Position::new(0.0, 0.0, 0.0))
            .with(Velocity::new(1.0, 1.0, 1.0))
            .build();

        allocator.allocate(&mut storage)
            .with(Position::new(0.0, 0.0, 0.0))
            .build();

        allocator.allocate(&mut storage)
            .with(Position::new(9.0, 1.0, 5.0))
            .with(Velocity::new(1.0, 1.0, 1.0))
            .build();


        let mut system = PhysicsSystem;
        system.run(&storage);
    }
}