use std::any::{Any, TypeId};
use std::collections::HashMap;
use ecs_macros::{Component, Constructor};
use crate::{component_id};
use crate::dsa::sparse_set::SparseSet;
use crate::query::Query;

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
}

impl ComponentStorage {
    pub fn new() -> Self {
        ComponentStorage {
            storages: HashMap::new(),
        }
    }

    pub fn insert<T: Component>(&mut self, entity_id: impl Into<usize>, component: T) {
        let storage = self.storages
            .entry(component_id!(T))
            .or_insert_with(|| Box::new(SparseSet::<T>::new()));

        let storage = storage.downcast_mut::<SparseSet<T>>().unwrap();
        storage.insert(entity_id.into(), component);
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
}

impl ComponentStorage {
    pub fn query<T: Component>(&mut self) -> Query<T> {
        Query::new(self)
    }
}