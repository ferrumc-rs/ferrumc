use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::components::Component;

pub type Entity = u32;

/// Basically a storage for components of a specific type. <br/>
/// <b>data</b>: A vector of boxed components.
/// <b>component_type</b>: The type of the component.
pub struct ComponentVec {
    /// Basically a pointer to a heap-allocated object.
    data: Vec<Box<dyn Any>>,
    component_type: TypeId,
}

impl ComponentVec {
    pub fn new<T: 'static>() -> Self {
        ComponentVec {
            data: Vec::new(),
            component_type: TypeId::of::<T>(),
        }
    }

    pub fn push<T: 'static>(&mut self, component: T) {
        self.data.push(Box::new(component));
    }

    pub fn get<T: 'static>(&self, index: usize) -> Option<&T> {
        self.data.get(index)?.downcast_ref()
    }

    pub fn get_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)?.downcast_mut()
    }
}

pub struct World {
    entities: Vec<Entity>,
    components: HashMap<TypeId, ComponentVec>,
    entity_to_index: HashMap<Entity, usize>,
    next_entity_id: Entity,
}

impl World {
    pub fn new() -> Self {
        World {
            entities: Vec::new(),
            components: HashMap::new(),
            entity_to_index: HashMap::new(),
            next_entity_id: 0,
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let entity = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(entity);
        let index = self.entities.len() - 1;
        self.entity_to_index.insert(entity, index);
        entity
    }

    pub fn add_component<T: Component + Default>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        let index = *self.entity_to_index.get(&entity).unwrap();

        self.components
            .entry(type_id)
            .or_insert_with(|| ComponentVec::new::<T>())
            .push(component);

        while self.components.get(&type_id).unwrap().data.len() <= index {
            self.components.get_mut(&type_id).unwrap().push(T::default());
        }
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        let index = *self.entity_to_index.get(&entity)?;
        self.components.get(&type_id)?.get(index)
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        let index = *self.entity_to_index.get(&entity)?;
        self.components.get_mut(&type_id)?.get_mut(index)
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
}