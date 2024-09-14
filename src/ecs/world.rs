use crate::ecs::component::{Component, ComponentRef, ComponentRefMut, ComponentStorage};
use crate::ecs::entity::EntityManager;
use crate::ecs::error::Error;
use crate::ecs::helpers::entity_builder::EntityBuilder;
use crate::ecs::query::Query;

use crate::utils::prelude::*;

/// <p style="color:#4CAF50;font-size:1.2em;font-weight:bold;">The World struct</p>
///
/// The `World` struct is the central point of the ECS (Entity Component System).
/// It manages entities and components, and provides an interface for querying and manipulating the game state.
///
/// <p style="color:#607D8B;font-size:1.1em;font-weight:bold;">Additional Examples</p>
///
/// <p style="color:#795548;">Creating and using a complete game world:</p>
///
/// ```ignore
/// # use ferrumc::ecs::world::World;
/// # use ferrumc::utils::encoding::position::Position;
/// # use ferrumc::utils::encoding::velocity::Velocity;
/// # use ferrumc::utils::components::player::Player;
/// # async fn doc() {
/// // Create a new world
/// let mut world = World::new();
///
/// // Create some entities
/// let player = world.create_entity()
///     .await
///     .with(Position { x: 0, y: 0, z: 0 })
///     .with(Velocity { x: 0, y: 0, z: 0 })
///     .with(Player { username: "Hero".to_string(), uuid: 1234 })
///     .build();
///
/// // Run a game loop
/// loop {
///     // Update player position
///     let mut query = world.query::<(&mut Position, &Velocity)>();
///     for (_, (mut pos, vel)) in query.iter().await {
///         pos.x += vel.x;
///         pos.y += vel.y;
///     }
///
///     // Check for collisions
///     let mut collision_query = world.query::<(&Position, Option<&Player>)>();
///     for (entity, (pos, player)) in collision_query.iter().await {
///         // Handle collisions...
///     }
///
///     // Break the loop when game is over
///     // break;
/// }
/// # }
/// ```
///
/// This example demonstrates creating a world, adding entities with various components,
/// and running a simple game loop with position updates and collision detection.
pub struct World {
    entity_manager: EntityManager,
    component_storage: ComponentStorage,
}

