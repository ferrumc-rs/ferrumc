use std::ops::{Deref, DerefMut};
use dashmap::DashMap;
use dashmap::mapref::one::{Ref, RefMut};
use dashmap::try_result::{TryResult};
use crate::ECSResult;
use crate::errors::ECSError;

pub trait Component: 'static {}

impl<T: 'static> Component for T {}

pub struct ComponentSparseSet<C: Component> {
    components: DashMap<usize, C>,
}


impl<C: Component> ComponentSparseSet<C> {
    pub fn new() -> Self {
        Self {
            components: DashMap::new(),
        }
    }
    pub fn with(entity_id: usize, component: C) -> ECSResult<Self> {
        let new_instance = Self::new();

        new_instance.insert(entity_id, component)?;


        Ok(new_instance)
    }
    pub fn insert(&self, entity_id: usize, component: C) -> ECSResult<()> {
        self.components.insert(entity_id, component);

        Ok(())
    }



    pub fn get(&self, entity_id: usize) -> ECSResult<ComponentRef<C>> {
        let components = self.components
            .try_get(&entity_id);

        match components {
            TryResult::Present(value) => {
                Ok(ComponentRef { guard: value })
            }
            TryResult::Absent => {
                Err(ECSError::ComponentRetrievalError)
            }
            TryResult::Locked => {
                Err(ECSError::ComponentLocked)
            }
        }
    }

    pub fn get_mut(&self, entity_id: usize) -> ECSResult<ComponentRefMut<C>> {
        let components = self.components
            .try_get_mut(&entity_id);

        match components {
            TryResult::Present(value) => {
                Ok(ComponentRefMut { guard: value })
            }
            TryResult::Absent => {
                Err(ECSError::ComponentRetrievalError)
            }
            TryResult::Locked => {
                Err(ECSError::ComponentLocked)
            }
        }
    }
    
    pub fn remove(&self, entity_id: usize) -> ECSResult<()>{
        if let TryResult::Locked = self.components.try_get_mut(&entity_id) {
            return Err(ECSError::ComponentLocked);
        }
        self.components.remove(&entity_id);
        
        Ok(())
    }
    pub fn entities(&self) -> Vec<usize> {
        self.components.iter().map(|entry| *entry.key()).collect()
    }
}

pub struct ComponentRef<'a, T> {
    guard: Ref<'a, usize, T>
}

impl<'a, T> Deref for ComponentRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[allow(clippy::explicit_auto_deref)]
        &*self.guard
    }
}

pub struct ComponentRefMut<'a, T> {
    guard: RefMut<'a, usize, T>
}

impl<'a, T> Deref for ComponentRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[allow(clippy::explicit_auto_deref)]
        &*self.guard
    }
}

impl<'a, T> DerefMut for ComponentRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[allow(clippy::explicit_auto_deref)]
        &mut *self.guard
    }
}