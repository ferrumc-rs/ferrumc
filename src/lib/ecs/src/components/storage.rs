use crate::errors::ECSError;
use crate::ECSResult;
use dashmap::mapref::one::{Ref, RefMut};
use dashmap::DashMap;
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use typename::TypeName;

pub trait Component: 'static + TypeName {}

impl<T: 'static + TypeName> Component for T {}

pub struct ComponentSparseSet<C: Component> {
    components: DashMap<usize, C>,
}

impl<C: Component> Default for ComponentSparseSet<C> {
    fn default() -> Self {
        Self::new()
    }
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
        self.components
            .get(&entity_id)
            .map(|entry| ComponentRef { guard: entry })
            .ok_or(ECSError::ComponentRetrievalError)
    }

    pub fn get_mut(&self, entity_id: usize) -> ECSResult<ComponentRefMut<C>> {
        self.components
            .get_mut(&entity_id)
            .map(|entry| ComponentRefMut { guard: entry })
            .ok_or(ECSError::ComponentRetrievalError)
    }

    pub fn remove(&self, entity_id: usize) -> ECSResult<()> {
        //! It will deadlock in the situation of a deadlock.
        self.components.remove(&entity_id);

        Ok(())
    }
    pub fn entities(&self) -> Vec<usize> {
        self.components.iter().map(|entry| *entry.key()).collect()
    }
}

pub struct ComponentRef<'a, T> {
    guard: Ref<'a, usize, T>,
}

impl<T: Display> Display for ComponentRef<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.guard.fmt(f)
    }
}

impl<T> Deref for ComponentRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[expect(clippy::explicit_auto_deref)]
        &*self.guard
    }
}

pub struct ComponentRefMut<'a, T> {
    guard: RefMut<'a, usize, T>,
}

impl<T> Deref for ComponentRefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[expect(clippy::explicit_auto_deref)]
        &*self.guard
    }
}

impl<T> DerefMut for ComponentRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[expect(clippy::explicit_auto_deref)]
        &mut *self.guard
    }
}

impl<T: Component + Debug> Debug for ComponentRef<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.guard.fmt(f)
    }
}

impl<T: Component + Debug> Debug for ComponentRefMut<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.guard.fmt(f)
    }
}
