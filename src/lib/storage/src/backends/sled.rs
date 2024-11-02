use crate::errors::StorageError;
use crate::DatabaseBackend;
use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub struct SledBackend {
    db: Arc<sled::Db>,
}

#[async_trait]
impl DatabaseBackend for SledBackend {
    async fn initialize(store_path: Option<PathBuf>) -> Result<Self, StorageError>
    where
        Self: Sized,
    {
        if let Some(path) = store_path {
            let db =
                sled::open(path).map_err(|e| StorageError::DatabaseInitError(e.to_string()))?;
            Ok(Self { db: Arc::new(db) })
        } else {
            Err(StorageError::InvalidPath)
        }
    }

    async fn insert(
        &self,
        table: String,
        key: u64,
        value: Vec<u8>,
    ) -> Result<(), StorageError> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let table = db
                .open_tree(table)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            table
                .insert(key.to_be_bytes(), value)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            Ok::<(), StorageError>(())
        })
        .await
        .expect("Failed to insert data into database")?;
        Ok(())
    }

    async fn get(&self, table: String, key: u64) -> Result<Option<Vec<u8>>, StorageError> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let table = db
                .open_tree(table)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            let value = table
                .get(key.to_be_bytes())
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            if let Some(value) = value {
                Ok(Some(value.to_vec()))
            } else {
                Ok(None)
            }
        })
        .await
        .expect("Failed to get data from database")?;
        Ok(result)
    }

    async fn delete(&self, table: String, key: u64) -> Result<(), StorageError> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let table = db
                .open_tree(table)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            table
                .remove(key.to_be_bytes())
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            Ok::<(), StorageError>(())
        })
        .await
        .expect("Failed to delete data from database")?;
        Ok(())
    }

    async fn update(
        &self,
        table: String,
        key: u64,
        value: Vec<u8>,
    ) -> Result<(), StorageError> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let table = db
                .open_tree(table)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            table
                .insert(key.to_be_bytes(), value)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            Ok::<(), StorageError>(())
        })
        .await
        .expect("Failed to update data in database")?;
        Ok(())
    }

    async fn upsert(
        &self,
        table: String,
        key: u64,
        value: Vec<u8>,
    ) -> Result<bool, StorageError> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let table = db
                .open_tree(table)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            let value = table
                .insert(key.to_be_bytes(), value)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            Ok(value.is_none())
        })
        .await
        .expect("Failed to upsert data in database")?;
        Ok(result)
    }

    async fn exists(&self, table: String, key: u64) -> Result<bool, StorageError> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let table = db
                .open_tree(table)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            let value = table
                .get(key.to_be_bytes())
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            Ok(value.is_some())
        })
        .await
        .expect("Failed to check if key exists in database")?;
        Ok(result)
    }

    async fn details(&self) -> String {
        "Sled 0.34.7".to_string()
    }

    async fn batch_insert(
        &self,
        table: String,
        data: Vec<(u64, Vec<u8>)>,
    ) -> Result<(), StorageError> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let table = db
                .open_tree(table)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            for (key, value) in data {
                table
                    .insert(key.to_be_bytes(), value)
                    .map_err(|e| StorageError::WriteError(e.to_string()))?;
            }
            Ok::<(), StorageError>(())
        })
        .await
        .expect("Failed to insert data into database")?;
        Ok(())
    }

    async fn batch_get(
        &self,
        table: String,
        keys: Vec<u64>,
    ) -> Result<Vec<Option<Vec<u8>>>, StorageError> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let table = db
                .open_tree(table)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            let mut values = Vec::new();
            for key in keys {
                let value = table
                    .get(key.to_be_bytes())
                    .map_err(|e| StorageError::ReadError(e.to_string()))?;
                if let Some(value) = value {
                    values.push(Some(value.to_vec()));
                } else {
                    values.push(None);
                }
            }
            Ok(values)
        })
        .await
        .expect("Failed to get data from database")?;
        Ok(result)
    }

    async fn flush(&self) -> Result<(), StorageError> {
        self.db
            .flush_async()
            .await
            .map_err(|e| StorageError::WriteError(e.to_string()))?;
        Ok(())
    }

    async fn create_table(&self, table: String) -> Result<(), StorageError> {
        self.db
            .open_tree(table)
            .map_err(|e| StorageError::WriteError(e.to_string()))?;
        Ok(())
    }

    async fn close(&self) -> Result<(), StorageError> {
        self.db
            .flush_async()
            .await
            .map_err(|e| StorageError::WriteError(e.to_string()))?;
        Ok(())
    }
}
