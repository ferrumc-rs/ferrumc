use std::any::TypeId;
use std::fmt::Debug;
use std::future::{Future, IntoFuture};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::helpers::sparse_set::SparseSet;

// Component trait

pub trait DynamicComponent: 'static + Send + Sync + Debug {}

#[derive(Debug)]
pub struct ComponentRef<'a, T: DynamicComponent> {
    read_guard: RwLockReadGuard<'a, Box<dyn DynamicComponent>>,
    _phantom: std::marker::PhantomData<T>,
}

/*impl<'a, T: DynamicComponent> Deref for ComponentRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.read_guard.as_ref() as *const dyn DynamicComponent as *const T) }
    }
}
*/

/*#[derive(Debug)]
pub struct ComponentRefMut<'a, T: DynamicComponent> {
    write_guard: &'a RwLockWriteGuard<'a, Box<dyn DynamicComponent>>,
    _phantom: std::marker::PhantomData<T>,
}
impl<'a, T: DynamicComponent> Deref for ComponentRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.write_guard.as_ref() as *const dyn DynamicComponent as *const T) }
    }
}*/




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

    // pub async fn get<T: DynamicComponent>(&self, entity_id: impl Into<usize>) -> Option<ComponentRef<T>> {
    pub fn get<T: DynamicComponent>(&self, entity_id: impl Into<usize>) -> Option<impl Future<Output = ComponentRef<T>> + '_> {
        let type_id = TypeId::of::<T>();
        let entity_id = entity_id.into();

        let storage = self.storages.get(&type_id)?;
        let component= storage.get(entity_id)?;
        
        Some(async move {
            let guard = component.read().await;
            
            ComponentRef {
                read_guard: guard,
                _phantom: std::marker::PhantomData,
            }
        })
        
        /*let guard = component.read().await;

        Some(ComponentRef {
            read_guard: guard,
            _phantom: std::marker::PhantomData,
        })*/
    }

    /*pub async fn get_mut<T: DynamicComponent>(&self, entity_id: impl Into<usize>) -> Option<ComponentRefMut<T>> {
        let type_id = TypeId::of::<T>();
        let storage = self.storages.get(&type_id)?;
        let entity_id = entity_id.into();

        let guard = storage.get(entity_id.into())?;
        let guard = guard.write().await;

        Some(ComponentRefMut {
            // SAFETY: The component cannot possibly outlive the storage itself. Perhaps?
            // If it does, then it'll be a dangling reference, which is UB (undefined behavior)
            // Trick to make the borrow checker happy
            write_guard: unsafe { std::mem::transmute(&guard) },
            _phantom: std::marker::PhantomData,
        })
    }*/
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