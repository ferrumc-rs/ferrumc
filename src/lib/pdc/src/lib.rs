use std::marker::PhantomData;

use serde::{Serialize, de::DeserializeOwned};

pub mod container;
pub mod errors;
pub mod load;
pub mod save;

#[cfg(test)]
mod tests;

pub trait PersistentContainer: Serialize + DeserializeOwned {}
impl<T> PersistentContainer for T where T: Serialize + DeserializeOwned {}

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
