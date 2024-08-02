use std::any::TypeId;
use std::sync::RwLock;
use dashmap::DashMap;
use crate::helpers::sparse_set::SparseSet;

// Component trait
pub trait Component: 'static + Send + Sync {}

#[derive()]
pub struct ComponentStorage {
    storages: DashMap<TypeId, SparseSet<RwLock<Box<dyn Component>>>>,
}

impl ComponentStorage {
    pub fn new() -> Self {
        Self {
            storages: DashMap::new(),
        }
    }

    pub fn insert<T: Component>(&self, entity_id: usize, component: T) {
        let type_id = TypeId::of::<T>();
        println!("Inserting component: {:?}, for entity: {}\ncomponent data: {:#?}", type_id, entity_id, component);
    }
}


#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Component for Position {}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
impl Component for Velocity {}