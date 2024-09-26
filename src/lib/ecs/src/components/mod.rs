use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use dashmap::DashMap;
use sparse_set::SparseSet;

use crate::entities::Entity;

pub mod sparse_set;

pub trait Component: Any + Send + Sync {}

pub struct ComponentStorage {
    pub components: DashMap<TypeId, SparseSet<Arc<dyn Any + Send + Sync>>>,
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
            .or_insert_with(|| SparseSet::new());
        components.insert(entity, Arc::new(component));
    }

    pub fn get<T: Component>(&self, entity: Entity) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        let components = self.components.get(&type_id)?;

        let component = components.get(entity)?;
        let component = Arc::downcast::<T>(component.clone()).ok()?;

        Some(component)
    }

    pub fn remove<T: Component>(&self, entity: Entity) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        let mut components = self.components.get_mut(&type_id)?;

        let component = components.remove(entity)?;
        let component = Arc::downcast::<T>(component.clone()).ok()?;

        Some(component)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::EntityManager;

    struct Position {
        x: f32,
        y: f32,
    }

    impl Component for Position {}

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
}
