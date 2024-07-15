use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;

pub type Entity = u32;

#[derive(Default)]
pub struct World {
    entities: Vec<Entity>,
    components: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

pub trait ComponentStorage {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> ComponentStorage for HashMap<Entity, RefCell<T>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_entity(&mut self) -> Entity {
        let entity = self.entities.len() as Entity;
        self.entities.push(entity);
        entity
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        self.components
            .entry(type_id)
            .or_insert_with(|| Box::new(HashMap::<Entity, RefCell<T>>::new()))
            .as_any_mut()
            .downcast_mut::<HashMap<Entity, RefCell<T>>>()
            .unwrap()
            .insert(entity, RefCell::new(component));
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<std::cell::Ref<T>> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)?
            .as_any()
            .downcast_ref::<HashMap<Entity, RefCell<T>>>()?
            .get(&entity)
            .map(|rc| rc.borrow())
    }

    pub fn get_component_mut<T: 'static>(&self, entity: Entity) -> Option<std::cell::RefMut<T>> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)?
            .as_any()
            .downcast_ref::<HashMap<Entity, RefCell<T>>>()?
            .get(&entity)
            .map(|rc| rc.borrow_mut())
    }

    pub fn get_all_components<T: 'static>(&self) -> Option<&HashMap<Entity, RefCell<T>>> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)?
            .as_any()
            .downcast_ref::<HashMap<Entity, RefCell<T>>>()
    }
}