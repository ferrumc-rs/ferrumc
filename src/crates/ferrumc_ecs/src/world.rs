use std::marker::PhantomData;
use std::path::Component;
use crate::components::Position;

pub type Entity = u64;

pub struct ComponentVec<T> {
    data: Vec<Option<T>>,
    _marker: PhantomData<T>,
}

impl<T> ComponentVec<T> {
    fn new() -> Self {
        ComponentVec {
            data: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn insert(&mut self, index: usize, component: T) {
        if index >= self.data.len() {
            self.data.resize_with(index + 1, || None);
        }
        self.data[index] = Some(component);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index).and_then(|opt| opt.as_ref())
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index).and_then(|opt| opt.as_mut())
    }

    pub fn get_all(&self) -> &Vec<Option<T>> {
        &self.data
    }
}

macro_rules! types {
    ($($something:ident),*) => {
        pub struct World {
            entities: Vec<Entity>,
            next_entity_id: Entity,
            $(
                $something: ComponentVec<$something>,
            )+
        }

        impl World {
            $(
                pub fn [<get_ $something>](&self) {
                    println!("{}", &self.$something);
                }
            )+
        }
    };
}

types!(Position, World);

impl World {
    pub fn new() -> Self {
        World {
            entities: Vec::new(),
            next_entity_id: 0,
            ..Default::default()
           /* $(
            $ component: ComponentVec::new(),
            ) **/
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let entity = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(entity);
        entity
    }
}