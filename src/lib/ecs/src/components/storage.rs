use crate::errors::ECSError;
use crate::ECSResult;
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use parking_lot::RwLock;
use whirlwind::mapref::{MapRef, MapRefMut};
use whirlwind::ShardMap;

pub trait Component: 'static {}

impl<T: 'static> Component for T {}

pub struct ComponentSparseSet<C: Component> {
    components: ShardMap<usize, C>,
    entities: RwLock<Vec<usize>>,
}

impl<C: Component> Default for ComponentSparseSet<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Component> ComponentSparseSet<C> {
    pub fn new() -> Self {
        Self {
            components: ShardMap::new(),
            entities: RwLock::new(Vec::new()),
        }
    }
    pub async fn with(entity_id: usize, component: C) -> ECSResult<Self> {
        let new_instance = Self::new();

        new_instance.insert(entity_id, component).await?;

        Ok(new_instance)
    }
    pub async fn insert(&self, entity_id: usize, component: C) -> ECSResult<()> {
        self.components.insert(entity_id, component).await;
        self.entities.write().push(entity_id);
        Ok(())
    }

    pub async fn get<'a>(&'a self, entity_id: &'a usize) -> ECSResult<ComponentRef<'a, C>> {
        self.components
            .get(entity_id)
            .await
            .map(|entry| ComponentRef { guard: entry })
            .ok_or(ECSError::ComponentRetrievalError)
    }

    pub async fn get_mut<'a>(&'a self, entity_id: &'a usize) -> ECSResult<ComponentRefMut<'a, C>> {
        self.components
            .get_mut(&entity_id)
            .await
            .map(|entry| ComponentRefMut { guard: entry })
            .ok_or(ECSError::ComponentRetrievalError)
    }

    pub async fn remove(&self, entity_id: usize) -> ECSResult<()> {
        //! It will deadlock in the situation of a deadlock.
        self.components.remove(&entity_id).await;
        self.entities.write().retain(|&id| id != entity_id);

        Ok(())
    }
    pub fn entities(&self) -> Vec<usize> {
        // self.components.iter().map(|entry| *entry.key()).collect()
        self.entities.read().clone()
    }
}

pub struct ComponentRef<'a, T> {
    // guard: Ref<'a, usize, T>,
    guard: MapRef<'a, usize, T>
}

impl<T: Display> Display for ComponentRef<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.guard.fmt(f)
    }
}

impl<T> Deref for ComponentRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[allow(clippy::explicit_auto_deref)]
        &*self.guard
    }
}

pub struct ComponentRefMut<'a, T> {
    // guard: RefMut<'a, usize, T>,
    guard: MapRefMut<'a, usize, T>
}

impl<T> Deref for ComponentRefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[allow(clippy::explicit_auto_deref)]
        &*self.guard
    }
}

impl<T> DerefMut for ComponentRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[allow(clippy::explicit_auto_deref)]
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
