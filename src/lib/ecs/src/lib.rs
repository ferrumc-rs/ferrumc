#![allow(incomplete_features)]
#![feature(async_iterator)]
#![feature(random)]
#![feature(inherent_associated_types)]
//! Ferrumc's ECS.
//!
//! The easiest way to use it is with queries, check out the docs for [query::Query] for more info.
use crate::components::storage::{Component, ComponentRef, ComponentRefMut};
use crate::components::ComponentManager;
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

    /// Creates a new entity.
    ///
    /// ### Example
    ///
    /// ```
    /// # use tokio_test;
    /// # use ferrumc_ecs::Universe;
    /// # tokio_test::block_on(async {
    ///
    /// let universe = Universe::new();
    ///
    /// let entity = universe.create_entity();
    ///
    /// # });
    /// ```
    pub fn create_entity(&self) -> Entity {
        self.entities.create_entity()
    }

    /// The primary way to create entities.
    ///
    /// This is the best way to create entities, as it will ensure that the entity is created correctly.
    ///
    /// ### Example
    ///
    /// ```
    /// # use tokio_test;
    /// # use ferrumc_ecs::Universe;
    /// # tokio_test::block_on(async {
    ///
    /// let universe = Universe::new();
    ///
    /// struct MyComponent {
    ///    value: i32,
    /// }
    ///
    /// let entity = universe.builder().with(MyComponent { value: 42 }).await.unwrap().build();
    ///
    /// let retrieved_component = universe.get::<MyComponent>(entity).await.unwrap();
    ///
    /// assert_eq!(retrieved_component.value, 42);
    ///
    /// # });
    /// ```
    pub fn builder(&self) -> EntityBuilder {
        self.entities.builder(&self.components)
    }

    /// Adds a component to an entity.
    ///
    /// ### Example
    ///
    /// ```
    /// # use tokio_test;
    /// # use ferrumc_ecs::Universe;
    /// # tokio_test::block_on(async {
    ///
    /// let universe = Universe::new();
    ///
    /// struct MyComponent {
    ///    value: i32,
    /// }
    ///
    /// let entity = universe.create_entity();
    ///
    /// universe.add_component(entity, MyComponent { value: 42 }).await.unwrap();
    ///
    /// let retrieved_component = universe.get::<MyComponent>(entity).await.unwrap();
    ///
    /// # });
    pub async fn add_component<T: Component>(
        &self,
        entity: Entity,
        component: T,
    ) -> ECSResult<&Self> {
        self.components.insert(entity, component).await?;
        Ok(self)
    }

    /// Removes a component from an entity.
    ///
    /// ### Example
    /// ```
    /// # use tokio_test;
    /// # use ferrumc_ecs::Universe;
    /// # tokio_test::block_on(async {
    ///
    /// let universe = Universe::new();
    ///
    /// struct MyComponent {
    ///   value: i32,
    /// }
    ///
    /// let entity = universe.create_entity();
    ///
    /// universe.add_component(entity, MyComponent { value: 42 }).await.unwrap();
    ///
    /// universe.remove_component::<MyComponent>(entity).await.unwrap();
    ///
    /// });
    /// ```
    pub async fn remove_component<T: Component>(&self, entity: Entity) -> ECSResult<()> {
        self.components.remove::<T>(entity).await
    }

    /// Removes all components from an entity.
    ///
    /// ### Example
    /// ```
    /// # use tokio_test;
    /// # use ferrumc_ecs::Universe;
    /// # tokio_test::block_on(async {
    ///
    /// let universe = Universe::new();
    ///
    /// struct MyComponent {
    ///    value: i32,
    /// }
    ///
    /// let entity = universe.create_entity();
    ///
    /// universe.add_component(entity, MyComponent { value: 42 }).await.unwrap();
    ///
    /// universe.remove_all_components(entity).await.unwrap();
    ///
    /// # });
    /// ```
    pub async fn remove_all_components(&self, entity: Entity) -> ECSResult<()> {
        self.components.remove_all_components(entity).await
    }

    /// Gets a component from the component manager
    ///
    /// ### Example
    ///
    /// ```
    /// # use tokio_test;
    /// # use ferrumc_ecs::Universe;
    /// # tokio_test::block_on(async {
    ///
    /// let universe = Universe::new();
    ///
    /// struct MyComponent {
    ///   value: i32,
    /// }
    ///
    /// let entity = universe.create_entity();
    ///
    /// universe.add_component(entity, MyComponent { value: 42 }).await.unwrap();
    ///
    /// let retrieved_component = universe.get::<MyComponent>(entity).await.unwrap();
    ///
    /// assert_eq!(retrieved_component.value, 42);
    ///
    /// # });
    /// ```
    pub async fn get<'a, T: Component>(&self, entity: Entity) -> ECSResult<ComponentRef<'a, T>> {
        self.components.get::<T>(entity).await
    }

    /// Gets a mutable reference to a component from the component manager
    ///
    /// ### Example
    ///
    /// ```
    /// # use tokio_test;
    /// # use ferrumc_ecs::Universe;
    /// # tokio_test::block_on(async {
    ///
    /// let universe = Universe::new();
    ///
    /// struct MyComponent {
    ///  value: i32,
    /// }
    ///
    /// let entity = universe.create_entity();
    ///
    /// universe.add_component(entity, MyComponent { value: 42 }).await.unwrap();
    /// {
    ///     let mut retrieved_component = universe.get_mut::<MyComponent>(entity).await.unwrap();
    ///
    ///     retrieved_component.value = 43;
    /// }
    /// let new_component = universe.get::<MyComponent>(entity).await.unwrap();
    ///
    /// assert_eq!(new_component.value, 43);
    ///
    /// # });
    /// ```
    pub async fn get_mut<'a, T: Component>(
        &self,
        entity: Entity,
    ) -> ECSResult<ComponentRefMut<'a, T>> {
        self.components.get_mut::<T>(entity).await
    }

    /// Queries the universe for components.
    ///
    /// Check out the [query::Query] struct for more info.
    pub async fn query<Q: QueryItem>(&self) -> Query<Q> {
        Query::new(&self.components).await
    }

    pub fn get_component_manager(&self) -> &ComponentManager {
        &self.components
    }
}
