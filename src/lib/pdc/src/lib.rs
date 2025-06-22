use std::marker::PhantomData;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

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

pub trait PersistentKey {
    type Value: Serialize + for<'de> Deserialize<'de>;

    fn key() -> &'static str;
}
