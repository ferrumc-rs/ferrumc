use std::marker::PhantomData;
use crate::component::{ComponentRef, ComponentStorage, DynamicComponent};
use crate::entity::{EntityManager};

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
            .filter_map(|(id, &_)| {
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


// #[cfg(test)]
pub mod tests {
    use std::sync::{OnceLock};
    use parking_lot::RwLock;
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

    #[test]
    fn test_basic_query() {
        let (em, component_storage) = GET_WORLD();
        let mut em = em.write();

        let entity = em.create_entity();
        let position = Position { x: 0.0, y: 0.0 };
        let velocity = Velocity { x: 1.0, y: 5.0 };
        component_storage.insert(entity, position);
        component_storage.insert(entity, velocity);

        let entity2 = em.create_entity();
        let position2 = Position { x: 0f32, y: 1f32 };
        component_storage.insert(entity2, position2);
        // QUERY
        let query = Query::<(Position, Velocity)>::new(&em, &component_storage);

        query.iter().for_each(|(_, (mut position, velocity))| {
            position.x += velocity.x;
            position.y += velocity.y;
        });

        let query = Query::<Position>::new(&em, &component_storage);

        query.iter().for_each(|(id, position)| {
            println!("{id}: {:?}", *position)
        });
    }

}