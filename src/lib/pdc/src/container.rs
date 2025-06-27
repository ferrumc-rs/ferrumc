use std::collections::HashMap;

use bevy_ecs::component::Component;
use bitcode::{Decode, Encode};

use crate::{PersistentKey, errors::PersistentDataError};

#[derive(Component)]
pub struct PersistentDataContainer {
    pub(crate) data: HashMap<String, Vec<u8>>,
}

impl std::fmt::Debug for PersistentDataContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PersistentDataContainer")
            .field("data", &self.data)
            .finish()
    }
}

impl Default for PersistentDataContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl PersistentDataContainer {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set<T>(&mut self, key: &PersistentKey<T>, value: T) -> Result<(), PersistentDataError>
    where
        T: Encode + Sync + Send + 'static,
    {
        let encoded = bitcode::encode(&value);
        self.data.insert(key.identifier.clone(), encoded);

        Ok(())
    }

    pub fn get<T>(&self, key: &PersistentKey<T>) -> Option<T>
    where
        T: Clone + for<'de> Decode<'de> + 'static,
    {
        let id = &key.identifier;
        if let Some(raw_bytes) = self.data.get(id) {
            if let Ok(decoded) = bitcode::decode::<T>(raw_bytes) {
                return Some(decoded);
            }
        }

        None
    }

    pub fn get_unchecked<T>(&self, key: &PersistentKey<T>) -> T
    where
        T: Clone + 'static + for<'de> bitcode::Decode<'de>,
    {
        self.get::<T>(key).expect(&format!(
            "PersistentDataContainer::get_unchecked failed for key: {}",
            key.identifier
        ))
    }

    pub fn get_or<T>(&self, key: &PersistentKey<T>, fallback: T) -> T
    where
        T: Clone + 'static + for<'de> bitcode::Decode<'de>,
    {
        self.get(key).unwrap_or(fallback)
    }

    pub fn get_or_default<T>(&self, key: &PersistentKey<T>) -> T
    where
        T: Clone + Default + 'static + for<'de> bitcode::Decode<'de>,
    {
        self.get(key).unwrap_or_default()
    }

    pub fn remove<T>(&mut self, key: &PersistentKey<T>) {
        self.data.remove(&key.identifier);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn has<T>(&self, key: &PersistentKey<T>) -> bool {
        self.data.contains_key(&key.identifier)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn merge(&mut self, container: &PersistentDataContainer) {
        self.data
            .extend(container.data.iter().map(|(k, v)| (k.clone(), v.clone())));
    }
}
