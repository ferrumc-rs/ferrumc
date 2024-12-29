use crate::components::storage::{Component, ComponentRef, ComponentRefMut, ComponentSparseSet};
use crate::errors::ECSError;
use crate::ECSResult;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::any::TypeId;
use std::hash::{Hash, Hasher};
use tracing::trace;

pub mod storage;

unsafe impl Send for ComponentManager {}
unsafe impl Sync for ComponentManager {}
pub struct ComponentManager {
    components: DashMap<TypeId, *const ()>,
    storage: RwLock<Vec<Box<dyn ComponentStorage>>>,
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
            components: DashMap::new(),
            storage: RwLock::new(Vec::new()),
        }
    }

    pub fn insert<T: Component>(&self, entity_id: usize, component: T) -> ECSResult<()> {
        use dashmap::mapref::entry::Entry;
        let type_id = TypeId::of::<T>();

        match self.components.entry(type_id) {
            Entry::Occupied(entry) => {
                let ptr = *entry.get();
                let component_set = unsafe { &mut *(ptr as *mut ComponentSparseSet<T>) };
                component_set.insert(entity_id, component)?;
            }
            Entry::Vacant(entry) => {
                let component_set = ComponentSparseSet::<T>::new();
                component_set.insert(entity_id, component)?;
                let boxed: Box<dyn ComponentStorage> = Box::new(component_set);
                let ptr = boxed.as_ptr();
                entry.insert(ptr);
                self.storage.write().push(boxed);
            }
        };

        Ok(())
    }
    pub fn get<'a, T: Component>(&self, entity_id: usize) -> ECSResult<ComponentRef<'a, T>> {
        let type_id = TypeId::of::<T>();
        #[cfg(debug_assertions)]
        {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            type_id.hash(&mut hasher);
            let type_hash = hasher.finish();
            trace!(
                "Getting static component (ID: {:X}) lock for entity {}",
                type_hash,
                entity_id
            );
            let locked = matches!(
                self.components.try_get(&type_id),
                dashmap::try_result::TryResult::Locked
            );
            if locked {
                trace!(
                    "Static component (ID: {:X}) lock for entity {} is locked",
                    type_hash,
                    entity_id
                );
            }
        }
        let ptr = *self
            .components
            .get(&type_id)
            .ok_or(ECSError::ComponentTypeNotFound)?;
        let component_set = unsafe { &*(ptr as *const ComponentSparseSet<T>) };
        let res = component_set.get(entity_id);
        #[cfg(debug_assertions)]
        {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            type_id.hash(&mut hasher);
            let type_hash = hasher.finish();
            if res.is_ok() {
                trace!(
                    "Got static component (ID: {:X}) lock for entity {}",
                    type_hash,
                    entity_id
                );
            }
        }
        res
    }

    pub fn get_mut<'a, T: Component>(&self, entity_id: usize) -> ECSResult<ComponentRefMut<'a, T>> {
        let type_id = TypeId::of::<T>();
        #[cfg(debug_assertions)]
        {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            type_id.hash(&mut hasher);
            let type_hash = hasher.finish();
            trace!(
                "Getting mutable component (ID: {:X}) lock for entity {}",
                type_hash,
                entity_id
            );
            let locked = matches!(
                self.components.try_get_mut(&type_id),
                dashmap::try_result::TryResult::Locked
            );
            if locked {
                trace!(
                    "Mutable component (ID: {:X}) lock for entity {} is locked",
                    type_hash,
                    entity_id
                );
            }
        }
        let ptr = *self
            .components
            .get(&type_id)
            .ok_or(ECSError::ComponentTypeNotFound)?;
        let component_set = unsafe { &*(ptr as *const ComponentSparseSet<T>) };
        {
            let res = component_set.get_mut(entity_id);
            #[cfg(debug_assertions)]
            {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                type_id.hash(&mut hasher);
                let type_hash = hasher.finish();
                if res.is_ok() {
                    trace!(
                        "Got mutable component (ID: {:X}) lock for entity {}",
                        type_hash,
                        entity_id
                    );
                }
            }
            res
        }
    }

    pub fn remove<T: Component>(&self, entity_id: usize) -> ECSResult<()> {
        let type_id = TypeId::of::<T>();
        let ptr = *self
            .components
            .get(&type_id)
            .ok_or(ECSError::ComponentTypeNotFound)?;
        let component_set = unsafe { &mut *(ptr as *mut ComponentSparseSet<T>) };
        component_set.remove(entity_id)?;

        Ok(())
    }

    pub fn remove_all_components(&self, entity_id: usize) -> ECSResult<()> {
        for storage in self.storage.read().iter() {
            storage.remove_component(entity_id)?;
        }

        Ok(())
    }

    pub fn get_entities_with<T: Component>(&self) -> Vec<usize> {
        let type_id = TypeId::of::<T>();
        let Some(ptr) = self.components.get(&type_id) else {
            return Vec::new();
        };
        let ptr = *ptr;
        let component_set = unsafe { &*(ptr as *const ComponentSparseSet<T>) };
        component_set.entities()
    }
}
