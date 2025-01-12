use crate::components::storage::{Component, ComponentRef, ComponentRefMut};
use crate::components::ComponentManager;
use crate::entities::Entity;
use crate::ECSResult;

#[allow(async_fn_in_trait)]
pub trait QueryItem {
    type Item<'a>;

    async fn fetch<'a>(entity: Entity, storage: &ComponentManager) -> ECSResult<Self::Item<'a>>;
    async fn entities(storage: &ComponentManager) -> Vec<Entity>;
}
impl<T: Component> QueryItem for &T {
    type Item<'a> = ComponentRef<'a, T>;

    async fn fetch<'a>(entity: Entity, storage: &ComponentManager) -> ECSResult<Self::Item<'a>> {
        storage.get(entity).await
    }

    async fn entities(storage: &ComponentManager) -> Vec<Entity> {
        storage.get_entities_with::<T>().await
    }
}
impl<T: Component> QueryItem for &mut T {
    type Item<'a> = ComponentRefMut<'a, T>;

    async fn fetch<'a>(entity: Entity, storage: &ComponentManager) -> ECSResult<Self::Item<'a>> {
        storage.get_mut(entity).await
    }

    async fn entities(storage: &ComponentManager) -> Vec<Entity> {
        storage.get_entities_with::<T>().await
    }
}

/// The backbone of the ECS, the query. You will love it, hate it, dream about it, and cry about it.
///
/// The query is a way to iterate over entities that have a specific set of components. This is the
/// primary way to interact with the ECS.
///
/// There are 2 main ways to use the query: iterating the components directly, or iterating
/// the entities and looking up the components.
///
/// Generally you will want to iterate the components directly, as it is more efficient. It does
/// however lock the component for the duration of the iteration, so if you need to do something
/// that could take a while (database access, network access, etc) you should iterate the entities
/// and look up the components as needed to ensure you aren't holding the locks longer than needed.
///
/// ### Example
/// Single component query:
/// ```
/// # use tokio_test;
/// # tokio_test::block_on(async {
/// use ferrumc_ecs::Universe;
///
/// // Generally this will be in the global state
/// let universe = Universe::new();
///
/// struct Position {
///     x: f32,
///     y: f32,
/// }
///
/// universe.builder().with(
///     Position { x: 0.0, y: 0.0 }
/// ).await.unwrap().build();
///
/// let mut query = universe.query::<&Position>().await;
///
/// while let Some((entity, position)) = query.next().await {
///    println!("Entity: {}, Position: ({}, {})", entity, position.x, position.y);
/// }
/// # });
///
/// ```
/// Multiple component query:
/// ```
/// # use tokio_test;
/// # tokio_test::block_on(async {
/// use ferrumc_ecs::Universe;
///  
/// let universe = Universe::new();
///
/// struct Position {
///     x: f32,
///     y: f32,
/// }
///
/// struct Velocity {
///     x: f32,
///     y: f32,
/// }
///
/// universe.builder().with(
///     Position { x: 0.0, y: 0.0 }
/// ).await.unwrap().with(
///     Velocity { x: 1.0, y: 1.0 }
/// ).await.unwrap().build();
///
/// let mut query = universe.query::<(&Position, &Velocity)>().await;
///
/// while let Some((entity, (position, velocity))) = query.next().await {
///    println!("Entity: {}, Position: ({}, {}), Velocity: ({}, {})", entity, position.x, position.y, velocity.x, velocity.y);
/// }
/// # });
/// ```
///
/// Look up components as needed:
/// ```
/// # use tokio_test;
/// # tokio_test::block_on(async {
/// use ferrumc_ecs::Universe;
///
/// let universe = Universe::new();
///
/// struct Position {
///    x: f32,
///   y: f32,
/// }
///
/// universe.builder().with(
///    Position { x: 0.0, y: 0.0 }
/// ).await.unwrap().build();
///
/// let mut query = universe.query::<&Position>().await.into_entities();
///
/// for entity in query {
///     let (mut x, mut y) = (100f32, 100f32);
///     {
///         // Takes a lock on the component
///         let position = universe.get::<Position>(entity).await.unwrap();
///         (x, y) = (position.x, position.y);
///     } // Lock is released here
///    // Do something that takes a while
///    println!("Entity: {}, Position: ({}, {})", entity, x, y);   
/// }
/// # });
/// ```
///
/// An important note is that you have to query the ecs with a reference to the component you want to query,
/// So`universe.query::<Position>()` will not work, you have to use `universe.query::<&Position>()`.
pub struct Query<'a, Q: QueryItem> {
    component_storage: &'a ComponentManager,
    entities: Vec<Entity>,
    _marker: std::marker::PhantomData<Q>,
}