impl World {
    /// <p style="color:#2196F3;">Creates a new World instance</p>
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use ferrumc::ecs::world::World;
    /// let world = World::new();
    /// ```
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_storage: ComponentStorage::new(),
        }
    }

    /// <p style="color:#FFC107;">Creates a new entity and returns an EntityBuilder</p>
    ///
    /// Use this method to create and configure new entities in the world.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use ferrumc::ecs::world::World;
    /// # use ferrumc::utils::encoding::position::Position;
    /// # use ferrumc::utils::encoding::velocity::Velocity;
    /// # async fn doc() {
    /// let mut world = World::new();
    /// let entity = world.create_entity()
    ///     .await
    ///     .with(Position { x: 0, y: 0, z: 1 })
    ///     .with(Velocity { x: 1, y: 1, z: 1 })
    ///     .build();
    /// # }
    /// ```
    pub async fn create_entity(&self) -> EntityBuilder {
        let entity = self.entity_manager.create_entity().await;
        EntityBuilder::new(entity, &self.component_storage)
    }

    pub async fn delete_entity(&self, entity_id: impl TryInto<usize>) -> Result<()> {
        let entity_id = entity_id.try_into().map_err(|_| Error::ConversionError)?;

        if !self.entity_manager.delete_entity(entity_id).await {
            return Err(Error::EntityNotFound(entity_id))?;
        }

        self.component_storage.remove_all(entity_id);

        Ok(())
    }

    /// <p style="color:#E91E63;">Creates a new query for components</p>
    ///
    /// Use this method to query entities with specific components.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use ferrumc::ecs::world::World;
    /// # use ferrumc::utils::encoding::position::Position;
    /// # use ferrumc::utils::encoding::velocity::Velocity;
    /// # async fn doc() {
    /// let world = World::new();
    ///
    /// // Query for all entities with Position and Velocity components
    /// let query = world.query::<(&Position, &Velocity)>();
    ///
    /// // Iterate over the query results
    /// for (entity_id, (position, velocity)) in query.iter().await {
    ///     println!("Entity {} at {:?} moving with velocity {:?}", entity_id, position, velocity);
    /// }
    ///
    /// // Query with mutable components
    /// let mut update_query = world.query::<(&mut Position, &Velocity)>();
    /// for (_, (mut position, velocity)) in update_query.iter().await {
    ///     position.x += velocity.x;
    ///     position.y += velocity.y;
    /// }
    /// # }
    /// ```
    pub fn query<Q>(&self) -> Query<Q>
    where
        Q: crate::ecs::query::QueryItem,
    {
        Query::<Q>::new(&self.entity_manager, &self.component_storage)
    }

    pub async fn get_component<T: Component>(
        &self,
        entity_id: impl TryInto<usize>,
    ) -> Result<ComponentRef<'_, T>> {
        let entity_id = entity_id.try_into().map_err(|_| Error::ConversionError)?;
        self.get_component_storage().get::<T>(entity_id).await
    }
    pub async fn get_component_mut<T: Component>(
        &self,
        entity_id: impl TryInto<usize>,
    ) -> Result<ComponentRefMut<'_, T>> {
        let entity_id = entity_id.try_into().map_err(|_| Error::ConversionError)?;
        self.get_component_storage().get_mut::<T>(entity_id).await
    }

    /// <p style="color:#9C27B0;">Returns a reference to the ComponentStorage</p>
    ///
    /// This method provides direct access to the component storage.
    /// <p style="color:#FF5722;"><strong>Note:</strong> Use with caution, as it bypasses the usual query system.</p>
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use ferrumc::ecs::world::World;
    /// # use ferrumc::utils::encoding::position::Position;
    /// # let entity_id = 0;
    /// let world = World::new();
    /// let component_storage = world.get_component_storage();
    ///
    /// // Directly insert a component for an entity
    /// component_storage.insert(entity_id, Position { x: 10, y: 20, z: 10 });
    /// ```
    pub fn get_component_storage(&self) -> &ComponentStorage {
        &self.component_storage
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

mod multiple_components {
    use super::*;

    impl World {
        pub async fn get_components<T>(
            &self,
            entity_id: impl TryInto<usize>,
        ) -> Result<T::Output<'_>>
        where
            T: GetComponents,
        {
            let entity_id = entity_id.try_into().map_err(|_| Error::ConversionError)?;
            T::get_components(self, entity_id).await
        }
    }

    pub trait GetComponents: Sized {
        type Output<'a>;
        #[allow(async_fn_in_trait)]
        async fn get_components(world: &World, entity_id: usize) -> Result<Self::Output<'_>>;
    }

    macro_rules! impl_get_components_for_tuple {
        ($($T:ident),*) => {
            impl<$($T: Component),*> GetComponents for ($($T,)*) {
                type Output<'a> = ($(ComponentRef<'a, $T>,)*);

                async fn get_components(world: &World, entity_id: usize) -> Result<Self::Output<'_>> {
                    Ok(($(world.get_component::<$T>(entity_id).await?,)*))
                }
            }
        };
    }

    // Implement for tuples of different sizes
    impl_get_components_for_tuple!(A);
    impl_get_components_for_tuple!(A, B);
    impl_get_components_for_tuple!(A, B, C);
    impl_get_components_for_tuple!(A, B, C, D);
    impl_get_components_for_tuple!(A, B, C, D, E);

    #[cfg(test)]
    mod tests {
        use crate::ecs::world::World;
        use crate::utils::components::rotation::Rotation;
        use crate::utils::encoding::position::Position;

        #[tokio::test]
        async fn test_get_components() {
            let world = World::new();
            let entity = world
                .create_entity()
                .await
                .with(Position::new(0, 0, 0))
                .with(Rotation::new(0f32, 0f32))
                .build();

            let result = world.get_components::<(Position, Rotation)>(entity).await;
            assert!(result.is_ok());
            let (pos, rot) = result.unwrap();
            assert_eq!(pos.x, 0);
            assert_eq!(rot.pitch, 0.0);
        }
    }
}
