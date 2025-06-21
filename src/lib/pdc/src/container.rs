use std::{
    any::{TypeId, type_name},
    collections::HashMap,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{PersistentContainer, PersistentKey, errors::PersistentDataError};

#[derive(Serialize, Deserialize)]
pub struct PersistentDataContainer {
    #[serde(skip)]
    type_map: HashMap<String, TypeId>, // Tracks runtime types

    data: HashMap<String, Value>,
}

impl Default for PersistentDataContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl PersistentDataContainer {
    pub fn new() -> Self {
        Self {
            type_map: HashMap::new(),
            data: HashMap::new(),
        }
    }

    pub fn set<T: PersistentContainer + 'static>(&mut self, key: &PersistentKey<T>, value: T) {
        if let Ok(json_value) = serde_json::to_value(value) {
            self.type_map
                .insert(key.identifier.clone(), TypeId::of::<T>());
            self.data.insert(key.identifier.clone(), json_value);
        }
    }

    pub fn get<T: PersistentContainer + 'static>(
        &self,
        key: &PersistentKey<T>,
    ) -> Result<T, PersistentDataError> {
        match self.type_map.get(&key.identifier) {
            Some(stored_type) if *stored_type == TypeId::of::<T>() => self
                .data
                .get(&key.identifier)
                .ok_or(PersistentDataError::KeyNotFound)
                .and_then(|value| {
                    serde_json::from_value(value.clone())
                        .map_err(|_| PersistentDataError::DeserializationError)
                }),
            Some(_) => Err(PersistentDataError::TypeMismatch {
                expected: type_name::<T>(),
            }),
            None => Err(PersistentDataError::KeyNotFound),
        }
    }

    pub fn get_unchecked<T: PersistentContainer + 'static>(&self, key: &PersistentKey<T>) -> T {
        self.get(key)
            .expect("PersistentDataContainer::get_unchecked failed")
    }

    pub fn get_or_default<T: PersistentContainer + Default + 'static>(
        &self,
        key: &PersistentKey<T>,
    ) -> T {
        self.get(key).unwrap_or_default()
    }

    pub fn remove<T>(&mut self, key: &PersistentKey<T>) {
        self.data.remove(&key.identifier);
        self.type_map.remove(&key.identifier);
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.type_map.clear();
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

    pub fn merge(&self, container: PersistentDataContainer) -> PersistentDataContainer {
        let mut new_data = self.data.clone();
        let mut new_type_map = self.type_map.clone();

        for (key, value) in container.data.into_iter() {
            new_data.insert(key, value);
        }

        for (key, type_id) in container.type_map.into_iter() {
            new_type_map.insert(key, type_id);
        }

        PersistentDataContainer {
            data: new_data,
            type_map: new_type_map,
        }
    }

    pub fn merge_in_place(&mut self, container: &PersistentDataContainer) {
        for (key, value) in &container.data {
            self.data.insert(key.clone(), value.clone());
        }

        for (key, type_id) in &container.type_map {
            self.type_map.insert(key.clone(), *type_id);
        }
    }
}
