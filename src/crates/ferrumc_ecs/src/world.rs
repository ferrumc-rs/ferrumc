use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;

type Entity = u32;

#[derive(Default)]
pub struct World {
    entities: Vec<Entity>,
    components: HashMap<TypeId, ComponentVec>,
}

impl World {
    pub fn new_entity(&mut self) -> Entity {
        let new_entity: Entity = self.entities.len() as Entity;
        self.entities.push(new_entity);
        new_entity
    }
}

struct ComponentVec {
    // Entity is the index of the entity in the world
    data: HashMap<Entity, Box<dyn Any>>,
    component_type: TypeId,
}

impl ComponentVec {
    fn new<T: 'static>() -> Self {
        ComponentVec {
            data: HashMap::new(),
            component_type: TypeId::of::<T>(),
        }
    }

    fn insert<T: 'static>(&mut self, entity: Entity, component: T) {
        self.data.insert(entity, Box::new(component));
    }

    fn get<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.data.get(&entity)?.downcast_ref::<T>()
    }

    fn get_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.data.get_mut(&entity)?.downcast_mut::<T>()
    }
}

impl World {
    pub fn add_component<T: 'static + Debug>(&mut self, entity: Entity, component: T) {
        println!("Adding component {:?} to {entity}", component);

        let type_id = TypeId::of::<T>();

        self.components
            .entry(type_id)
            .or_insert_with(|| ComponentVec::new::<T>())
            .insert(entity, component);
    }
    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id)?.get(entity)
    }
    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components.get_mut(&type_id)?.get_mut(entity)
    }

    pub fn get_component_data_values<T: 'static>(&self) -> Vec<&T> {
        let type_id = TypeId::of::<T>();
        let empty_bindings: Vec<T> = Vec::new();

        self.components.get(&type_id).map(|component_vec| {
            component_vec.data.values().map(|boxed| {
                boxed.downcast_ref::<T>().unwrap()
            }).collect()
        }).unwrap_or(Vec::new())
    }
    pub fn get_component_data<T: 'static>(&self) -> Option<&ComponentVec> {
        let type_id = TypeId::of::<T>();

        self.components.get(&type_id)
    }
}