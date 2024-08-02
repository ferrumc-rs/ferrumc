use std::any::TypeId;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use dashmap::DashMap;
use parking_lot::RwLock;

use crate::helpers::sparse_set::SparseSet;

// Component trait

trait DynamicComponent: 'static + Send + Sync + Debug {}

#[derive(Debug)]
pub struct ComponentRef<'a, T: DynamicComponent> {
    // guard: parking_lot::RwLockReadGuard<'a, Box<dyn DynamicComponent>>,
    guard: &'a RwLock<Box<dyn DynamicComponent>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: DynamicComponent> Deref for ComponentRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.guard.read().as_ref() as *const dyn DynamicComponent as *const T) }
    }
}

impl<'a, T: DynamicComponent> DerefMut for ComponentRef<'a, T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.guard.write().as_mut() as *mut dyn DynamicComponent as *mut T) }
    }
}


#[derive(Debug)]
pub struct ComponentStorage {
    storages: DashMap<TypeId, SparseSet<RwLock<Box<dyn DynamicComponent>>>>,
}


impl ComponentStorage {
    pub fn new() -> Self {
        Self {
            storages: DashMap::new(),
        }
    }

    pub fn insert<T: DynamicComponent>(&self, entity_id: impl Into<usize>, component: T) {
        let type_id = TypeId::of::<T>();

        let mut storage = self.storages.entry(type_id).or_insert_with(SparseSet::new);

        storage.insert(entity_id.into(), RwLock::new(Box::new(component)));
    }

    /// Get a component of a specific type for an entity
    /// Returns None if the entity does not have the component
    /// or if the entity does not exist
    /// NOTE: Uses unsafe
    pub fn get<T: DynamicComponent>(&self, entity_id: impl Into<usize>) -> Option<ComponentRef<T>> {
        let type_id = TypeId::of::<T>();
        let storage = self.storages.get(&type_id)?;
        let entity_id = entity_id.into();

        let guard = storage.get(entity_id.into())?;

        Some(ComponentRef {
            // Safety: prayers 🤲
            guard: unsafe { std::mem::transmute(guard) },
            _phantom: std::marker::PhantomData,
        })
    }
}


#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl DynamicComponent for Position {}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
impl DynamicComponent for Velocity {}