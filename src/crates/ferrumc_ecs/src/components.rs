use std::any::{Any, TypeId};
use std::collections::HashMap;


use crate::component_id;
use crate::dsa::sparse_set::SparseSet;
use crate::query::{Query, QueryFilter};

pub trait Component: 'static + Send + Sync {}

pub trait ComponentType: Any + Send + Sync {
    fn remove(&mut self, entity_id: usize);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Component> ComponentType for SparseSet<T> {
    fn remove(&mut self, entity_id: usize) {
        SparseSet::remove(self, entity_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct ComponentStorage {
    storages: HashMap<TypeId, Box<dyn ComponentType>>,
    max_entity_id: usize,
}

impl ComponentStorage {
    pub fn new() -> Self {
        ComponentStorage {
            storages: HashMap::new(),
            max_entity_id: 0,
        }
    }

    pub fn insert<T: Component>(&mut self, entity_id: impl Into<usize>, component: T) {
        let entity_id = entity_id.into();
        let type_id = component_id!(T);

        let storage = self.storages
            .entry(type_id)
            .or_insert(Box::new(SparseSet::<T>::new()));

        let storage = storage.as_any_mut().downcast_mut::<SparseSet<T>>().unwrap();

        storage.insert(entity_id, component);
        self.max_entity_id = self.max_entity_id.max(entity_id);
    }

    pub fn get<T: Component>(&self, entity_id: impl Into<usize>) -> Option<&T> {
        self.storages.get(&component_id!(T))
            .and_then(|storage| {
                let storage = storage.as_any().downcast_ref::<SparseSet<T>>().unwrap();

                storage.get(entity_id.into())
            })
    }

    pub fn get_mut<T: Component>(&mut self, entity_id: impl Into<usize>) -> Option<&mut T> {
        self.storages.get_mut(&component_id!(T))
            .and_then(|storage| {
                let storage = storage.as_any_mut().downcast_mut::<SparseSet<T>>().unwrap();
                storage.get_mut(entity_id.into())
            })
    }

    pub fn remove<T: Component>(&mut self, entity_id: impl Into<usize>) -> Option<T> {
        self.storages.get_mut(&component_id!(T))
            .and_then(|storage| {
                let storage = (storage as &mut dyn Any).downcast_mut::<SparseSet<T>>().unwrap();
                storage.remove(entity_id.into())
            })
    }

    pub fn get_storage<T: Component>(&self) -> Option<&SparseSet<T>> {
        self.storages.get(&component_id!(T))
            .and_then(|storage| (storage as &dyn Any).downcast_ref::<SparseSet<T>>())
    }

    pub fn get_storage_mut<T: Component>(&mut self) -> Option<&mut SparseSet<T>> {
        self.storages.get_mut(&component_id!(T))
            .and_then(|storage| (storage as &mut dyn Any).downcast_mut::<SparseSet<T>>())
    }

    pub fn max_entity_id(&self) -> usize {
        self.max_entity_id
    }

    pub fn query<F: QueryFilter>(&self) -> Query<F> {
        Query::new(self)
    }

    pub fn remove_all(&mut self, entity_id: impl Into<usize>) {
        let entity_id = entity_id.into();
        for storage in self.storages.values_mut() {
            storage.remove(entity_id);
        }
    }
}
