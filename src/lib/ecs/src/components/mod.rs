/// How does the ECS work? Barely
///
/// ok but this is how it works.
///
/// You've got 2 layers of maps: You got a map for mapping a type to a storage mapping, which in 
/// turn maps an entity ID to its component of that type. You can kinda think of it as a 
/// `HashMap<TypeID, HashMap<entity_id, component>>` except with more borrow checker abuse. Also
/// sparse sets are in there somewhere, but idfk how those work, i think they are just an optimised
/// hashmap? Ask Sweatty.
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

/// A sketchy wrapper over pointers so we can convince the borrow checker the pointer is Send+Sync
///
/// It's probably not but shhhh
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
        *self
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

    /// Inserts a component into the component manager
    ///
    /// You probably don't want to be using this function directly.
    pub async fn insert<T: Component>(&self, entity_id: usize, component: T) -> ECSResult<()> {
        use scc::hash_map::Entry;
        let type_id = TypeId::of::<T>();
        // Get the entry directly and modify, instead of using `.contains()` and`.insert()`, since
        // that can cause data races if the value is added/removed between the two calls.
        match self.components.entry_async(type_id).await {
            Entry::Occupied(entry) => {
                let ptr = entry.get();
                let component_set = unsafe { ptr.0.cast::<ComponentSparseSet<T>>().as_ref() }
                    .expect("ComponentSparseSet is null");
                component_set.insert(entity_id, component)?;
            }
            // A vacant entry can be thought of as the place where the value would be if it existed.
            // So by getting the lock for a vacant entry, we ensure that we have exclusive access to
            // where the value would be if it existed, and can insert it without any data race issues.
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
    /// Gets a component from the component manager
    ///
    /// You probably don't want to be using this function directly.
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

    /// Gets a mutable reference to a component from the component manager
    /// 
    /// You probably don't want to be using this function directly.
    pub async fn get_mut<'a, T: Component>(
        &self,
        entity_id: usize,
    ) -> ECSResult<ComponentRefMut<'a, T>> {
        let type_id = TypeId::of::<T>();
        let ptr = self
            .components
            // Since we are just getting a pointer from the hashmap and then unsafely casting it,
            // we can just get a read lock on the hashmap, since we are not modifying the hashmap
            // itself.
            .read_async(&type_id, |_k, v| *v)
            .await
            .ok_or(ECSError::ComponentTypeNotFound)?;
        let component_set = unsafe { &*(ptr.0 as *mut ComponentSparseSet<T>) };
        component_set.get_mut(entity_id)
    }

    /// Removes a component from the component manager
    /// 
    /// You probably don't want to be using this function directly.
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

    /// Removes all components from an entity
    /// 
    /// You probably don't want to be using this function directly.
    pub async fn remove_all_components(&self, entity_id: usize) -> ECSResult<()> {
        for storage in self.storage.read().await.iter() {
            storage.remove_component(entity_id)?;
        }

        Ok(())
    }

    /// Gets all entities with a component of type `T`
    /// 
    /// You probably don't want to be using this function directly.
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
