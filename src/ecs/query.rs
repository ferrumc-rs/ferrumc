use std::marker::PhantomData;
use crate::ecs::component::{Component, ComponentRef, ComponentRefMut, ComponentStorage};
use crate::ecs::entity::EntityManager;

#[allow(async_fn_in_trait)]
/// Trait for items that can be queried in the ECS.
pub trait QueryItem {
    type Item<'a>;
    async fn fetch<'a>(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item<'a>>;
}

// Implement QueryItem for immutable references
impl<T: Component> QueryItem for &T {
    type Item<'a> = ComponentRef<'a, T>;

    async fn fetch<'a>(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item<'a>> {
        storage.get::<T>(entity_id).await
    }
}

// Implement QueryItem for mutable references
impl<T: Component> QueryItem for &mut T {
    type Item<'a> = ComponentRefMut<'a, T>;

    async fn fetch<'a>(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item<'a>> {
        storage.get_mut::<T>(entity_id).await
    }
}

/// Struct for querying components in the ECS.
pub struct Query<'a, Q: QueryItem> {
    entity_manager: &'a EntityManager,
    component_storage: &'a ComponentStorage,
    current_id: usize,
    _marker: PhantomData<Q>,
}

impl<'a, Q: QueryItem> Query<'a, Q> {
    /// Creates a new Query.
    ///
    /// # Examples
    ///
    /// ```
    /// let query = Query::<&Position>::new(&entity_manager, &component_storage);
    /// ```
    pub fn new(entity_manager: &'a EntityManager, component_storage: &'a ComponentStorage) -> Self {
        Self {
            entity_manager,
            component_storage,
            current_id: 0,
            _marker: PhantomData,
        }
    }

    /// Returns an iterator over the query results.
    ///
    /// # Examples
    ///
    /// ```
    /// // Query for positions
    /// let query = Query::<&Position>::new(&entity_manager, &component_storage);
    /// for (entity_id, position) in query.iter().await {
    ///     println!("Entity {} is at position {:?}", entity_id, position);
    /// }
    ///
    /// // Query for mutable positions and velocities
    /// let query = Query::<(&mut Position, &Velocity)>::new(&entity_manager, &component_storage);
    /// for (entity_id, (mut position, velocity)) in query.iter().await {
    ///     position.x += velocity.x;
    ///     position.y += velocity.y;
    /// }
    /// ```
    pub async fn iter(&'a self) -> impl Iterator<Item=(usize, Q::Item<'a>)> + 'a {
        let max_entity_id = self.entity_manager.len().await;
        let mut results = vec![];

        for entity_id in 0..=max_entity_id {
            if let Some(item) = Q::fetch(entity_id, self.component_storage).await {
                results.push((entity_id, item));
            }
        }

        results.into_iter()
    }

    /// Returns the next query result.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut query = Query::<(&Position, &Velocity)>::new(&entity_manager, &component_storage);
    /// while let Some((entity_id, (position, velocity))) = query.next().await {
    ///     println!("Entity {} is at {:?} moving at {:?}", entity_id, position, velocity);
    /// }
    /// ```
    pub async fn next<'b>(&mut self) -> Option<(usize, Q::Item<'b>)>
    where
        'a: 'b, // 'a must outlive 'b
    {
        let max_entity_id = self.entity_manager.len().await;
        while self.current_id <= max_entity_id {
            if let Some(item) = Q::fetch(self.current_id, self.component_storage).await {
                let result = Some((self.current_id, item));
                self.current_id += 1;
                return result;
            }
            self.current_id += 1;
        }
        None
    }
}

// Macro to automatically generate tuples
macro_rules! impl_query_item_tuple {
    ($($T: ident), *) => {
        impl<$($T),*> QueryItem for ($($T,)*)
        where
            $($T: QueryItem,)*
        {
            type Item<'a> = ($($T::Item<'a>,)*);

            async fn fetch<'a>(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item<'a>> {
                let entity_id = entity_id.into();
                Some((
                    $(
                        $T::fetch(entity_id, storage).await?,
                    )*
                ))
            }
        }
    };
}

