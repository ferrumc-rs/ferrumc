use std::any::{Any, TypeId};
use std::collections::HashMap;
use ecs_macros::{Component, Constructor};
use crate::{component_id};
use crate::dsa::sparse_set::SparseSet;
use crate::world::Entity;

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

// register_components!(Position, Velocity);


pub struct ComponentStorage {
    storages: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentStorage {
    pub fn new() -> Self {
        ComponentStorage {
            storages: HashMap::new(),
        }
    }

    pub fn insert<T: Component>(&mut self, entity:&Entity, component: T) {
        let storage = self.storages
            .entry(component_id!(T))
            .or_insert_with(|| Box::new(SparseSet::<T>::new()));

        let storage = storage.downcast_mut::<SparseSet<T>>().unwrap();
        storage.insert(entity.id() as usize, component);
    }

    pub fn remove<T: Component>(&mut self, entity: Entity) -> Option<T> {
        self.storages.get_mut(&component_id!(T))
            .and_then(|storage| {
                let storage = storage.downcast_mut::<SparseSet<T>>().unwrap();
                storage.remove(entity.id() as usize)
            })
    }

    pub fn get<T: Component>(&self, entity: &Entity) -> Option<&T> {
        self.storages.get(&component_id!(T))
            .and_then(|storage| {
                let storage = storage.downcast_ref::<SparseSet<T>>().unwrap();
                storage.get(entity.id() as usize)
            })
    }

    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        self.storages.get_mut(&component_id!(T))
            .and_then(|storage| {
                let storage = storage.downcast_mut::<SparseSet<T>>().unwrap();
                storage.get_mut(entity.id() as usize)
            })
    }
}