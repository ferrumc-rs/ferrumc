use crate::components::storage::{Component, ComponentRef, ComponentRefMut};
use crate::components::ComponentManager;
use crate::entities::Entity;
use crate::ECSResult;

#[allow(async_fn_in_trait)]
pub trait QueryItem {
    type Item<'a>;

    fn fetch<'a>(
        entity: Entity,
        storage: &ComponentManager,
    ) -> ECSResult<Self::Item<'a>>;

    /*fn entities(
        storage: &ComponentManager,
    ) -> Vec<Entity>;*/
    fn entities(
        storage: &ComponentManager
    ) -> Vec<Entity>;
}
impl<T: Component> QueryItem for &T {
    type Item<'a> = ComponentRef<'a, T>;

    fn fetch<'a>(
        entity: Entity,
        storage: &ComponentManager,
    ) -> ECSResult<Self::Item<'a>> {
        storage.get(entity)
    }

    fn entities(
        storage: &ComponentManager,
    ) -> Vec<Entity> {
        storage.get_entities_with::<T>()
    }
}
impl<T: Component> QueryItem for &mut T {
    type Item<'a> = ComponentRefMut<'a, T>;

    fn fetch<'a>(
        entity: Entity,
        storage: &ComponentManager,
    ) -> ECSResult<Self::Item<'a>> {
        storage.get_mut(entity)
    }

    fn entities(
        storage: &ComponentManager,
    ) -> Vec<Entity> {
        storage.get_entities_with::<T>()
    }
}

pub struct Query<'a, Q: QueryItem> {
    component_storage: &'a ComponentManager,
    entities: Vec<Entity>,
    _marker: std::marker::PhantomData<Q>,
}

impl<Q: QueryItem> Clone for Query<'_, Q> {
    fn clone(&self) -> Self {
        //! Clones the query, and re-calculates the entities
        Self {
            component_storage: self.component_storage,
            entities: Q::entities(self.component_storage),
            _marker: std::marker::PhantomData,
        }
    }
}


impl<'a, Q: QueryItem> Query<'a, Q> {
    pub fn new(component_storage: &'a ComponentManager) -> Self {
        Self {
            component_storage,
            entities: Q::entities(component_storage),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    pub fn into_entities(self) -> Vec<Entity> {
        self.entities
    }
}


mod iter_impl {
    use super::*;
    use rayon::prelude::*;

    impl<'a, Q: QueryItem> Iterator for Query<'a, Q>
    {
        type Item = (Entity, Q::Item<'a>);

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(entity) = self.entities.pop() {
                let Ok(item) = Q::fetch(entity, self.component_storage) else {
                    continue;
                };
                return Some((entity, item));
            }
            None
        }
    }

    impl<'a, Q> ParallelIterator for Query<'a, Q>
    where
        Q: QueryItem + Send,
        Q::Item<'a>: Send,
    {
        type Item = Q::Item<'a>;

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
        {
            self.entities
                .into_par_iter()
                .filter_map(|entity| Q::fetch(entity, self.component_storage).ok())
                .drive_unindexed(consumer)
        }
    }
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

            fn fetch<'a>(
                entity: Entity,
                storage: &ComponentManager
            ) -> ECSResult<Self::Item<'a>> {
                Ok(($($T::fetch(entity, storage)?,)*))
            }

            fn entities(
                storage: &ComponentManager,
            ) -> Vec<Entity> {
                let entities = vec![$($T::entities(storage)),*];

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