use crate::components::{Position, Velocity};

pub type Entity = u64;

pub struct ComponentArray<T> {
    data: Vec<Option<T>>,
}

impl<T> ComponentArray<T> {
    fn new() -> Self {
        ComponentArray { data: Vec::new() }
    }

    fn insert(&mut self, index: usize, component: T) {
        if index >= self.data.len() {
            self.data.resize_with(index + 1, || None);
        }
        self.data[index] = Some(component);
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index).and_then(|opt| opt.as_ref())
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index).and_then(|opt| opt.as_mut())
    }

    fn get_all<'a>(&mut self) -> &'a Vec<Option<T>> {
        &self.data
    }
}

macro_rules! define_world {
    ($($component:ident),*) => {
        #[derive(Default)]
        pub struct World {
            entities: Vec<Entity>,
            next_entity_id: Entity,
            $(
                $component: ComponentArray<$component>,
            )*
        }

        impl World {
            pub fn new() -> Self {
                World::default()
            }

            pub fn new_entity(&mut self) -> Entity {
                let entity = self.next_entity_id;
                self.next_entity_id += 1;
                self.entities.push(entity);
                entity
            }

            $(
                pub fn add_component_$component(&mut self, entity: Entity, component: $component) {
                    let index = self.entities.iter().position(|&e| e == entity).unwrap();
                    self.$component.insert(index, component);
                }

                pub fn get_component_$component(&self, entity: Entity) -> Option<&$component> {
                    let index = self.entities.iter().position(|&e| e == entity)?;
                    self.$component.get(index)
                }

                pub fn get_component_mut_$component(&mut self, entity: Entity) -> Option<&mut $component> {
                    let index = self.entities.iter().position(|&e| e == entity)?;
                    self.$component.get_mut(index)
                }

                pub fn get_all_component_$component<'a>(&mut self) -> &'a Vec<$component> {
                    $component.get_all()
                }
            )*
        }
    }
}

// Use the macro to define the World struct with these components
define_world!(Position,Velocity);
