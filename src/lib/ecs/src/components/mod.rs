use crate::components::storage::{Component, ComponentRef, ComponentRefMut, ComponentSparseSet};
use crate::errors::ECSError;
use crate::ECSResult;
use scc::HashMap;
use std::any::TypeId;
#[cfg(debug_assertions)]
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod storage;

unsafe impl Send for ComponentManager {}
unsafe impl Sync for ComponentManager {}

pub struct PtrWrapper(*mut ());

impl Deref for PtrWrapper {
    type Target = *mut ();
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<*mut ()> for PtrWrapper {
    fn from(ptr: *mut ()) -> Self {
        PtrWrapper(ptr)
    }
}

impl From<PtrWrapper> for *mut () {
    fn from(ptr: PtrWrapper) -> Self {
        ptr.0
    }
}

impl Clone for PtrWrapper {
    fn clone(&self) -> Self {
        PtrWrapper(self.0)
    }
}

impl Copy for PtrWrapper {}

unsafe impl Send for PtrWrapper {}
unsafe impl Sync for PtrWrapper {}

pub struct ComponentManager {
    components: Arc<HashMap<TypeId, PtrWrapper>>,
    storage: Arc<RwLock<Vec<Box<dyn ComponentStorage>>>>,
}

pub trait ComponentStorage {
    fn as_ptr(&self) -> *const ();
    fn remove_component(&self, entity_id: usize) -> ECSResult<()>;
}
impl<T: Component> ComponentStorage for ComponentSparseSet<T> {
    fn as_ptr(&self) -> *const () {
        self as *const Self as *const ()
    }

    fn remove_component(&self, entity_id: usize) -> ECSResult<()> {
        self.remove(entity_id)
    }
}

impl Default for ComponentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: Arc::default(),
            storage: Arc::default(),
        }
    }

    pub async fn insert<T: Component>(&self, entity_id: usize, component: T) -> ECSResult<()> {
        use scc::hash_map::Entry;
        let type_id = TypeId::of::<T>();

        match self.components.entry_async(type_id).await {
            Entry::Occupied(entry) => {
                let ptr = entry.get();
                let component_set = unsafe { ptr.0.cast::<ComponentSparseSet<T>>().as_ref() }.expect("ComponentSparseSet is null");
                component_set.insert(entity_id, component)?;
            }
            Entry::Vacant(entry) => {
                let component_set = ComponentSparseSet::<T>::new();
                component_set.insert(entity_id, component)?;
                let boxed: Box<dyn ComponentStorage> = Box::new(component_set);
                let ptr = boxed.as_ptr();
                entry.insert_entry(ptr.cast_mut().into());
                self.storage.write().await.push(boxed);
            }
        };

        Ok(())
    }
    pub async fn get<'a, T: Component>(&self, entity_id: usize) -> ECSResult<ComponentRef<'a, T>> {
        let type_id = TypeId::of::<T>();
        let ptr = self
            .components
            .read_async(&type_id, |_k, v| *v)
            .await
            .ok_or(ECSError::ComponentTypeNotFound)?;
        let component_set = unsafe { &*(ptr.0 as *const ComponentSparseSet<T>) };
        let res = component_set.get(entity_id);
        res
    }

    pub async fn get_mut<'a, T: Component>(
        &self,
        entity_id: usize,
    ) -> ECSResult<ComponentRefMut<'a, T>> {
        let type_id = TypeId::of::<T>();
        let ptr = self
            .components
            .read_async(&type_id, |_k, v| *v)
            .await
            .ok_or(ECSError::ComponentTypeNotFound)?;
        let component_set = unsafe { &*(ptr.0 as *const ComponentSparseSet<T>) };
        component_set.get_mut(entity_id)
    }

    pub async fn remove<T: Component>(&self, entity_id: usize) -> ECSResult<()> {
        let type_id = TypeId::of::<T>();
        let ptr = self
            .components
            .read_async(&type_id, |_k, v| *v)
            .await
            .ok_or(ECSError::ComponentTypeNotFound)?;
        let component_set = unsafe { &mut *(ptr.0 as *mut ComponentSparseSet<T>) };
        component_set.remove(entity_id)?;

        Ok(())
    }

    pub async fn remove_all_components(&self, entity_id: usize) -> ECSResult<()> {
        for storage in self.storage.read().await.iter() {
            storage.remove_component(entity_id)?;
        }

        Ok(())
    }

    pub async fn get_entities_with<T: Component>(&self) -> Vec<usize> {
        let type_id = TypeId::of::<T>();
        let Some(ptr) = self.components.get_async(&type_id).await else {
            return Vec::new();
        };
        let ptr = *ptr;
        let component_set = unsafe { &*(ptr.0 as *const ComponentSparseSet<T>) };
        component_set.entities()
    }
}
