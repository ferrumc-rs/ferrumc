use crate::components::storage::{Component, ComponentRef, ComponentRefMut, ComponentSparseSet};
use dashmap::{DashMap};
use parking_lot::RwLock;
use std::any::{TypeId};

mod storage;


pub type Result<T> = std::result::Result<T, ECSError>;

#[derive(thiserror::Error, Debug)]
pub enum ECSError {
    #[error("Component retrieval error")]
    ComponentRetrievalError,
    #[error("Component is locked")]
    ComponentIsLocked,
}


pub struct ComponentManager {
    components: DashMap<TypeId, *const ()>,
    storage: RwLock<Vec<Box<dyn ComponentStorage>>>,
}

pub trait ComponentStorage {
    fn as_ptr(&self) -> *const ();
}
impl<T: Component> ComponentStorage for ComponentSparseSet<T> {
    fn as_ptr(&self) -> *const () {
        self as *const Self as *const ()
    }
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: DashMap::new(),
            storage: RwLock::new(Vec::new()),
        }
    }

    pub fn insert<T: Component>(&self, entity_id: usize, component: T) -> Result<()> {
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
    pub fn get<T: Component>(&self, entity_id: usize) -> Option<ComponentRef<T>> {
        let type_id = TypeId::of::<T>();
        let ptr = *self.components.get(&type_id)?;
        let component_set = unsafe { &*(ptr as *const ComponentSparseSet<T>) };
        component_set.get(entity_id).ok()
    }

    pub fn get_mut<T: Component>(&self, entity_id: usize) -> Option<ComponentRefMut<T>> {
        let type_id = TypeId::of::<T>();
        let ptr = *self.components.get(&type_id)?;
        let component_set = unsafe { &*(ptr as *const ComponentSparseSet<T>) };
        component_set.get_mut(entity_id).ok()
    }

    /*pub fn get_mut<T: Component>(&self, entity_id: usize) -> Option<ComponentRefMut<T>> {
        let type_id = TypeId::of::<T>();
        let ptr = *self.components.get(&type_id)?;
        let component_set = unsafe { &*(ptr as *const ComponentSparseSet<T>) };
        component_set.get_mut(entity_id).ok()
    }

    pub fn remove<T: Component>(&self, entity_id: usize) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let ptr = *self.components.get(&type_id).ok_or(ECSError::ComponentTypeNotFound)?;
        let component_set = unsafe { &mut *(ptr as *mut ComponentSparseSet<T>) };
        component_set.remove(entity_id)?;
        Ok(())
    }*/
}