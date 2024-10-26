/*use dashmap::DashMap;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use sparse_set::SparseSet;
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use crate::ECSResult;
use crate::entities::Entity;
use crate::errors::ECSError;

pub mod sparse_set;

pub trait Component: Any + Send + Sync {}
impl<T: Any + Send + Sync> Component for T {}

unsafe impl<T> Send for ComponentRef<'_, T> where T: Component {}
unsafe impl<T> Send for ComponentRefMut<'_, T> where T: Component {}

pub struct ComponentStorage {
    pub components: DashMap<TypeId, SparseSet<RwLock<Box<dyn Component>>>>,
}

impl Default for ComponentStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentStorage {
    pub fn new() -> Self {
        ComponentStorage {
            components: DashMap::new(),
        }
    }

    pub fn insert<T: Component>(&self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        let mut components = self
            .components
            .entry(type_id)
            .or_default();
        components.insert(entity, RwLock::new(Box::new(component)));
    }

    pub fn get_entities_with<T: Component>(&self) -> Vec<Entity> {
        let type_id = TypeId::of::<T>();
        let components = match self.components.get(&type_id) {
            Some(components) => components,
            None => {
                return Vec::new();
            }
        };
        
        components.value().entities()
    }
    
    pub fn remove<T: Component>(&self, entity: Entity) {
        let type_id = TypeId::of::<T>();
        self.components.get_mut(&type_id)
            .map(|mut components| components.remove(entity));
    }

    pub fn remove_all_components(&self, entity: Entity) -> ECSResult<()> {
        self.components.iter_mut()
            .for_each(|mut components| {
                // check if its locked or not
                if let Some(component) = components.get_mut(entity) {
                    let lock = component.write();
                    // basically wait for component to be able to be written to (or have no readers & writers)
                    drop(lock);
                    // Remove else-wise
                    components.remove(entity);
                }
            });
        
        Ok(())
    }
}
impl ComponentStorage {
    pub fn get<'a, T: Component>(&self, entity: Entity) -> ECSResult<ComponentRef<'a, T>>
    {
        let type_id = TypeId::of::<T>();
        let components = self.components.get(&type_id)
            .ok_or(ECSError::ComponentNotFound)?;
        let component = components.get(entity)
            .ok_or(ECSError::ComponentNotFound)?;

        let read_guard = component.try_read()
            .ok_or(ECSError::ComponentLocked)?;

        let read_guard = unsafe {
            std::mem::transmute::<
                RwLockReadGuard<'_, Box<dyn Component>>,
                RwLockReadGuard<'a, Box<dyn Component>>,
            >(read_guard)
        };

        Ok(ComponentRef {
            read_guard,
            _phantom: PhantomData,
        })
    }

    pub fn get_mut<'a, T: Component>(&self, entity: Entity) -> ECSResult<ComponentRefMut<'a, T>>
    {
        let type_id = TypeId::of::<T>();
        let components = self.components.get(&type_id)
            .ok_or(ECSError::ComponentNotFound)?;
        let component = components.get(entity)
            .ok_or(ECSError::ComponentNotFound)?;

        let write_guard = component.try_write()
            .ok_or(ECSError::ComponentLocked)?;

        let write_guard = unsafe {
            std::mem::transmute::<
                RwLockWriteGuard<'_, Box<dyn Component>>,
                RwLockWriteGuard<'a, Box<dyn Component>>,
            >(write_guard)
        };

        Ok(ComponentRefMut {
            write_guard,
            _phantom: PhantomData,
        })
    }
}
pub struct ComponentRef<'a, T: Component> {
    read_guard: RwLockReadGuard<'a, Box<dyn Component>>,
    _phantom: PhantomData<&'a T>,
}

pub struct ComponentRefMut<'a, T: Component> {
    write_guard: RwLockWriteGuard<'a, Box<dyn Component>>,
    _phantom: PhantomData<&'a mut T>,
}
mod debug {
    use std::fmt::Debug;
    use crate::components::{Component, ComponentRef, ComponentRefMut};

    impl<T: Component + Debug> Debug for ComponentRef<'_, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&**self, f)
        }
    }
    
    impl<T: Component + Debug> Debug for ComponentRefMut<'_, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&**self, f)
        }
    }
}

impl<T: Component> Deref for ComponentRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&**self.read_guard as *const dyn Component as *const T) }
    }
}

impl<T: Component> Deref for ComponentRefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&**self.write_guard as *const dyn Component as *const T) }
    }
}

impl<T: Component> DerefMut for ComponentRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(&mut **self.write_guard as *mut dyn Component as *mut T) }
    }
}



#[cfg(test)]
mod tests {
    use crate::components::ComponentStorage;
    use crate::entities::EntityManager;

    struct Position {
        x: f32,
        y: f32,
    }

    #[test]
    fn insert_get() {
        let entity_manager = EntityManager::new();
        let component_storage = ComponentStorage::new();
        let entity = entity_manager.create_entity();
        let position = Position { x: 0.0, y: 0.0 };
        component_storage.insert(entity, position);

        let position = component_storage.get::<Position>(entity).unwrap();
        assert_eq!(position.x, 0.0);
        assert_eq!(position.y, 0.0);
    }

    #[test]
    fn insert_get_mut() {
        let entity_manager = EntityManager::new();
        let component_storage = ComponentStorage::new();
        let entity = entity_manager.create_entity();
        let position = Position { x: 0.0, y: 0.0 };
        component_storage.insert(entity, position);

        let mut position = component_storage.get_mut::<Position>(entity).unwrap();
        position.x = 1.0;
        position.y = 2.0;

        assert_eq!(position.x, 1.0);
        assert_eq!(position.y, 2.0);
    }

    #[test]
    fn test_multi_mut() {
        let entity_manager = EntityManager::new();
        let component_storage = ComponentStorage::new();
        let entity = entity_manager.create_entity();
        let position = Position { x: 0.0, y: 0.0 };
        component_storage.insert(entity, position);

        let mut position = component_storage.get_mut::<Position>(entity).unwrap();
        position.x = 1.0;
        position.y = 2.0;

        let position = component_storage.get_mut::<Position>(entity);

        assert!(position.is_err());
    }
}
*/use crate::components::storage::{Component, ComponentRef, ComponentRefMut, ComponentSparseSet};
use dashmap::DashMap;
use parking_lot::RwLock;
use std::any::TypeId;
use crate::{errors::ECSError, ECSResult};

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
    pub fn get<'a, T: Component>(&self, entity_id: usize) -> Option<ComponentRef<'a, T>> {
        let type_id = TypeId::of::<T>();
        let ptr = *self.components.get(&type_id)?;
        let component_set = unsafe { &*(ptr as *const ComponentSparseSet<T>) };
        component_set.get(entity_id).ok()
    }

    pub fn get_mut<'a, T: Component>(&self, entity_id: usize) -> Option<ComponentRefMut<'a, T>> {
        let type_id = TypeId::of::<T>();
        let ptr = *self.components.get(&type_id)?;
        let component_set = unsafe { &*(ptr as *const ComponentSparseSet<T>) };
        component_set.get_mut(entity_id).ok()
    }

    pub fn remove<T: Component>(&self, entity_id: usize) -> ECSResult<()> {
        let type_id = TypeId::of::<T>();
        let ptr = *self.components.get(&type_id).ok_or(ECSError::ComponentTypeNotFound)?;
        let component_set = unsafe { &mut *(ptr as *mut ComponentSparseSet<T>) };
        component_set.remove(entity_id)?;

        Ok(())
    }

    pub fn remove_all_components(&self, entity_id: usize) -> ECSResult<()>{
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
