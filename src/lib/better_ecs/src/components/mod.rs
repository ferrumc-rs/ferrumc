use std::collections::HashMap;
use parking_lot::RwLock;

trait Component {}

impl<T> Component for T {}

pub struct ComponentSparseSet<C: Component> {
    // Map of <EntityId, Index>
    lookup: RwLock<HashMap<usize, usize>>,
    data: Vec<C>
}



impl<T: Component> ComponentSparseSet<T> {
    pub fn add(&mut self, entity: usize, component: T) {
        let index = self.data.len();
        self.lookup.insert(entity, index);
        self.data.push(component);
    }
}