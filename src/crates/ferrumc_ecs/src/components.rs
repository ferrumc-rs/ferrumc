use std::any::{Any, TypeId};
use std::collections::HashMap;
use ecs_macros::{Component, Constructor};
use crate::{component_id};
use crate::dsa::sparse_set::SparseSet;
use crate::query::{Query, QueryFilter};

pub trait Component: 'static {}

#[derive(Debug, Component, Constructor)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Component, Constructor)]
pub struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn add_velocity(&mut self, velocity: &Velocity) {
        self.x += velocity.x;
        self.y += velocity.y;
        self.z += velocity.z;
    }
}


pub struct ComponentStorage {
    storages: HashMap<TypeId, Box<dyn Any>>,
    max_entity_id: usize,
}

impl ComponentStorage {
    pub fn new() -> Self {
        ComponentStorage {
            storages: HashMap::new(),
            max_entity_id: 0,
        }
    }

    pub fn insert<T: Component>(&mut self, entity_id: impl Into<usize>, component: T) {
        let entity_id = entity_id.into();

        let storage = self.storages
            .entry(component_id!(T))
            .or_insert_with(|| Box::new(SparseSet::<T>::new()));

        let storage = storage.downcast_mut::<SparseSet<T>>().unwrap();
        storage.insert(entity_id, component);

        self.max_entity_id = self.max_entity_id.max(entity_id);
    }

    pub fn remove<T: Component>(&mut self, entity_id: impl Into<usize>) -> Option<T> {
        self.storages.get_mut(&component_id!(T))
            .and_then(|storage| {
                let storage = storage.downcast_mut::<SparseSet<T>>().unwrap();
                storage.remove(entity_id.into())
            })
    }

    pub fn get<T: Component>(&self, entity_id: impl Into<usize>) -> Option<&T> {
        self.storages.get(&component_id!(T))
            .and_then(|storage| {
                let storage = storage.downcast_ref::<SparseSet<T>>().unwrap();
                storage.get(entity_id.into())
            })
    }

    pub fn get_mut<T: Component>(&mut self, entity_id: impl Into<usize>) -> Option<&mut T> {
        self.storages.get_mut(&component_id!(T))
            .and_then(|storage| {
                let storage = storage.downcast_mut::<SparseSet<T>>().unwrap();
                storage.get_mut(entity_id.into())
            })
    }

    pub fn get_storage<T: Component>(&self) -> Option<&SparseSet<T>> {
        self.storages.get(&component_id!(T))
            .and_then(|storage| {
                storage.downcast_ref::<SparseSet<T>>()
            })
    }

    pub fn get_storage_mut<T: Component>(&mut self) -> Option<&mut SparseSet<T>> {
        self.storages.get_mut(&component_id!(T))
            .and_then(|storage| {
                storage.downcast_mut::<SparseSet<T>>()
            })
    }

    pub fn max_entity_id(&self) -> usize {
        self.max_entity_id
    }
}

impl ComponentStorage {
    pub fn query<F: QueryFilter>(&self) -> Query<F> {
        Query::new(self)
    }
}