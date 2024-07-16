use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub trait Component: 'static {}

// An entity is essentially just a unique identifier.
pub type Entity = u32;

pub struct World {
    entities: Vec<Entity>,
    pub(crate) components: HashMap<TypeId, Box<dyn AnyComponentStorage>>,
}

/// A trait object that allows us to store any component storage type.
/// This is necessary because we want to store different component storage types in the same map.
pub trait AnyComponentStorage: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A storage for a specific component type.
/// T: The component type.
pub struct ComponentStorage<T: 'static> {
    pub(crate) data: HashMap<Entity, T>,
}

impl <T: Component> AnyComponentStorage for ComponentStorage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl World{
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let entity = self.entities.len() as Entity;
        self.entities.push(entity);
        entity
    }

    pub fn add_components<T: Component>(&mut self, entity: Entity, component: T) {
        let storage = self.components
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(ComponentStorage::<T>::new()))
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()
            // Couldn't possibly fail, since we just inserted the storage.
            .unwrap();

        storage.insert(entity, component);
    }

    pub fn get_component<T: Component>(&self, entity: &Entity) -> Option<&T> {
        self.components.get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<ComponentStorage<T>>()?
            .get(entity)
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: &Entity) -> Option<&mut T> {
        self.components.get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()?
            .get_mut(entity)
    }

}

impl<T: 'static> ComponentStorage<T> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, entity: Entity, component: T) {
        self.data.insert(entity, component);
    }

    fn get(&self, entity: &Entity) -> Option<&T> {
        self.data.get(entity)
    }

    fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.data.get_mut(entity)
    }
}

