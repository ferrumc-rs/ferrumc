use crate::component::ComponentStorage;
use crate::entity::EntityManager;
use crate::helpers::entity_builder::EntityBuilder;
use crate::query::Query;

pub struct World {
    entity_manager: EntityManager,
    component_storage: ComponentStorage,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_storage: ComponentStorage::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity = self.entity_manager.create_entity();
        EntityBuilder::new(entity, &self.component_storage)
    }

    pub fn query<Q>(&self) -> Query<Q>
    where
        Q: crate::query::QueryItem,
    {
        Query::<Q>::new(&self.entity_manager, &self.component_storage)
    }

    pub fn get_component_storage(&self) -> &ComponentStorage {
        &self.component_storage
    }
}
