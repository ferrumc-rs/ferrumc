use std::marker::PhantomData;

use serde::{Serialize, de::DeserializeOwned};

use crate::container::PersistentDataContainer;

pub mod container;
pub mod errors;

#[cfg(test)]
mod tests;

pub trait PersistentContainer: Serialize + DeserializeOwned {}
impl<T> PersistentContainer for T where T: Serialize + DeserializeOwned {}

pub trait PersistentDataHolder {
    fn get_persistent_data(&self) -> &PersistentDataContainer;

    fn edit_persistent_data<F: FnOnce(&mut PersistentDataContainer)>(&mut self, func: F);
}

pub struct PersistentKey<T> {
    pub identifier: String,
    _marker: PhantomData<T>,
}

impl<T> PersistentKey<T> {
    pub fn new(namespace: &str, key: &str) -> Self {
        Self {
            identifier: format!("{}:{}", namespace, key),
            _marker: PhantomData,
        }
    }
}
