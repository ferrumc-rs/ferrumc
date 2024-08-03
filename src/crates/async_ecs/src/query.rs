use std::marker::PhantomData;

use crate::component::{ComponentRef, ComponentRefMut, ComponentStorage, DynamicComponent, Position, Velocity};
use crate::entity::EntityManager;

pub trait QueryItem {
    type Item<'a>;
    async fn fetch<'a>(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item<'a>>;
}

// Usage: &T ; &mut T => To get the component

impl<T: DynamicComponent> QueryItem for &T {
    type Item<'a> = ComponentRef<'a, T>;

    async fn fetch<'a>(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item<'a>> {
        storage.get::<T>(entity_id).await
    }
}
impl<T: DynamicComponent> QueryItem for &mut T {
    type Item<'a> = ComponentRefMut<'a, T>;
    
    async fn fetch<'a>(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item<'a>> {
        storage.get_mut::<T>(entity_id).await
    }
}

pub struct Query<'a, Q: QueryItem> {
    entity_manager: &'a EntityManager,
    component_storage: &'a ComponentStorage,
    _marker: PhantomData<Q>,
}

impl<'a, Q: QueryItem> Query<'a, Q> {
    pub fn new(entity_manager: &'a EntityManager, component_storage: &'a ComponentStorage) -> Self {
        Self {
            entity_manager,
            component_storage,
            _marker: PhantomData,
        }
    }

    pub async fn iter(&'a self) -> impl Iterator<Item = (usize, Q::Item<'a>)> + 'a {
        let max_entity_id = self.entity_manager.len();
        let mut results = vec![];

        for entity_id in 0..=max_entity_id {
            if let Some(item) = Q::fetch(entity_id, self.component_storage).await {
                results.push((entity_id, item));
            }
        }

        results.into_iter()
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


#[tokio::test]
async fn test_immut_query() {
    let storage = ComponentStorage::new();
    storage.insert(0usize, Position { x: 0.0, y: 0.0 });
    let component = <&Position as QueryItem>::fetch(0usize, &storage).await;
    assert!(component.is_some());
    assert_eq!(component.unwrap().x, 0.0);
}

#[tokio::test]
async fn test_iter() {
    let storage = ComponentStorage::new();
    let mut entity_manager = EntityManager::new();

    for _ in 0..=2 {
        entity_manager.create_entity();
    }

    storage.insert(0usize, Position { x: 0.0, y: 0.0 });
    storage.insert(0usize, Velocity { x: 2.0, y: 0.0 });
    storage.insert(1usize, Position { x: 1.0, y: 1.0 });
    storage.insert(1usize, Velocity { x: 2.0, y: 2.0 });


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