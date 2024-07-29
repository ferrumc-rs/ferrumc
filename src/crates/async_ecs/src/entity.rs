#[derive(Debug, PartialEq, Eq)]
pub struct EntityId(u32);

impl Into<EntityId> for u32 {
    fn into(self) -> EntityId {
        EntityId(self)
    }
}

pub struct EntityManager {
    next_id: u32,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager { next_id: 0 }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let new_entity_id = self.next_id;

        self.next_id += 1;

        new_entity_id.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::EntityManager;

    #[test]
    fn create_entity() {
        let mut entity_manager = EntityManager::new();

        let entity1 = entity_manager.create_entity();
        let entity2  = entity_manager.create_entity();

        assert_ne!(entity1, entity2);
    }
}