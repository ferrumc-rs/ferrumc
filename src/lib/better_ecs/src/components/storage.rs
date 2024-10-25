use super::{ECSError, Result};
use crate::entity::EntityId;
use std::ops::{Deref, DerefMut};
use dashmap::DashMap;
use dashmap::mapref::one::{Ref, RefMut};
use dashmap::try_result::TryResult;

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
    pub fn with(entity_id: EntityId, component: C) -> Result<Self> {
        let new_instance = Self::new();

        new_instance.insert(entity_id, component)?;


        Ok(new_instance)
    }
    pub fn insert(&self, entity_id: usize, component: C) -> Result<()> {
        self.components.insert(entity_id, component);

        Ok(())
    }



    pub fn get(&self, entity_id: usize) -> Result<ComponentRef<C>> {
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
                Err(ECSError::ComponentIsLocked)
            }
        }
    }

    pub fn get_mut(&self, entity_id: usize) -> Result<ComponentRefMut<C>> {
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
                Err(ECSError::ComponentIsLocked)
            }
        }
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