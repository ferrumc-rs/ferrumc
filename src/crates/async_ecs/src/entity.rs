#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub(crate) id: u32,
    pub(crate) generation: u32,
}

impl Into<usize> for Entity {
    fn into(self) -> usize {
        self.id as usize
    }
}

pub struct EntityManager {
    pub generations: Vec<u32>,
    free_ids: Vec<u32>,
}


impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            generations: Vec::new(),
            free_ids: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        if let Some(id) = self.free_ids.pop() {
            let generation = self.generations[id as usize];
            Entity { id, generation }
        } else {
            let id = self.generations.len() as u32;
            self.generations.push(0);
            Entity { id, generation: 0 }
        }
    }

    pub fn delete_entity(&mut self, entity: Entity) -> bool {
        if (entity.id as usize) < self.generations.len() &&
            self.generations[entity.id as usize] == entity.generation {
            self.generations[entity.id as usize] += 1;
            self.free_ids.push(entity.id);
            true
        } else {
            false
        }
    }

    pub fn entity_exists(&self, entity: Entity) -> bool {
        (entity.id as usize) < self.generations.len() &&
            self.generations[entity.id as usize] == entity.generation
    }

    pub fn entity_count(&self) -> usize {
        self.generations.len() - self.free_ids.len()
    }

    pub fn clear(&mut self) {
        self.generations.clear();
        self.free_ids.clear();
    }

    pub fn get_entity(&self, id: u32) -> Option<Entity> {
        if (id as usize) < self.generations.len() {
            Some(Entity { id, generation: self.generations[id as usize] })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_entity() {
        let mut manager = EntityManager::new();
        let entity1 = manager.create_entity();
        let entity2 = manager.create_entity();

        assert_eq!(entity1.id, 0);
        assert_eq!(entity2.id, 1);
        assert_eq!(manager.entity_count(), 2);
    }

    #[test]
    fn test_delete_entity() {
        let mut manager = EntityManager::new();
        let entity = manager.create_entity();

        assert!(manager.delete_entity(entity));
        assert_eq!(manager.entity_count(), 0);
        assert!(!manager.delete_entity(entity)); // Trying to delete non-existent entity
    }

    #[test]
    fn test_entity_exists() {
        let mut manager = EntityManager::new();
        let entity = manager.create_entity();

        assert!(manager.entity_exists(entity));
        assert!(!manager.entity_exists(Entity { id: entity.id + 1, generation: 0 }));
    }

    #[test]
    fn test_clear() {
        let mut manager = EntityManager::new();
        manager.create_entity();
        manager.create_entity();
        manager.clear();

        assert_eq!(manager.entity_count(), 0);
        let new_entity = manager.create_entity();
        assert_eq!(new_entity.id, 0); // Ensure IDs are reset
    }

    #[test]
    fn test_create_after_delete() {
        let mut manager = EntityManager::new();
        let entity1 = manager.create_entity();
        manager.delete_entity(entity1);
        let entity2 = manager.create_entity();

        assert_eq!(entity1.id, entity2.id);
        assert_ne!(entity1.generation, entity2.generation);
        assert_eq!(manager.entity_count(), 1);
    }

    #[test]
    fn test_generational_index() {
        let mut manager = EntityManager::new();
        let entity1 = manager.create_entity();
        manager.delete_entity(entity1);
        let entity2 = manager.create_entity();

        assert!(!manager.entity_exists(entity1));
        assert!(manager.entity_exists(entity2));
    }

    #[test]
    fn test_reuse_deleted_ids() {
        let mut manager = EntityManager::new();
        let e1 = manager.create_entity();
        let e2 = manager.create_entity();
        let _e3 = manager.create_entity();

        manager.delete_entity(e2);
        manager.delete_entity(e1);

        let e4 = manager.create_entity();
        let e5 = manager.create_entity();

        assert_eq!(e4.id, e1.id);
        assert_eq!(e5.id, e2.id);
        assert_ne!(e4.generation, e1.generation);
        assert_ne!(e5.generation, e2.generation);
        assert_eq!(manager.entity_count(), 3);
    }
}