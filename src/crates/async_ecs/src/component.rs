use std::any::TypeId;
use std::fmt::Debug;
use std::marker::PhantomData;

use dashmap::DashMap;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::helpers::sparse_set::SparseSet;

/// A trait that represents a dynamic component in the ECS (Entity Component System).
///
/// This trait must be implemented by any type that is intended to be used as a component
/// within the ECS.
pub trait DynamicComponent: 'static + Send + Sync + Debug {}

/// An **immutable** reference to a component in the ECS.
///
/// Equivalent of <p style="color:#FFD700">&Position</p>
///
/// # Examples
/// ```rs
/// // Equivalent of &Position
///let position: ComponentRef<Position> = ...;
///let x = position.x;
///let y = position.y;
/// ```
#[derive(Debug)]
pub struct ComponentRef<'a, T: DynamicComponent + 'a> {
    read_guard: RwLockReadGuard<'a, Box<dyn DynamicComponent>>,
    _phantom: PhantomData<T>,
}

/// A mutable reference to a component in the ECS.
///
/// Equivalent of <p style="color:#FFD700">&mut Position</p>
///
/// # Examples
/// ```rs
/// let mut position: ComponentRefMut<Position> = ...;
/// position.x = 10.0;
/// position.y = 20.0;
/// ```
#[derive(Debug)]
pub struct ComponentRefMut<'a, T: DynamicComponent> {
    write_guard: RwLockWriteGuard<'a, Box<dyn DynamicComponent>>,
    _phantom: PhantomData<T>,
}

// Can get component directly from ComponentRef.
// e.g. let position: ComponentRef<Position> = ...;
// let x = position.x
impl<'a, T: DynamicComponent> std::ops::Deref for ComponentRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&**self.read_guard as *const dyn DynamicComponent as *const T) }
    }
}

// Can get component directly from ComponentRefMut.
// e.g. let mut position: ComponentRefMut<Position> = ...;
// position.x = 10.0;
// position.y = 20.0;
impl<'id, T: DynamicComponent> std::ops::Deref for ComponentRefMut<'id, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&**self.write_guard as *const dyn DynamicComponent as *const T) }
    }
}
impl<'id, T: DynamicComponent> std::ops::DerefMut for ComponentRefMut<'id, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(&mut **self.write_guard as *mut dyn DynamicComponent as *mut T) }
    }
}


/// A storage structure for components in the ECS (Entity Component System).
///
/// This struct uses a `DashMap` to store components, where each component type
/// is associated with a `TypeId` and stored in a `SparseSet` protected by an `RwLock`.
///
/// # Fields
/// - `storages`: A `DashMap` that maps `TypeId` to `SparseSet` of components wrapped in `RwLock`.
#[derive(Debug)]
pub struct ComponentStorage {
    storages: DashMap<TypeId, SparseSet<RwLock<Box<dyn DynamicComponent>>>>,
}


impl ComponentStorage {
    /// Creates a new instance of `ComponentStorage`.
    ///
    /// # Returns
    /// A new `ComponentStorage` with an empty `DashMap` for storing components.
    pub fn new() -> Self {
        Self {
            storages: DashMap::new(),
        }
    }

    /// Inserts a component into the storage for a given entity.
    ///
    /// This function takes an entity ID and a component, and inserts the component
    /// into the storage associated with the component's type.
    ///
    /// # Parameters
    /// - `entity_id`: The ID of the entity to which the component belongs.
    /// - `component`: The component to be inserted.
    ///
    /// # Examples
    /// ```
    /// let storage = ComponentStorage::new();
    /// storage.insert(0usize, Position { x: 0.0, y: 0.0 });
    /// ```
    pub fn insert<T: DynamicComponent>(&self, entity_id: impl Into<usize>, component: T) {
        let type_id = TypeId::of::<T>();

        let mut storage = self.storages.entry(type_id).or_insert_with(|| SparseSet::new());

        storage.insert(entity_id.into(), RwLock::new(Box::new(component)));
    }

    /// Retrieves an immutable reference to a component for a given entity.
    ///
    /// This function takes an entity ID and returns an immutable reference to the component
    /// of type `T` associated with that entity, if it exists.
    ///
    /// # Parameters
    /// - `entity_id`: The ID of the entity to which the component belongs.
    ///
    /// # Returns
    /// An `Option` containing a `ComponentRef` to the component if it exists, or `None` if it does not.
    ///
    /// # Examples
    /// ```
    /// let storage = ComponentStorage::new();
    /// storage.insert(0usize, Position { x: 0.0, y: 0.0 });
    /// let position = storage.get::<Position>(0usize).await.unwrap();
    /// assert_eq!(position.x, 0.0);
    /// assert_eq!(position.y, 0.0);
    /// ```
    pub async fn get<'a, T: DynamicComponent + 'a>(&self, entity_id: impl Into<usize>) -> Option<ComponentRef<'a, T>> {
        let type_id = TypeId::of::<T>();

        let entity_id = entity_id.into();

        let storage = self.storages.get(&type_id)?;
        let component = storage.get(entity_id)?;

        // SAFETY: The RwLock is guaranteed to outlive self, so this read_guard can use the same lifetime
        let read_guard = unsafe {
            std::mem::transmute::<RwLockReadGuard<'_, Box<dyn DynamicComponent>>, RwLockReadGuard<'_, Box<dyn DynamicComponent>>>(component.read().await)
        };

        Some(ComponentRef {
            read_guard,
            _phantom: PhantomData,
        })
    }

    pub async fn get_mut<T: DynamicComponent>(&self, entity_id: impl Into<usize>) -> Option<ComponentRefMut<T>> {
        let type_id = TypeId::of::<T>();

        let entity_id = entity_id.into();

        let storage = self.storages.get(&type_id)?;
        let component = storage.get(entity_id)?;

        // SAFETY: The RwLock is guaranteed to outlive self, so this write_guard can use the same lifetime
        let write_guard = unsafe {
            std::mem::transmute::<RwLockWriteGuard<'_, Box<dyn DynamicComponent>>, RwLockWriteGuard<'_, Box<dyn DynamicComponent>>>(component.write().await)
        };

        Some(ComponentRefMut {
            write_guard,
            _phantom: PhantomData,
        })
    }

    pub fn remove<T: DynamicComponent>(&self, entity_id: impl Into<usize>) {
        let type_id = TypeId::of::<T>();

        let entity_id = entity_id.into();

        if let Some(mut storage) = self.storages.get_mut(&type_id) {
            storage.remove(entity_id);
        }
    }
}


#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl DynamicComponent for Position {}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
impl DynamicComponent for Velocity {}

#[tokio::test]
async fn test_basic_usage() {
    let component_storage = ComponentStorage::new();

    let entity = 0usize;
    let position = Position { x: 0.0, y: 0.0 };
    component_storage.insert(entity, position);

    let position = component_storage.get::<Position>(entity).await.unwrap();
    assert_eq!(position.x, 0.0);
    assert_eq!(position.y, 0.0);
}

#[tokio::test]
async fn test_insert_and_get() {
    let storage = ComponentStorage::new();
    storage.insert(0usize, Position { x: 0.0, y: 0.0 });
    let component = storage.get::<Position>(0usize).await;
    assert!(component.is_some());
    assert_eq!(component.unwrap().x, 0.0);
}

#[tokio::test]
async fn test_insert_and_get_mut() {
    let storage = ComponentStorage::new();
    storage.insert(0usize, Position { x: 0.0, y: 0.0 });
    let component = storage.get_mut::<Position>(0usize).await;
    assert!(component.is_some());
    assert_eq!(component.unwrap().x, 0.0);
}