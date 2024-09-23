use std::collections::HashMap;

use crate::entities::Entity;

pub struct SparseSet<T> {
    /// The actual components. Aligned perfectly for cache efficiency.
    components: Vec<T>,
    /// The index of the entity in the entities vector.
    indices: HashMap<Entity, usize>,
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        SparseSet {
            components: Vec::new(),
            indices: HashMap::new(),
        }
    }

    /// Insert a component for a specific Entity.
    /// If the component already exists, it will be replaced.
    pub fn insert(&mut self, entity: Entity, component: T) {
        if let Some(index) = self.indices.get(&entity) {
            // Update the component if it already exists.
            self.components[*index] = component;
            return;
        }
        let index = self.components.len();
        self.components.push(component);
        self.indices.insert(entity, index);
    }

    /// Get a (immutable) reference to a component for a specific Entity.
    /// Returns None if the component does not exist.
    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.indices.get(&entity).map(|index| &self.components[*index])
    }

    /// Get a mutable reference to a component for a specific Entity.
    /// Returns None if the component does not exist.
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.indices.get(&entity).map(|index| &mut self.components[*index])
    }

    /// Remove a component for a specific Entity.
    /// Returns the removed component if it existed.
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        if let Some(index) = self.indices.remove(&entity) {
            Some(self.components.remove(index))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::EntityManager;

    use super::*;

    #[test]
    fn insert_get() {
        let mut entity_manager = EntityManager::new();
        let mut sparse_set = SparseSet::new();
        let entity =  entity_manager.create_entity();
        let component = 42;
        sparse_set.insert(entity, component);
        assert_eq!(sparse_set.get(entity), Some(&42));
    }

    #[test]
    fn insert_get_mut() {
        let mut entity_manager = EntityManager::new();
        let mut sparse_set = SparseSet::new();
        let entity =  entity_manager.create_entity();
        let component = 42;
        sparse_set.insert(entity, component);
        let component = sparse_set.get_mut(entity).unwrap();
        *component = 43;
        assert_eq!(sparse_set.get(entity), Some(&43));
    }

    #[test]
    fn remove() {
        let mut entity_manager = EntityManager::new();
        let mut sparse_set = SparseSet::new();
        let entity =  entity_manager.create_entity();
        let component = 42;
        sparse_set.insert(entity, component);
        assert_eq!(sparse_set.remove(entity), Some(42));
        assert_eq!(sparse_set.get(entity), None);
    }

    #[test]
    fn remove_non_existent() {
        let mut entity_manager = EntityManager::new();
        let mut sparse_set: SparseSet<usize> = SparseSet::new();
        let entity =  entity_manager.create_entity();
        assert_eq!(sparse_set.remove(entity), None);
    }
}