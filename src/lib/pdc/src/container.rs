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
    type_map: HashMap<String, TypeId>, // Runtime type tracking

    data: HashMap<String, Value>,
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
            type_map: HashMap::new(),
            data: HashMap::new(),
        }
    }

    pub fn set<K: PersistentKey + 'static>(
        &mut self,
        value: K::Value,
    ) -> Result<(), PersistentDataError> {
        let json_value =
            serde_json::to_value(value).map_err(|_| PersistentDataError::DeserializationError)?;

        self.type_map
            .insert(K::key().to_string(), TypeId::of::<K::Value>());
        self.data.insert(K::key().to_string(), json_value);

        Ok(())
    }

    pub fn get<K: PersistentKey + 'static>(&self) -> Result<K::Value, PersistentDataError> {
        match self.type_map.get(K::key()) {
            Some(stored_type) if *stored_type == TypeId::of::<K::Value>() => {
                let json_value = self
                    .data
                    .get(K::key())
                    .ok_or(PersistentDataError::KeyNotFound)?;

                serde_json::from_value(json_value.clone())
                    .map_err(|_| PersistentDataError::DeserializationError)
            }
            Some(_) => Err(PersistentDataError::TypeMismatch {
                expected: type_name::<K::Value>(),
            }),
            None => Err(PersistentDataError::KeyNotFound),
        }
    }

    pub fn get_unchecked<K: PersistentKey + 'static>(&self) -> K::Value {
        self.get::<K>().unwrap_or_else(|_| {
            panic!(
                "PersistentDataContainer::get_unchecked failed for key: {}",
                K::key()
            )
        })
    }

    pub fn get_or<K: PersistentKey + 'static>(&self, fallback: K::Value) -> K::Value
    where
        K::Value: Clone,
    {
        self.get::<K>().unwrap_or(fallback)
    }

    pub fn get_or_default<K: PersistentKey + 'static>(&self) -> K::Value
    where
        K::Value: Default,
    {
        self.get::<K>().unwrap_or_default()
    }

    pub fn remove<K: PersistentKey>(&mut self) {
        let key = K::key();

        self.data.remove(key);
        self.type_map.remove(key);
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.type_map.clear();
    }

    pub fn has<K: PersistentKey>(&self) -> bool {
        self.data.contains_key(K::key())
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn merge(&self, container: &PersistentDataContainer) -> PersistentDataContainer {
        let mut new_data = self.data.clone();
        let mut new_type_map = self.type_map.clone();

        for (key, value) in &container.data {
            new_data.insert(key.clone(), value.clone());
        }

        for (key, type_id) in &container.type_map {
            new_type_map.insert(key.clone(), *type_id);
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
