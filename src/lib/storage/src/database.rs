use crate::errors::StorageError;

pub trait Database {
    type Key;
    type Value;

    fn create_table(&self, table: &str) -> Result<(), StorageError>;
    fn insert(&self, table: &str, key: Self::Key, value: Self::Value) -> Result<(), StorageError>;
    fn get(&self, table: &str, key: Self::Key) -> Result<Option<Self::Value>, StorageError>;
    fn delete(&self, table: &str, key: Self::Key) -> Result<(), StorageError>;
    fn update(&self, table: &str, key: Self::Key, value: Self::Value) -> Result<(), StorageError>;
    fn upsert(
        &self,
        table: &str,
        key: Self::Key,
        value: &Self::Value,
    ) -> Result<bool, StorageError>;
    fn batch_insert(
        &self,
        table: &str,
        data: Vec<(Self::Key, Self::Value)>,
    ) -> Result<(), StorageError>;
    fn batch_get(
        &self,
        table: &str,
        keys: Vec<Self::Key>,
    ) -> Result<Vec<Option<Self::Value>>, StorageError>;
    fn batch_upsert(
        &self,
        table: &str,
        data: Vec<(Self::Key, Self::Value)>,
    ) -> Result<(), StorageError>;
}
