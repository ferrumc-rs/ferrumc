use std::marker::PhantomData;

pub mod container;
pub mod db;
pub mod errors;

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
