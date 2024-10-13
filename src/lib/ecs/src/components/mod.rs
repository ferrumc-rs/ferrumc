use dashmap::DashMap;
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
            .or_insert_with(SparseSet::new);
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

impl<'a, T: Component> Deref for ComponentRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&**self.read_guard as *const dyn Component as *const T) }
    }
}

impl<'a, T: Component> Deref for ComponentRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&**self.write_guard as *const dyn Component as *const T) }
    }
}

impl<'a, T: Component> DerefMut for ComponentRefMut<'a, T> {
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
