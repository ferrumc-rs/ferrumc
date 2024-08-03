use std::marker::PhantomData;
use crate::component::{ComponentRef, ComponentStorage, DynamicComponent, Position};
use crate::entity::EntityManager;

pub trait QueryItem<'a> : 'a {
    type Item;
    async fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item>;
}

// Usage: &T ; &mut T => To get the component

impl <'a, T: DynamicComponent> QueryItem<'a> for &'a T {
    type Item = ComponentRef<'a, T>;

    async fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item> {
        storage.get::<T>(entity_id).await
    }
}

pub struct Query<'a, Q: QueryItem<'a>> {
    entity_manager: &'a EntityManager,
    component_storage: &'a ComponentStorage,
    _marker: PhantomData<Q>,
}

impl<'a, Q: QueryItem<'a>> Query<'a, Q> {
    pub fn new(entity_manager: &'a EntityManager, component_storage: &'a ComponentStorage) -> Self {
        Self {
            entity_manager,
            component_storage,
            _marker: PhantomData,
        }
    }

    // pub async fn iter(&self) -> Vec<Q::Item> {
    pub async fn iter(&self) -> impl Iterator<Item = (usize, Q::Item)> + '_ {
        let max_entity_id = self.entity_manager.len();
        (0..=max_entity_id).filter_map(|entity_id| {
            async {
                Q::fetch(entity_id, self.component_storage).await.map(|item| (entity_id, item))
            }
        })
    }

}

#[tokio::test]
async fn test_immut_query() {
    let storage = ComponentStorage::new();
    storage.insert(0usize, Position { x: 0.0, y: 0.0 });
    let component = <&Position as QueryItem>::fetch(0usize, &storage).await;
    assert!(component.is_some());
    assert_eq!(component.unwrap().x, 0.0);
}