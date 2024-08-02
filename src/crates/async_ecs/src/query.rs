/*use std::marker::PhantomData;
use tokio::sync::RwLockWriteGuard;
use crate::component::{ComponentRef, ComponentRefMut, ComponentStorage, DynamicComponent};
use crate::entity::EntityManager;


pub trait QueryItem<'a>: 'a {
    type Item;
    async fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item>;
}


impl<'a, T: DynamicComponent> QueryItem<'a> for &'a T {
    type Item = ComponentRef<'a, T>;

    async fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item> {
        storage.get::<T>(entity_id).await
    }
}
impl<'a, T: DynamicComponent> QueryItem<'a> for &'a mut T {
    type Item = ComponentRefMut<'a, T>;

    async fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item> {
        storage.get_mut::<T>(entity_id).await
    }
}





// Example implementation of QueryItem for a tuple (A, B)

impl <'a, A: QueryItem<'a>, B: QueryItem<'a>> QueryItem<'a> for (A, B) {
    type Item = (A::Item, B::Item);

    async fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item> {
        let entity_id: usize = entity_id.into();
        match (A::fetch(entity_id, storage).await, B::fetch(entity_id, storage).await) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
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


    pub async fn iter(&self) -> impl Iterator<Item=(usize, Q::Item)> + 'a {
        let entity_manager = self.entity_manager;
        let component_storage = self.component_storage;

        let mut components = vec![];

        for entity_id in 0..entity_manager.len() {
            let item = Q::fetch(entity_id, component_storage).await;
            if let Some(item) = item {
                components.push((entity_id, item));
            }
        }

        components.into_iter()
    }

}



mod tests {
    use crate::component::{ComponentStorage, Position, Velocity};
    use crate::entity::EntityManager;

    #[tokio::test]
    async fn test_basic_usage() {
        let mut  entity_manager = EntityManager::new();
        let component_storage = ComponentStorage::new();

        let entity = entity_manager.create_entity();
        let position = Position { x: 0.0, y: 0.0 };
        let velocity = Velocity { x: 1.0, y: 5.0 };
        component_storage.insert(entity, position);
        component_storage.insert(entity, velocity);

        let entity = entity_manager.create_entity();
        let position = Position { x: 2.0, y: 3.0 };
        component_storage.insert(entity, position);

        let entity = entity_manager.create_entity();
        let position = Position { x: 4.0, y: 5.0 };
        let velocity = Velocity { x: 6.0, y: 7.0 };
        component_storage.insert(entity, position);

        let query = crate::query::Query::<(&mut Position, &Velocity)>::new(&entity_manager, &component_storage);

        query.iter().await.for_each(|(id, (position, velocity))| {
            println!("{id}: {:?} {:?}", *position, *velocity)
        });
    }
}*/