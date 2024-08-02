use std::marker::PhantomData;
use crate::component::{ComponentRef, ComponentStorage, DynamicComponent};
use crate::entity::{Entity, EntityManager};

pub trait QueryItem<'a>: 'a {
    type Item;
    fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item>;
}

impl<'a, T: DynamicComponent> QueryItem<'a> for T {
    type Item = ComponentRef<'a, T>;

    fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item> {
        storage.get::<T>(entity_id)
    }
}
/*impl<'a, T: DynamicComponent> QueryItem<'a> for T {
    type Item = ComponentRef<'a, T>;

    fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item> {
        storage.get::<T>(entity_id)
    }
}
*/

pub struct Query<'a, Q: QueryItem<'a>> {
    entity_manager: &'a EntityManager,
    component_storage: &'a ComponentStorage,
    _marker: PhantomData<Q>
}

impl<'a, Q: QueryItem<'a>> Query<'a, Q> {
    pub fn new(entity_manager: &'a EntityManager, component_storage: &'a ComponentStorage) -> Self {
        Self {
            entity_manager,
            component_storage,
            _marker: PhantomData
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, Q::Item)> + 'a {
        self.entity_manager.generations.iter().enumerate()
            .filter_map(|(id, &generation)| {
                Q::fetch(id, self.component_storage)
                    .map(|item| (id, item))
            })
    }
}


macro_rules! impl_query_item_tuple {
    // ex: impl_query_item_tuple!(A, B, C)
    ($($ty:ident),*) => {
        // With lifetimes
        impl<'a, $($ty: QueryItem<'a>),*> QueryItem<'a> for ($($ty,)*) {
            type Item = ($($ty::Item,)*);

            fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item> {
                let entity_id: usize = entity_id.into();
                Some(($($ty::fetch(entity_id, storage)?,)*))
            }
        }
    };
}

impl_query_item_tuple!(A);
impl_query_item_tuple!(A, B);
impl_query_item_tuple!(A, B, C);
impl_query_item_tuple!(A, B, C, D);
impl_query_item_tuple!(A, B, C, D, E);


#[cfg(test)]
mod tests {
    use crate::component::{ComponentStorage, Position};
    use crate::entity::EntityManager;
    use crate::query::Query;

    #[test]
    fn test_basic_query() {
        let mut em = EntityManager::new();
        let component_storage = ComponentStorage::new();

        let entity = em.create_entity();
        let position = Position { x: 1.5, y: 3.7 };
        component_storage.insert(entity, position);

        let entity2 = em.create_entity();
        let position2 = Position { x: 0f32, y: 1f32 };
        component_storage.insert(entity2, position2);


        let query = Query::<Position>::new(&em, &component_storage);

        query.iter().for_each(|(id, mut position)| {
            position.x += 1.0;
            println!("Entity {} has position ({}, {})", id, position.x, position.y);
        });
    }

}