impl<Q: QueryItem> Clone for Query<'_, Q> {
    fn clone(&self) -> Self {
        //! Clones the query
        Self {
            component_storage: self.component_storage,
            entities: self.entities.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, Q: QueryItem> Query<'a, Q> {
    pub async fn new(component_storage: &'a ComponentManager) -> Self {
        Self {
            component_storage,
            entities: Q::entities(component_storage).await,
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns a reference to the entities in the query
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    /// Converts the query into a vector of entities
    pub fn into_entities(self) -> Vec<Entity> {
        self.entities
    }
}

mod iter_impl {
    use super::*;

    impl<'a, Q: QueryItem> Query<'a, Q> {
        pub type Item = (Entity, Q::Item<'a>);

        pub async fn next(&mut self) -> Option<Self::Item> {
            while let Some(entity) = self.entities.pop() {
                let Ok(item) = Q::fetch(entity, self.component_storage).await else {
                    continue;
                };
                return Some((entity, item));
            }
            None
        }
    }

    // Removed due to async_iterator not supporting parallel iterators
    // At some point I'll make a proper async parallel iterator for this

    // impl<'a, Q> ParallelIterator for Query<'a, Q>
    // where
    //     Q: QueryItem + Send,
    //     Q::Item<'a>: Send,
    // {
    //     type Item = (Entity, Q::Item<'a>);
    //
    //     fn drive_unindexed<C>(self, consumer: C) -> C::Result
    //     where
    //         C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    //     {
    //         self.entities
    //             .into_par_iter()
    //             .filter_map(|entity| {
    //                 let item = Q::fetch(entity, self.component_storage);
    //                 item.ok().map(|item| (entity, item))
    //             })
    //             .drive_unindexed(consumer)
    //     }
    // }
}

mod multi_impl {
    use super::*;
    macro_rules! impl_query_item_tuple {
    ($($T: ident), *) => {
        impl<$($T),*> QueryItem for ($($T,)*)
        where
            $($T: QueryItem,)*
        {
            type Item<'a> = ($($T::Item<'a>,)*);

            async fn fetch<'a>(
                entity: Entity,
                storage: &ComponentManager
            ) -> ECSResult<Self::Item<'a>> {
                Ok(($($T::fetch(entity, storage).await?,)*))
            }

            async fn entities(
                storage: &ComponentManager,
            ) -> Vec<Entity> {
                let entities = vec![$($T::entities(storage).await),*];

                find_common_elements(entities)
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

    /// Find the common elements in a vector of vectors
    /// Uses smallest vector as comparator, making it somewhat efficient.
    fn find_common_elements(mut vecs: Vec<Vec<Entity>>) -> Vec<Entity> {
        if vecs.is_empty() {
            return vec![];
        }

        // Sort all vectors and the vector of vectors itself
        for vec in &mut vecs {
            vec.sort_unstable();
        }
        vecs.sort_by_key(|v| v.len());

        let shortest = &vecs[0];
        let mut common = Vec::new();

        for &num in shortest {
            if vecs.iter().skip(1).all(|v| v.binary_search(&num).is_ok()) {
                common.push(num);
            }
        }

        common
    }
}
