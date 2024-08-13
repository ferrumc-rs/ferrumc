use crate::ecs::component::{Component, ComponentStorage};

/// A builder for creating and configuring entities in an Entity-Component-System architecture.
pub struct EntityBuilder<'a> {
    entity_id: usize,
    component_storage: &'a ComponentStorage,
}
impl<'a> EntityBuilder<'a> {
    /// Creates a new `EntityBuilder` instance.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - A unique identifier for the entity being built.
    /// * `component_storage` - A reference to the `ComponentStorage` where components will be stored.
    pub fn new(entity_id: impl Into<usize>, component_storage: &'a ComponentStorage) -> Self {
        let entity_id = entity_id.into();
        EntityBuilder {
            entity_id,
            component_storage,
        }
    }

    /// Adds a component to the entity being built.
    ///
    /// # Arguments
    ///
    /// * `component` - An instance of a type that implements the `DynamicComponent` trait.
    ///
    /// # Returns
    ///
    /// The `EntityBuilder` instance, allowing for method chaining.
    pub fn with<T: Component>(self, component: T) -> Self {
        self.component_storage.insert(self.entity_id, component);
        self
    }

    /// Finalizes the entity creation process.
    ///
    /// # Returns
    ///
    /// The `entity_id` of the built entity.
    pub fn build(self) -> usize {
        self.entity_id
    }
}
