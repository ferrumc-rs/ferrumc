use std::marker::PhantomData;

use serde::{Serialize, de::DeserializeOwned};

use crate::{container::PersistentDataContainer, errors::PersistentDataError};

pub mod container;
pub mod errors;

#[cfg(test)]
mod tests;

pub trait PersistentContainer: Serialize + DeserializeOwned {}
impl<T> PersistentContainer for T where T: Serialize + DeserializeOwned {}

pub trait PersistentDataHolder {
    fn get_persistent_data(&self) -> &PersistentDataContainer;

    fn edit_persistent_data<
        F: FnOnce(&mut PersistentDataContainer) -> Result<(), PersistentDataError>,
    >(
        &mut self,
        func: F,
    );
}

pub struct PersistentKey<T> {
    identifier: String,
    _marker: PhantomData<T>,
}

impl<T> PersistentKey<T> {
    pub fn new(key: &str) -> Self {
        Self {
            identifier: key.to_string(),
            _marker: PhantomData,
        }
    }
}
