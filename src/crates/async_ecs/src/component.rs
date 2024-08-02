use std::any::TypeId;
use std::fmt::Debug;
use std::future::IntoFuture;
use std::marker::PhantomData;
use std::sync::Arc;

use dashmap::DashMap;
use dashmap::mapref::one::{Ref, RefMut};
use tokio::sync::{RwLock, RwLockReadGuard};

use crate::helpers::sparse_set::SparseSet;



pub trait DynamicComponent: 'static + Send + Sync + Debug {}

#[derive(Debug)]
pub struct ComponentRef<'a, T: DynamicComponent> {
    read_guard: RwLockReadGuard<'a, Box<dyn DynamicComponent>>,
    _phantom: PhantomData<T>,
}

#[derive(Debug)]
pub struct ComponentRefMut<'a, T: DynamicComponent> {
    write_guard: &'a RwLock<Box<dyn DynamicComponent>>,
    _phantom: PhantomData<T>,
}


impl<'id, T: DynamicComponent> std::ops::Deref for ComponentRef<'id, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&**self.read_guard as *const dyn DynamicComponent as *const T) }
    }
}


type StoragesMap = DashMap<TypeId, SparseSet<RwLock<Box<dyn DynamicComponent>>>>;

#[derive(Debug)]
pub struct ComponentStorage {
    // storages: DashMap<TypeId, SparseSet<RwLock<Box<dyn DynamicComponent>>>>,s
    storages: Arc<StoragesMap>,
}


impl ComponentStorage {
    pub fn new() -> Self {
        Self {
            storages: Arc::new(DashMap::new()),
        }
    }

    pub fn insert<T: DynamicComponent>(&self, entity_id: impl Into<usize>, component: T) {
        let type_id = TypeId::of::<T>();

        let mut storage = self.storages.entry(type_id).or_insert_with(|| SparseSet::new());

        storage.insert(entity_id.into(), RwLock::new(Box::new(component)));
    }
    pub async fn get<T: DynamicComponent>(&self, entity_id: impl Into<usize>) -> Option<ComponentRef<T>> {
        let type_id = TypeId::of::<T>();

        let entity_id = entity_id.into();

        let storage = self.storages.get(&type_id)?;
        let component = storage.get(entity_id)?;

        // SAFETY: The RwLock is guaranteed to outlive self, so this read_guard can use the same lifetime
        let read_guard = unsafe {
            std::mem::transmute::<RwLockReadGuard<'_, Box<dyn DynamicComponent>>, RwLockReadGuard<'_, Box<dyn DynamicComponent>>>(component.read().await)
        };

        Some(ComponentRef {
            read_guard,
            _phantom: PhantomData,
        })
    }

    pub async fn get_mut<T: DynamicComponent>(&self, entity_id: impl Into<usize>) -> Option<ComponentRefMut<T>> {
        let type_id = TypeId::of::<T>();

        let entity_id = entity_id.into();

        let storage = self.storages.get(&type_id)?;
        let component = storage.get(entity_id)?;

        // SAFETY: The RwLock is guaranteed to outlive self, so this write_guard can use the same lifetime
        let write_guard = unsafe {
            std::mem::transmute::<&RwLock<Box<dyn DynamicComponent>>, &RwLock<Box<dyn DynamicComponent>>>(component)
        };

        Some(ComponentRefMut {
            write_guard,
            _phantom: PhantomData,
        })
    }

    pub fn remove<T: DynamicComponent>(&self, entity_id: impl Into<usize>) {
        let type_id = TypeId::of::<T>();

        let entity_id = entity_id.into();

        if let Some(mut storage) = self.storages.get_mut(&type_id) {
            storage.remove(entity_id);
        }
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

#[tokio::test]
async fn test_basic_usage() {
    let component_storage = ComponentStorage::new();

    let entity = 0usize;
    let position = Position { x: 0.0, y: 0.0 };
    component_storage.insert(entity, position);

    let position = component_storage.get::<Position>(entity).await.unwrap();
    assert_eq!(position.x, 0.0);
    assert_eq!(position.y, 0.0);
}

#[tokio::test]
async fn test_insert_and_get() {
    let storage = ComponentStorage::new();
    storage.insert(0usize, Position { x: 0.0, y: 0.0 });
    let component = storage.get::<Position>(0usize).await;
    assert!(component.is_some());
    assert_eq!(component.unwrap().x, 0.0);
}