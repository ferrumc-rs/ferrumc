use crate::components::ComponentManager;
use crate::components::storage::{Component, ComponentRef, ComponentRefMut};
use crate::entities::{Entity, EntityBuilder, EntityManager};
use crate::query::{Query, QueryItem};

pub mod errors;

pub mod components;
pub mod entities;
pub mod query;

#[cfg(test)]
mod tests;
pub type ECSResult<T> = Result<T, errors::ECSError>;


/// The main struct that holds all the ECS data.
/// It's called the universe because I didn't want to name it 'World'.
/// Since it may be confused with the Minecraft world.
pub struct Universe {
    entities: EntityManager,
    components: ComponentManager,
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

impl Universe {
    pub fn new() -> Self {
        Self {
            entities: EntityManager::new(),
            components: ComponentManager::new(),
        }
    }

    pub fn create_entity(&self) -> Entity {
        self.entities.create_entity()
    }

    pub fn builder(&self) -> EntityBuilder {
        self.entities.builder(&self.components)
    }

    pub fn add_component<T: Component>(&self, entity: Entity, component: T) -> ECSResult<()> {
        self.components.insert(entity, component)
    }
    
    pub fn remove_component<T: Component>(&self, entity: Entity) -> ECSResult<()> {
        self.components.remove::<T>(entity)
    }
    
    pub fn remove_all_components(&self, entity: Entity) -> ECSResult<()> {
        self.components.remove_all_components(entity)
    }

    pub fn get<'a, T: Component>(&self, entity: Entity) -> ECSResult<ComponentRef<'a, T>> {
        self.components.get::<T>(entity)
    }
    pub fn get_mut<'a, T: Component>(&self, entity: Entity) -> ECSResult<ComponentRefMut<'a, T>> {
        self.components.get_mut::<T>(entity)
    }

    pub fn query<Q: QueryItem>(&self) -> Query<Q> {
        Query::new(&self.components)
    }
    
    pub fn get_component_manager(&self) -> &ComponentManager {
        &self.components
    }
}