impl_query_item_tuple!(A);
impl_query_item_tuple!(A, B);
impl_query_item_tuple!(A, B, C);
impl_query_item_tuple!(A, B, C, D);
impl_query_item_tuple!(A, B, C, D, E);
impl_query_item_tuple!(A, B, C, D, E, F);

/// Examples of using the Query system
///
/// ```
/// // Example 1: Basic query for a single component
/// let query = Query::<&Position>::new(&entity_manager, &component_storage);
/// for (entity_id, position) in query.iter().await {
///     println!("Entity {} is at position {:?}", entity_id, position);
/// }
///
/// // Example 2: Query for multiple components
/// let query = Query::<(&Position, &Velocity)>::new(&entity_manager, &component_storage);
/// for (entity_id, (position, velocity)) in query.iter().await {
///     println!("Entity {} is at {:?} moving at {:?}", entity_id, position, velocity);
/// }
///
/// // Example 3: Query with mutable components
/// let query = Query::<(&mut Position, &Velocity)>::new(&entity_manager, &component_storage);
/// for (entity_id, (mut position, velocity)) in query.iter().await {
///     position.x += velocity.x;
///     position.y += velocity.y;
///     println!("Updated position of entity {} to {:?}", entity_id, position);
/// }
///
/// // Example 4: Combining mutable and immutable queries
/// let query = Query::<(&mut Health, &Position, &Attack)>::new(&entity_manager, &component_storage);
/// for (entity_id, (mut health, position, attack)) in query.iter().await {
///     if position.x < 0.0 {
///         health.current -= attack.damage;
///         println!("Entity {} took damage, new health: {}", entity_id, health.current);
///     }
/// }
///
/// // Example 5: Using next() for manual iteration
/// let mut query = Query::<&Position>::new(&entity_manager, &component_storage);
/// while let Some((entity_id, position)) = query.next().await {
///     println!("Found entity {} at position {:?}", entity_id, position);
/// }
/// ```

#[cfg(test)]
mod tests {
    use crate::ecs::component::ComponentStorage;
    use crate::ecs::entity::EntityManager;
    use crate::ecs::query::{Query, QueryItem};
    use crate::utils::encoding::position::Position;
    use crate::utils::encoding::velocity::Velocity;

    #[tokio::test]
    async fn test_immut_query() {
        let storage = ComponentStorage::new();
        storage.insert(0usize, Position { x: 0, y: 0, z: 0 });
        let component = <&Position as QueryItem>::fetch(0usize, &storage).await;
        assert!(component.is_some());
        assert_eq!(component.unwrap().x, 0);
    }

    #[tokio::test]
    async fn test_iter() {
        let storage = ComponentStorage::new();
        let mut entity_manager = EntityManager::new();

        for _ in 0..=2 {
            entity_manager.create_entity();
        }

        storage.insert(0usize, Position { x: 0,  y: 0, z: 0 });
        storage.insert(0usize, Velocity { x: 2, y: 0, z: 0 });
        storage.insert(1usize, Position { x: 1, y: 1 , z: 1 });
        storage.insert(1usize, Velocity { x: 2, y: 2 , z: 2 });

        let query = Query::<(&mut Position, &Velocity)>::new(&entity_manager, &storage);

        // System to update position (with velocity)
        for (entity_id, (mut pos, vel)) in query.iter().await {
            pos.x += vel.x;
            pos.y += vel.y;
            storage.remove::<Velocity>(entity_id);
        }

        // Log the results
        let query = Query::<&Position>::new(&entity_manager, &storage);
        for (entity_id, pos) in query.iter().await {
            println!("Entity {}: {:?}", entity_id, *pos);
        }
    }
}