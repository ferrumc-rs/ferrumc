use crate::component::{ComponentStorage, DynamicComponent};

pub struct EntityBuilder<'a> {
    entity_id: usize,
    component_storage: &'a ComponentStorage,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(entity_id: impl Into<usize>, component_storage: &'a ComponentStorage) -> Self {
        let entity_id = entity_id.into();
        EntityBuilder {
            entity_id,
            component_storage,
        }
    }

    pub fn with<T: DynamicComponent>(self, component: T) -> Self {
        self.component_storage.insert(self.entity_id, component);
        self
    }

    pub fn build(self) -> usize {
        self.entity_id
    }
}

