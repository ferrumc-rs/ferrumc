#![feature(async_closure)]
pub mod backends;
pub mod compressors;
pub mod errors;

use crate::errors::StorageError;
use async_trait::async_trait;
use std::path::PathBuf;

/// A trait for database backends. This is used to abstract away the underlying database implementation.
/// This allows for easy swapping of databases without changing the rest of the code. These functions are
/// purely for storage and retrieval of data. Any other functionality such as serialization or caching
/// should be implemented in a separate layer.
#[async_trait]
pub trait DatabaseBackend {
    /// Initializes the database
    ///
    /// # Arguments
    ///
    /// * `store_path` - An optional path to the database file
    ///
    /// # Returns
    ///
    /// A Result containing the initialized database or a StorageError
    async fn initialize(store_path: Option<PathBuf>) -> Result<Self, StorageError>
    where
        Self: Sized;

    /// Inserts a key-value pair into the database
    ///
    /// # Arguments
    ///
    /// * `table` - The table to insert the key-value pair into
    /// * `key` - The key to insert
    /// * `value` - The value to insert
    ///
    /// # Returns
    ///
    /// A Result containing the inserted key-value pair or a StorageError
    async fn insert(&self, table: String, key: u64, value: Vec<u8>)
        -> Result<(), StorageError>;

    /// Retrieves a value from the database
    ///
    /// # Arguments
    ///
    /// * `table` - The table to retrieve the value from
    /// * `key` - The key to retrieve
    ///
    /// # Returns
    ///
    /// A Result containing a possible value or a StorageError
    async fn get(&self, table: String, key: u64) -> Result<Option<Vec<u8>>, StorageError>;

    /// Deletes a key-value pair from the database
    ///
    /// # Arguments
    ///
    /// * `table` - The table to delete the key-value pair from
    /// * `key` - The key to delete
    ///
    /// # Returns
    ///
    /// A Result containing nothing or a StorageError
    async fn delete(&self, table: String, key: u64) -> Result<(), StorageError>;

    /// Updates a key-value pair in the database
    ///
    /// # Arguments
    ///
    /// * `table` - The table to update the key-value pair in
    /// * `key` - The key to update
    /// * `value` - The new value
    ///
    /// # Returns
    ///
    /// A Result containing nothing or a StorageError
    async fn update(&self, table: String, key: u64, value: Vec<u8>)
        -> Result<(), StorageError>;

    /// Upserts a key-value pair in the database
    ///
    /// # Arguments
    ///
    /// * `table` - The table to upsert the key-value pair in
    /// * `key` - The key to upsert
    /// * `value` - The value to upsert
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating if the key was inserted or updated or a StorageError
    async fn upsert(
        &self,
        table: String,
        key: u64,
        value: Vec<u8>,
    ) -> Result<bool, StorageError>;

    /// Checks if a key exists in the database
    ///
    /// # Arguments
    ///
    /// * `table` - The table to check
    /// * `key` - The key to check
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating if the key exists or a StorageError
    async fn exists(&self, table: String, key: u64) -> Result<bool, StorageError>;

    /// Returns some details about the underlying database
    ///
    /// This is just a string but can be useful for determining the underlying database
    ///
    /// # Returns
    ///
    /// A string containing details about the database
    async fn details(&self) -> String;

    /// Inserts multiple key-value pairs into the database
    ///
    /// # Arguments
    ///
    /// * `table` - The table to insert the key-value pairs into
    /// * `data` - A vector of key-value pairs to insert
    ///
    /// # Returns
    ///
    /// A Result containing nothing or a StorageError
    async fn batch_insert(
        &self,
        table: String,
        data: Vec<(u64, Vec<u8>)>,
    ) -> Result<(), StorageError>;

    /// Retrieves multiple values from the database
    ///
    /// # Arguments
    ///
    /// * `table` - The table to retrieve the values from
    /// * `keys` - A vector of keys to retrieve
    ///
    /// # Returns
    ///
    /// A Result containing the retrieved values or a StorageError
    async fn batch_get(
        &self,
        table: String,
        keys: Vec<u64>,
    ) -> Result<Vec<Option<Vec<u8>>>, StorageError>;

    /// Flushes the database
    ///
    /// Writes all pending changes to disk. This can be used to force all updates to be written to disk
    /// and therefore prevent data loss in the event of a crash. Not all backends will need this
    /// function as they may write changes to disk immediately. In that case, this function can be a no-op.
    async fn flush(&self) -> Result<(), StorageError>;

    /// Creates a new table in the database
    ///
    /// # Arguments
    ///
    /// * `table` - The name of the table to create
    ///
    /// # Returns
    ///
    /// A Result containing nothing or a StorageError
    async fn create_table(&self, table: String) -> Result<(), StorageError>;

    /// Closes the database
    ///
    /// This is used to close the database and free up any resources. This should be called when the database
    /// is no longer needed. This can be a no-op for some backends.
    ///
    /// # Returns
    ///
    /// A Result containing nothing or a StorageError
    async fn close(&self) -> Result<(), StorageError>;
}
