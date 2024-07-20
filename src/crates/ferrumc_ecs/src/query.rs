use std::marker::PhantomData;
use crate::components::{Component, ComponentStorage};


pub trait QueryFilter: 'static {
    type Item<'a>;
    unsafe fn filter_fetch<'a>(storage: *const ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>>;
}

pub trait QueryFilterMut: 'static {
    type Item<'a>;
    unsafe fn filter_fetch_mut<'a>(storage: *mut ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>>;
}

impl<T: Component> QueryFilter for T {
    type Item<'a> = &'a T;

    unsafe fn filter_fetch<'a>(storage: *const ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>> {
        (*storage).get::<T>(entity_id)
    }
}

impl<T: Component> QueryFilterMut for T {
    type Item<'a> = &'a mut T;

    unsafe fn filter_fetch_mut<'a>(storage: *mut ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>> {
        (*storage).get_mut::<T>(entity_id)
    }
}

pub struct Query<'a, F: QueryFilter> {
    storage: *const ComponentStorage,
    _marker: PhantomData<&'a F>,
}

pub struct QueryMut<'a, F: QueryFilterMut> {
    storage: *mut ComponentStorage,
    _marker: PhantomData<&'a mut F>,
}

impl<'a, F: QueryFilter> Query<'a, F> {
    pub fn new(storage: &'a ComponentStorage) -> Self {
        Query {
            storage: storage as *const ComponentStorage,
            _marker: PhantomData,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, F::Item<'a>)> + '_ {
        let max_entity_id = unsafe { (*self.storage).max_entity_id() };
        (0..=max_entity_id).filter_map(|entity_id| {
            unsafe {
                F::filter_fetch(self.storage, entity_id).map(|item| (entity_id, item))
            }
        })
    }
}

impl<'a, F: QueryFilterMut> QueryMut<'a, F> {
    pub fn new(storage: &'a mut ComponentStorage) -> Self {
        QueryMut {
            storage: storage as *mut ComponentStorage,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, F::Item<'a>)> + '_ {
        let max_entity_id = unsafe { (*self.storage).max_entity_id() };
        (0..=max_entity_id).filter_map(|entity_id| {
            unsafe {
                F::filter_fetch_mut(self.storage, entity_id).map(|item| (entity_id, item))
            }
        })
    }
}

impl<A: Component, B: Component> QueryFilter for (A, B) {
    type Item<'a> = (&'a A, &'a B);

    unsafe fn filter_fetch<'a>(storage: *const ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>> {
        Some((
            (*storage).get::<A>(entity_id)?,
            (*storage).get::<B>(entity_id)?
        ))
    }
}

impl<A: Component, B: Component> QueryFilterMut for (A, B) {
    type Item<'a> = (&'a mut A, &'a mut B);

    unsafe fn filter_fetch_mut<'a>(storage: *mut ComponentStorage, entity_id: usize) -> Option<Self::Item<'a>> {
        Some((
            (*storage).get_mut::<A>(entity_id)?,
            (*storage).get_mut::<B>(entity_id)?
        ))
    }
}

mod tests  {
    use crate::components::{ComponentStorage, Position, Velocity};
    use crate::query::{Query, QueryMut};
    use crate::world::EntityAllocator;

    struct MovementSystem;

    impl MovementSystem {
        pub fn run(storage: &mut ComponentStorage) {
            let mut query = QueryMut::<(Position, Velocity)>::new(storage);

            for (_, (pos, vel)) in query.iter_mut() {
                pos.add_velocity(vel);
            }
        }
    }

    struct EntityLogger;

    impl EntityLogger {
        pub fn log(storage: &ComponentStorage) {
            let query = Query::<Position>::new(storage);

            for (entity_id, pos) in query.iter() {
                println!("Entity {}: {:?}", entity_id, pos);
            }
        }
    }

    #[test]
    fn test_movement_system() {
        let mut allocator = EntityAllocator::new();
        let mut storage = ComponentStorage::new();

        allocator.allocate(&mut storage)
            .with(Position::new(0.0, 0.0, 0.0))
            .with(Velocity::new(1.0, 1.0, 1.0));

        allocator.allocate(&mut storage)
            .with(Position::new(1.0, 0.0, 3.0))
            .with(Velocity::new(2.0, 2.0, 2.0));

        allocator.allocate(&mut storage)
            .with(Position::new(2.0, 0.0, 6.0));

        MovementSystem::run(&mut storage);
        EntityLogger::log(&storage);
    }
}