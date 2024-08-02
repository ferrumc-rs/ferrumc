use std::any::TypeId;
use std::fmt::Debug;
use std::sync::RwLock;

use dashmap::{DashMap, Entry};
use dashmap::mapref::one::{Ref, RefMut};
use crate::Error;
use crate::helpers::sparse_set::SparseSet;

// Component trait
pub trait Component: 'static + Send + Sync + Debug {}

#[derive(Debug)]
pub struct ComponentStorage {
    storages: DashMap<TypeId, SparseSet<RwLock<Box<dyn Component>>>>,
}


impl ComponentStorage {
    pub fn new() -> Self {
        Self {
            storages: DashMap::new(),
        }
    }

    pub fn insert<T: Component>(&self, entity_id: impl Into<usize>, component: T) {
        let type_id = TypeId::of::<T>();

        let mut storage = self.storages.entry(type_id).or_insert_with(|| SparseSet::new());
        storage.insert(entity_id.into(), RwLock::new(Box::new(component)));
    }

    pub fn get<T: Component>(&self, entity_id: impl Into<usize>) -> Result<?, Error> {
    }

    fn get_storage<'a, T: Component>(&self) -> RefMut<'a, TypeId, SparseSet<RwLock<Box<dyn Component>>>> {
        let type_id = TypeId::of::<T>();
        self.storages.entry(type_id).or_insert_with(|| SparseSet::new())
    }
}


#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Component for Position {}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
impl Component for Velocity {}