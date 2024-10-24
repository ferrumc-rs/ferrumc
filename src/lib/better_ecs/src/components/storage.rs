use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use crossbeam::sync::ShardedLock;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::entity::EntityId;
use super::{Result, ECSError};

pub trait Component: 'static {}

impl<T: 'static> Component for T {}

pub struct ComponentSparseSet<C: Component> {
    // Map of <EntityId, Index>; Using a RwLock since we will be doing a lot of reads than writes (inserts)
    lookup: RwLock<HashMap<usize, usize>>,
    // Vector of components. Each component is wrapped in a RwLock to allow for concurrent interior mutability
    data: RwLock<Vec<RwLock<C>>>,
}

impl<C: Component> Default for ComponentSparseSet<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Component> ComponentSparseSet<T> {
    pub fn new() -> Self {
        Self {
            lookup: RwLock::new(HashMap::new()),
            data: RwLock::new(Vec::new()),
        }
    }
    pub fn with(entity_id: EntityId, component: T) -> Result<Self> {
        let new_instance = Self::new();

        new_instance.insert(entity_id, component)?;


        Ok(new_instance)
    }

    pub fn insert(&self, entity_id: EntityId, component: T) -> Result<()> {
        let mut lookup = self.lookup.write();
        let mut data = self.data.write();

        let index = data.len();
        data.push(RwLock::new(component));
        lookup.insert(entity_id, index);

        Ok(())
    }


    pub fn get<'a, 'b>(&'a self, entity_id: usize) -> Result<ComponentRef<'b, T>>
    where
        'a: 'b, // Self outlives 'b therefore should be safe. And valid till 'b is dropped.
    {
        let lookup = self.lookup.read();
        let data: RwLockReadGuard<'b, Vec<RwLock<T>>> = self.data.read();
        let data = unsafe {
            std::mem::transmute::<RwLockReadGuard<'_, Vec<RwLock<T>>>, RwLockReadGuard<'b, Vec<RwLock<T>>>>(data)
        };

        let index = lookup.get(&entity_id).ok_or(ECSError::ComponentRetrievalError)?;

            /*let try_read = data[*index].try_read();
            if try_read.is_none() {
                return Err(ECSError::ComponentIsLocked);
            }
            drop(try_read);*/

        let Some(read_guard) = data[*index].try_read() else {
            return Err(ECSError::ComponentIsLocked);
        };

        let component = data[*index].data_ptr();

        let read_guard = unsafe {
            // make the read_guard lifetime to be 'b
            std::mem::transmute::<RwLockReadGuard<'_, T>, RwLockReadGuard<'b, T>>(read_guard)
        };

        let component_ref = ComponentRef {
            component,
            read_guard,
            lifetime: std::marker::PhantomData,
        };
        Ok(component_ref)
    }

    pub fn get_mut<'a, 'b>(&'a self, entity_id: usize) -> Result<ComponentRefMut<'b, T>>
    where
        'a: 'b, // Self outlives 'b therefore should be safe. And valid till 'b is dropped.
    {
        let lookup = self.lookup.read();
        let data: RwLockReadGuard<'b, Vec<RwLock<T>>> = self.data.read();
        let data = unsafe {
            std::mem::transmute::<RwLockReadGuard<'_, Vec<RwLock<T>>>, RwLockReadGuard<'b, Vec<RwLock<T>>>>(data)
        };

        let index = lookup.get(&entity_id).ok_or(ECSError::ComponentRetrievalError)?;
/*
        {
            let try_write = data[*index].try_write();
            if try_write.is_none() {
                return Err(ECSError::ComponentIsLocked);
            }
            drop(try_write);
        }*/

        let Some(write_guard) = data[*index].try_write() else {
            return Err(ECSError::ComponentIsLocked);
        };

        let write_guard = unsafe {
            // make the write_guard lifetime to be 'b
            std::mem::transmute::<RwLockWriteGuard<'_, T>, RwLockWriteGuard<'b, T>>(write_guard)
        };
        
        let component = data[*index].data_ptr();

        let component_ref = ComponentRefMut {
            component,
            write_guard,
            lifetime: std::marker::PhantomData,
        };

        Ok(component_ref)
    }
}

pub struct ComponentRef<'a, T> {
    component: *mut T,
    read_guard: RwLockReadGuard<'a, T>,
    lifetime: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Deref for ComponentRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.component }
    }
}

pub struct ComponentRefMut<'a, T> {
    component: *mut T,
    write_guard: RwLockWriteGuard<'a, T>,
    lifetime: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Deref for ComponentRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.component }
    }
}

impl<'a, T> DerefMut for ComponentRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.component }
    }
}