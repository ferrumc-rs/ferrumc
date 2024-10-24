use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::{ECSResult};
use crate::errors::ECSError;

pub trait Component: 'static + Send + Sync {}

unsafe impl<T> Send for ComponentRef<'_, T> where T: Component {}
unsafe impl<T> Send for ComponentRefMut<'_, T> where T: Component {}
impl<T: 'static + Send + Sync> Component for T {}

pub struct ComponentSparseSet<C: Component> {
    // Map of <Entity Id, Index>; Using a RwLock since we will be doing a lot of reads than writes (inserts)
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
    pub fn with(entity_id: usize, component: T) -> ECSResult<Self> {
        let new_instance = Self::new();

        new_instance.insert(entity_id, component)?;


        Ok(new_instance)
    }

    pub fn insert(&self, entity_id: usize, component: T) -> ECSResult<()> {
        let mut lookup = self.lookup.write();
        let mut data = self.data.write();

        let index = data.len();
        data.push(RwLock::new(component));
        lookup.insert(entity_id, index);

        Ok(())
    }


    pub fn get<'a, 'b>(&'a self, entity_id: usize) -> ECSResult<ComponentRef<'b, T>>
    where
        'a: 'b, // Self outlives 'b therefore should be safe. And valid till 'b is dropped.
    {
        let lookup = self.lookup.read();
        let data: RwLockReadGuard<'b, Vec<RwLock<T>>> = self.data.read();
        let data = unsafe {
            std::mem::transmute::<RwLockReadGuard<'_, Vec<RwLock<T>>>, RwLockReadGuard<'b, Vec<RwLock<T>>>>(data)
        };

        let index = lookup.get(&entity_id).ok_or(ECSError::ComponentRetrievalError)?;

        let Some(read_guard) = data[*index].try_read() else {
            return Err(ECSError::ComponentLocked);
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

    pub fn get_mut<'a, 'b>(&'a self, entity_id: usize) -> ECSResult<ComponentRefMut<'b, T>>
    where
        'a: 'b, // Self outlives 'b therefore should be safe. And valid till 'b is dropped.
    {
        let lookup = self.lookup.read();
        let data: RwLockReadGuard<'b, Vec<RwLock<T>>> = self.data.read();
        let data = unsafe {
            std::mem::transmute::<RwLockReadGuard<'_, Vec<RwLock<T>>>, RwLockReadGuard<'b, Vec<RwLock<T>>>>(data)
        };

        let index = lookup.get(&entity_id).ok_or(ECSError::ComponentRetrievalError)?;

        let Some(write_guard) = data[*index].try_write() else {
            return Err(ECSError::ComponentLocked);
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

    pub fn remove(&self, entity_id: usize) -> ECSResult<()> {
        let mut lookup = self.lookup.write();
        let mut data = self.data.write();

        let index = lookup.remove(&entity_id).ok_or(ECSError::ComponentRemovalError)?;
        data.remove(index);

        Ok(())
    }
    
    pub fn entities(&self) -> Vec<usize> {
        self.lookup.read().keys().copied().collect()
    }
}

pub struct ComponentRef<'a, T> {
    component: *mut T,
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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