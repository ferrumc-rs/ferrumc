use std::marker::PhantomData;
use tokio::sync::RwLockWriteGuard;
use crate::component::{ComponentRef, ComponentRefMut, ComponentStorage, DynamicComponent};
use crate::entity::EntityManager;


pub trait QueryItem<'a>: 'a {
    type Item;
    async fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item>;
}

impl<'a, T: DynamicComponent> QueryItem<'a> for T {
    type Item = ComponentRef<'a, T>;

    async fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item> {
        storage.get::<T>(entity_id).await
    }
}
pub struct Query<'a, Q, M = ()> {
    entity_manager: &'a EntityManager,
    component_storage: &'a ComponentStorage,
    _marker: PhantomData<(Q, M)>,
}

// ===== Mutable version of Query =====
pub struct Mutable;
pub type QueryMut<'a, Q> = Query<'a, Q, Mutable>;

pub trait QueryItemMut<'a>: 'a {
    type Item;
    async fn fetch_mut(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item>;
}

impl<'a, T: DynamicComponent> QueryItemMut<'a> for T {
    type Item = ComponentRefMut<'a, T>;

    async fn fetch_mut(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item> {
        storage.get_mut::<T>(entity_id).await
    }
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

impl<'a, Q: QueryItemMut<'a>> Query<'a, Q, Mutable> {
    pub fn new_mut(entity_manager: &'a EntityManager, component_storage: &'a ComponentStorage) -> Self {
        Self {
            entity_manager,
            component_storage,
            _marker: PhantomData,
        }
    }

    pub async fn iter_mut(&self) -> impl Iterator<Item=(usize, Q::Item)> + 'a {
        let entity_manager = self.entity_manager;
        let component_storage = self.component_storage;

        let mut components = vec![];

        for entity_id in 0..entity_manager.len() {
            let item = Q::fetch_mut(entity_id, component_storage).await;
            if let Some(item) = item {
                components.push((entity_id, item));
            }
        }

        components.into_iter()
    }
}


/*macro_rules! impl_query_item_tuple {
    // ex: impl_query_item_tuple!(A, B, C)
    ($($ty:ident),*) => {
        // With lifetimes
        impl<'a, $($ty: QueryItem<'a>),*> QueryItem<'a> for ($($ty,)*) {
            type Item = ($($ty::Item,)*);

            async fn fetch(entity_id: impl Into<usize>, storage: &'a ComponentStorage) -> Option<Self::Item> {
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
*/

#[cfg(test)]
pub mod tests {
    use std::sync::OnceLock;
    use tokio::sync::RwLock;
    use crate::component::{ComponentStorage, Position, Velocity};
    use crate::entity::EntityManager;
    use crate::query::Query;

    /*fn GET_WORLD() -> &(EntityManager, ComponentStorage) {
                static static_world: OnceLock<(EntityManager, ComponentStorage)> = OnceLock::new();

                static_world.get_or_init(|| {
                    let entity_manager = EntityManager::new();
                    let component_storage = ComponentStorage::new();
                    (entity_manager, component_storage)
                })
            }*/
    type World = (RwLock<EntityManager>, ComponentStorage);

    #[allow(non_snake_case)]
    pub fn GET_WORLD() -> &'static World {
        static STATIC_WORLD: OnceLock<World> = OnceLock::new();

        STATIC_WORLD.get_or_init(|| {
            let entity_manager = EntityManager::new();
            let entity_manager = RwLock::new(entity_manager);
            let component_storage = ComponentStorage::new();

            (entity_manager, component_storage)
        })
    }

    #[tokio::test]
    async fn test_basic_query() {
        let (em, component_storage) = GET_WORLD();
        let mut em = em.write().await;

        let entity = em.create_entity();
        let position = Position { x: 0.0, y: 0.0 };
        let velocity = Velocity { x: 1.0, y: 5.0 };
        component_storage.insert(entity, position);
        component_storage.insert(entity, velocity);

        let entity2 = em.create_entity();
        let position2 = Position { x: 0f32, y: 1f32 };
        component_storage.insert(entity2, position2);
        // QUERY
        let query = Query::<Position>::new(&em, &component_storage);

        query.iter().await.for_each(|(id, position)| {
            println!("{id}: {:?}", *position)
        });

/*
        query.iter().for_each(|(_, (mut position, velocity))| {
            position.x += velocity.x;
            position.y += velocity.y;
        });

        let query = Query::<Position>::new(&em, &component_storage);

        query.iter().for_each(|(id, position)| {
            println!("{id}: {:?}", *position)
        });*/
    }
}