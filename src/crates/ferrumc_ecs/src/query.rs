use crate::components::{Component, ComponentStorage};

pub struct Query<'a, T: Component> {
    storage: &'a mut ComponentStorage,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: Component> Query<'a, T> {
    pub fn new(component_storage: &'a mut ComponentStorage) -> Self {
        Query {
            storage: component_storage,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T: Component> Query<'a, T> {
    pub fn iter(&self) -> impl Iterator<Item=(&usize, &T)> + '_ {
        self.storage
            .get_storage::<T>()
            .map(|storage| storage.iter())
            .into_iter()
            .flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(&mut usize, &mut T)> + '_ {
        self.storage
            .get_storage_mut::<T>()
            .map(|storage| storage.iter_mut())
            .into_iter()
            .flatten()
    }
}

pub trait System {
    fn run(&self, storage: &mut ComponentStorage);
}

mod test {
    use crate::components::{ComponentStorage, Position, Velocity};
    use crate::query::System;
    use crate::world::EntityAllocator;


    #[test]
    fn test_physics_system() {
        pub struct PhysicsSystem;

        impl System for PhysicsSystem {
            fn run(&self, storage: &mut ComponentStorage) {
                let mut query = storage.query::<Position>();
                for (entity, position) in query.iter_mut() {
                    println!("Entity: {:?}, Position: {:?}", entity, position);
                    let Some(velocity) = storage.get::<Velocity>(*entity) else {
                        continue;
                    };
                    println!("Adding velocity: {:?}", velocity);
                    position.add_velocity(velocity);
                }
            }
        }

        let mut storage = ComponentStorage::new();
        let mut allocator = EntityAllocator::new();

        allocator.allocate(&mut storage)
            .with(Position::new(1.0, 2.0, 3.0))
            .with(Velocity::new(2.0, 3.0, 4.0))
            .build();

        allocator.allocate(&mut storage)
            .with(Position::new(1.0, 2.0, 3.0))
            .build();

        let physics_system = PhysicsSystem;
        physics_system.run(&mut storage);
    }
}
