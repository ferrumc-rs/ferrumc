use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::entity::EntityId;

pub trait Component: Any {}

pub struct ComponentManager {
    components: HashMap<(EntityId, TypeId), Box<dyn Any>>
}

impl ComponentManager {
    pub fn new () -> Self {
        Self {
            components: HashMap::new()
        }
    }

    pub fn add_component<T: Component>(&mut self, entity: EntityId, component: T)  {
        
    }
}