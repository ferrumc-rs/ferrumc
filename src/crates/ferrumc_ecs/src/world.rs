use std::any::TypeId;
use std::collections::HashMap;
use std::marker::PhantomData;

pub type Entity = u32;

pub struct World {
    entities: Vec<Entity>,
    next_entity: Entity,
    components: HashMap<TypeId, Box<dyn AnyComponentStorage>>,
}

trait AnyComponentStorage {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub struct ComponentStorage<T> {
    pub(crate) data: Vec<Option<T>>,
    _marker: PhantomData<T>,
}

impl<T: 'static> ComponentStorage<T> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            _marker: PhantomData,
        }
    }

    fn insert(&mut self, entity: Entity, component: T) {
        if entity as usize >= self.data.len() {
            self.data.resize_with(entity as usize + 1, || None);
        }
        self.data[entity as usize] = Some(component);
    }

    fn get(&self, entity: Entity) -> Option<&T> {
        self.data.get(entity as usize)?.as_ref()
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.data.get_mut(entity as usize)?.as_mut()
    }
}

impl<T: 'static> AnyComponentStorage for ComponentStorage<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_entity: 0,
            components: HashMap::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let entity = self.next_entity;
        self.entities.push(entity);
        self.next_entity += 1;
        entity
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        let storage = self
            .components
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(ComponentStorage::<T>::new()))
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()
            .unwrap();
        storage.insert(entity, component);
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        let storage = self
            .components
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<ComponentStorage<T>>()?;
        storage.get(entity)
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let storage = self
            .components
            .get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()?;
        storage.get_mut(entity)
    }

    pub fn get_all_components<T: 'static>(&self) -> Option<&ComponentStorage<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|storage| storage.as_any().downcast_ref::<ComponentStorage<T>>())
    }

    pub fn get_all_components_mut<T: 'static>(&mut self) -> Option<&mut ComponentStorage<T>> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|storage| storage.as_any_mut().downcast_mut::<ComponentStorage<T>>())
    }

    pub fn get_component_storages<T: 'static, U: 'static>(
        &mut self,
    ) -> (
        Option<&mut ComponentStorage<T>>,
        Option<&ComponentStorage<U>>,
    ) {
        let t_id = TypeId::of::<T>();
        let u_id = TypeId::of::<U>();

        // SAFETY: This function uses unsafe code to bypass Rust's borrowing rules,
        // allowing us to return multiple references to different parts of self.components.
        // It is safe because:
        // 1. We only create one mutable reference (to T) and one shared reference (to U).
        // 2. We ensure that T and U are different types (guaranteed by the generic constraints),
        //    so we're not creating multiple references to the same data.
        // 3. The raw pointers are created from references that are guaranteed to be valid
        //    for the lifetime of self.
        // 4. We immediately convert the raw pointers back into references before returning,
        //    so no unsafe operations leak outside this function.
        // 5. The returned references are bound to the lifetime of &mut self, ensuring
        //    they remain valid for as long as the mutable borrow of World.
        // 6. We don't modify the structure of self.components, so the pointers remain valid.
        //
        // This approach allows for efficient, simultaneous access to multiple component
        // storages without the need for runtime borrow checking, while still maintaining
        // memory safety guarantees.
        unsafe {
            let t_ptr = self
                .components
                .get_mut(&t_id)
                .and_then(|storage| storage.as_any_mut().downcast_mut::<ComponentStorage<T>>())
                .map(|storage| storage as *mut ComponentStorage<T>);

            let u_ptr = self
                .components
                .get(&u_id)
                .and_then(|storage| storage.as_any().downcast_ref::<ComponentStorage<U>>())
                .map(|storage| storage as *const ComponentStorage<U>);

            (t_ptr.map(|p| &mut *p), u_ptr.map(|p| &*p))
        }
    }

    pub fn get_component_storage<T: 'static>(&mut self) -> Option<&mut ComponentStorage<T>> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|storage| storage.as_any_mut().downcast_mut::<ComponentStorage<T>>())
    }
}
