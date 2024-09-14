use std::sync::Arc;

use tokio::sync::RwLock;

/// Represents an entity in the ECS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Entity {
    pub id: u32,
    pub generation: u32,
}

impl From<Entity> for usize {
    fn from(val: Entity) -> Self {
        val.id as usize
    }
}

/// Manages entity creation, deletion, and lifecycle.
pub struct EntityManager {
    inner: Arc<RwLock<EntityManagerInner>>,
}

struct EntityManagerInner {
    generations: Vec<u32>,
    free_ids: Vec<u32>,
}

impl EntityManager {
    /// Creates a new `EntityManager`.
    pub fn new() -> Self {
        EntityManager {
            inner: Arc::new(RwLock::new(EntityManagerInner {
                generations: Vec::new(),
                free_ids: Vec::new(),
            })),
        }
    }

    /// Creates a new entity and returns it.
    ///
    /// # Examples
    /// ```ignore
    /// let mut manager = EntityManager::new();
    /// let entity = manager.create_entity();
    /// ```
    pub async fn create_entity(&self) -> Entity {
        let mut inner = self.inner.write().await;
        if let Some(id) = inner.free_ids.pop() {
            let generation = inner.generations[id as usize];
            Entity { id, generation }
        } else {
            let id = inner.generations.len() as u32;
            inner.generations.push(0);
            Entity { id, generation: 0 }
        }
    }

    /// Deletes an entity.
    ///
    /// Returns `true` if the entity was successfully deleted, `false` otherwise.
    ///
    /// # Examples
    /// ```ignore
    /// let mut manager = EntityManager::new();
    /// let entity = manager.create_entity();
    /// assert!(manager.delete_entity(entity));
    /// ```
    pub async fn delete_entity(&self, entity: impl Into<usize>) -> bool {
        let entity = entity.into();
        let mut inner = self.inner.write().await;

        if inner.free_ids.contains(&(entity as u32)) {
            return false;
        }

        if entity < inner.generations.len() {
            inner.generations[entity] += 1;
            inner.free_ids.push(entity as u32);
            true
        } else {
            false
        }
    }

    /// Checks if an entity exists.
    ///
    /// # Examples
    /// ```ignore
    /// let mut manager = EntityManager::new();
    /// let entity = manager.create_entity();
    /// assert!(manager.entity_exists(entity));
    /// ```
    pub async fn entity_exists(&self, entity: Entity) -> bool {
        let inner = self.inner.read().await;
        (entity.id as usize) < inner.generations.len()
            && inner.generations[entity.id as usize] == entity.generation
    }

    /// Returns the number of active entities.
    pub async fn entity_count(&self) -> usize {
        let inner = self.inner.read().await;
        inner.generations.len() - inner.free_ids.len()
    }

    /// Removes all entities from the manager.
    pub async fn clear(&self) {
        let mut inner = self.inner.write().await;
        inner.generations.clear();
        inner.free_ids.clear();
    }

    /// Retrieves an entity by its ID.
    ///
    /// Returns `None` if the entity doesn't exist.
    ///
    /// # Examples
    /// ```ignore
    /// let mut manager = EntityManager::new();
    /// let entity = manager.create_entity();
    /// assert!(manager.get_entity(entity.id).is_some());
    /// ```
    pub async fn get_entity(&self, id: u32) -> Option<Entity> {
        let inner = self.inner.read().await;
        if (id as usize) < inner.generations.len() {
            Some(Entity {
                id,
                generation: inner.generations[id as usize],
            })
        } else {
            None
        }
    }

    /// Returns the total number of entity slots (including deleted entities).
    pub async fn len(&self) -> usize {
        let inner = self.inner.read().await;
        inner.generations.len()
    }

    /// Returns bool if total number of entity slots (including deleted entities) is empty.
    pub async fn is_empty(&self) -> bool {
        let inner = self.inner.read().await;
        inner.generations.is_empty()
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for EntityManager {
    fn clone(&self) -> Self {
        EntityManager {
            inner: Arc::clone(&self.inner),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_entity() {
        let manager = EntityManager::new();
        let entity1 = manager.create_entity().await;
        let entity2 = manager.create_entity().await;

        assert_eq!(entity1.id, 0);
        assert_eq!(entity2.id, 1);
        assert_eq!(manager.entity_count().await, 2);
    }

    #[tokio::test]
    async fn test_delete_entity() {
        let manager = EntityManager::new();
        let entity = manager.create_entity().await;

        assert!(manager.delete_entity(entity).await);
        assert_eq!(manager.entity_count().await, 0);
        assert!(!manager.delete_entity(entity).await); // Trying to delete non-existent entity
    }

    #[tokio::test]
    async fn test_entity_exists() {
        let manager = EntityManager::new();
        let entity = manager.create_entity().await;

        assert!(manager.entity_exists(entity).await);
        assert!(
            !manager
                .entity_exists(Entity {
                    id: entity.id + 1,
                    generation: 0
                })
                .await
        );
    }

    #[tokio::test]
    async fn test_clear() {
        let manager = EntityManager::new();
        manager.create_entity().await;
        manager.create_entity().await;
        manager.clear().await;

        assert_eq!(manager.entity_count().await, 0);
        let new_entity = manager.create_entity().await;
        assert_eq!(new_entity.id, 0); // Ensure IDs are reset
    }

    #[tokio::test]
    async fn test_create_after_delete() {
        let manager = EntityManager::new();
        let entity1 = manager.create_entity().await;
        manager.delete_entity(entity1).await;
        let entity2 = manager.create_entity().await;

        assert_eq!(entity1.id, entity2.id);
        assert_ne!(entity1.generation, entity2.generation);
        assert_eq!(manager.entity_count().await, 1);
    }

    #[tokio::test]
    async fn test_generational_index() {
        let manager = EntityManager::new();
        let entity1 = manager.create_entity().await;
        manager.delete_entity(entity1).await;
        let entity2 = manager.create_entity().await;

        assert!(!manager.entity_exists(entity1).await);
        assert!(manager.entity_exists(entity2).await);
    }

    #[tokio::test]
    async fn test_reuse_deleted_ids() {
        let manager = EntityManager::new();
        let e1 = manager.create_entity().await;
        let e2 = manager.create_entity().await;
        let _e3 = manager.create_entity().await;

        manager.delete_entity(e2).await;
        manager.delete_entity(e1).await;

        let e4 = manager.create_entity().await;
        let e5 = manager.create_entity().await;

        assert_eq!(e4.id, e1.id);
        assert_eq!(e5.id, e2.id);
        assert_ne!(e4.generation, e1.generation);
        assert_ne!(e5.generation, e2.generation);
        assert_eq!(manager.entity_count().await, 3);
    }
}
