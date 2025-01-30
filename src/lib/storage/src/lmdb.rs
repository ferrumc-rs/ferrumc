use crate::errors::StorageError;
use heed;
use heed::byteorder::BigEndian;
use heed::types::{Bytes, U128};
use heed::{Database, Env, EnvOpenOptions, Error};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LmdbBackend {
    env: Arc<Env>,
}

impl From<Error> for StorageError {
    fn from(err: heed::Error) -> Self {
        match err {
            Error::Io(e) => StorageError::GenericIoError(e),
            Error::Encoding(e) => StorageError::WriteError(e.to_string()),
            Error::Decoding(e) => StorageError::ReadError(e.to_string()),
            _ => StorageError::DatabaseError(err.to_string()),
        }
    }
}

impl LmdbBackend {
    pub async fn initialize(store_path: Option<PathBuf>) -> Result<Self, StorageError>
    where
        Self: Sized,
    {
        let Some(checked_path) = store_path else {
            return Err(StorageError::InvalidPath);
        };
        if !checked_path.exists() {
            std::fs::create_dir_all(&checked_path)?;
        }
        // Convert the map size from GB to bytes and round it to the nearest page size.
        let map_size = ferrumc_config::statics::get_global_config()
            .database
            .map_size as usize
            * 1024
            * 1024
            * 1024;
        let rounded_map_size = ((map_size as f64 / page_size::get() as f64).round()
            * page_size::get() as f64) as usize;
        unsafe {
            let backend = LmdbBackend {
                env: Arc::new(
                    EnvOpenOptions::new()
                        // Change this as more tables are needed.
                        .max_dbs(1)
                        .map_size(rounded_map_size)
                        .open(checked_path)
                        .map_err(|e| StorageError::DatabaseInitError(e.to_string()))?,
                ),
            };
            Ok(backend)
        }
    }

    pub async fn insert(
        &self,
        table: String,
        key: u128,
        value: Vec<u8>,
    ) -> Result<(), StorageError> {
        let env = self.env.clone();
        tokio::task::spawn_blocking(move || {
            let mut rw_txn = env.write_txn()?;
            let db: Database<U128<BigEndian>, Bytes> =
                env.create_database(&mut rw_txn, Some(&table))?;
            if db.get(&rw_txn, &key)?.is_some() {
                return Err(StorageError::KeyExists(key as u64));
            }
            db.put(&mut rw_txn, &key, &value)?;
            rw_txn.commit()?;
            Ok(())
        })
        .await
        .expect("Failed to run tokio task")
    }

    pub async fn get(&self, table: String, key: u128) -> Result<Option<Vec<u8>>, StorageError> {
        let env = self.env.clone();
        tokio::task::spawn_blocking(move || {
            let ro_txn = env.read_txn()?;
            let db: Database<U128<BigEndian>, Bytes> = env
                .open_database(&ro_txn, Some(&table))?
                .ok_or(StorageError::TableError("Table not found".to_string()))?;
            let value = db.get(&ro_txn, &key)?;
            if let Some(v) = value {
                Ok(Some(v.to_vec()))
            } else {
                Ok(None)
            }
        })
        .await
        .expect("Failed to run tokio task")
    }

    pub async fn delete(&self, table: String, key: u128) -> Result<(), StorageError> {
        let env = self.env.clone();
        tokio::task::spawn_blocking(move || {
            let mut rw_txn = env.write_txn()?;
            let db: Database<U128<BigEndian>, Bytes> = env
                .open_database(&rw_txn, Some(&table))?
                .ok_or(StorageError::TableError("Table not found".to_string()))?;
            if db.get(&rw_txn, &key)?.is_none() {
                return Err(StorageError::KeyNotFound(key as u64));
            }
            db.delete(&mut rw_txn, &key)?;
            rw_txn.commit()?;
            Ok(())
        })
        .await
        .expect("Failed to run tokio task")
    }

    pub async fn update(
        &self,
        table: String,
        key: u128,
        value: Vec<u8>,
    ) -> Result<(), StorageError> {
        let env = self.env.clone();
        tokio::task::spawn_blocking(move || {
            let mut rw_txn = env.write_txn()?;
            let db: Database<U128<BigEndian>, Bytes> = env
                .open_database(&rw_txn, Some(&table))?
                .ok_or(StorageError::TableError("Table not found".to_string()))?;
            if db.get(&rw_txn, &key)?.is_none() {
                return Err(StorageError::KeyNotFound(key as u64));
            }
            db.put(&mut rw_txn, &key, &value)?;
            rw_txn.commit()?;
            Ok(())
        })
        .await
        .expect("Failed to run tokio task")
    }

    pub async fn upsert(
        &self,
        table: String,
        key: u128,
        value: Vec<u8>,
    ) -> Result<bool, StorageError> {
        let env = self.env.clone();
        tokio::task::spawn_blocking(move || {
            let mut rw_txn = env.write_txn()?;
            let db: Database<U128<BigEndian>, Bytes> = env
                .open_database(&rw_txn, Some(&table))?
                .ok_or(StorageError::TableError("Table not found".to_string()))?;
            db.put(&mut rw_txn, &key, &value)?;
            rw_txn.commit()?;
            Ok(true)
        })
        .await
        .expect("Failed to run tokio task")
    }

    pub async fn batch_upsert(
        &self,
        table: String,
        data: Vec<(u128, Vec<u8>)>,
    ) -> Result<(), StorageError> {
        let env = self.env.clone();
        tokio::task::spawn_blocking(move || {
            let mut rw_txn = env.write_txn()?;

            // Open or create the database for the given table
            let db = env.create_database::<U128<BigEndian>, Bytes>(&mut rw_txn, Some(&table))?;

            // Create a map of keys and their associated values
            let keymap: HashMap<u128, &Vec<u8>> = data.iter().map(|(k, v)| (*k, v)).collect();

            // Iterate through the keys in sorted order
            let mut sorted_keys: Vec<u128> = keymap.keys().cloned().collect();
            sorted_keys.sort();

            // Iterate through the sorted keys to perform upserts
            for key in sorted_keys {
                // Check if the key already exists
                if db.get(&rw_txn, &key)?.is_some() {
                    // Update the value if it exists (you can modify this logic as needed)
                    db.put(&mut rw_txn, &key, keymap[&key])?;
                } else {
                    // Insert the new key-value pair if the key doesn't exist
                    db.put(&mut rw_txn, &key, keymap[&key])?;
                }
            }

            // Commit the transaction after all upserts are performed
            rw_txn.commit()?;
            Ok(())
        })
        .await
        .expect("Failed to run tokio task")
    }

    pub async fn exists(&self, table: String, key: u128) -> Result<bool, StorageError> {
        let env = self.env.clone();
        tokio::task::spawn_blocking(move || {
            let ro_txn = env.read_txn()?;
            let db: Database<U128<BigEndian>, Bytes> = env
                .open_database(&ro_txn, Some(&table))?
                .ok_or(StorageError::TableError("Table not found".to_string()))?;
            Ok(db.get(&ro_txn, &key)?.is_some())
        })
        .await
        .expect("Failed to run tokio task")
    }
    
    pub async fn table_exists(&self, table: String) -> Result<bool, StorageError> {
        let env = self.env.clone();
        tokio::task::spawn_blocking(move || {
            let ro_txn = env.read_txn()?;
            let db = env.open_database::<U128<BigEndian>, Bytes>(&ro_txn, Some(&table))?;
            Ok(db.is_some())
        })
        .await
        .expect("Failed to run tokio task")
    }

    pub async fn details(&self) -> String {
        format!("LMDB (heed 0.20.5): {:?}", self.env.info())
    }

    pub async fn batch_insert(
        &self,
        table: String,
        data: Vec<(u128, Vec<u8>)>,
    ) -> Result<(), StorageError> {
        let env = self.env.clone();
        tokio::task::spawn_blocking(move || {
            let mut rw_txn = env.write_txn()?;
            let db = env.create_database::<U128<BigEndian>, Bytes>(&mut rw_txn, Some(&table))?;

            let keymap: HashMap<u128, &Vec<u8>> = data.iter().map(|(k, v)| (*k, v)).collect();
            let mut sorted_keys: Vec<u128> = keymap.keys().cloned().collect();
            sorted_keys.sort();

            for key in sorted_keys {
                if db.get(&rw_txn, &key)?.is_some() {
                    return Err(StorageError::KeyExists(key as u64));
                }
                db.put(&mut rw_txn, &key, keymap[&key])?;
            }
            rw_txn.commit()?;
            Ok(())
        })
        .await
        .expect("Failed to run tokio task")
    }

    pub async fn batch_get(
        &self,
        table: String,
        keys: Vec<u128>,
    ) -> Result<Vec<Option<Vec<u8>>>, StorageError> {
        let env = self.env.clone();
        tokio::task::spawn_blocking(move || {
            let ro_txn = env.read_txn()?;
            let db: Database<U128<BigEndian>, Bytes> = env
                .open_database(&ro_txn, Some(&table))?
                .ok_or(StorageError::TableError("Table not found".to_string()))?;
            let mut values = Vec::new();
            for key in keys {
                let value = db.get(&ro_txn, &key)?;
                if let Some(v) = value {
                    values.push(Some(v.to_vec()));
                } else {
                    values.push(None);
                }
            }
            Ok(values)
        })
        .await
        .expect("Failed to run tokio task")
    }

    pub async fn flush(&self) -> Result<(), StorageError> {
        let env = self.env.clone();
        tokio::task::spawn_blocking(move || {
            env.clear_stale_readers()?;
            env.force_sync()?;
            Ok(())
        })
        .await
        .expect("Failed to run tokio task")
    }

    pub async fn create_table(&self, table: String) -> Result<(), StorageError> {
        let env = self.env.clone();
        tokio::task::spawn_blocking(move || {
            let mut rw_txn = env.write_txn()?;
            env.create_database::<U128<BigEndian>, Bytes>(&mut rw_txn, Some(&table))?;
            rw_txn.commit()?;
            Ok(())
        })
        .await
        .expect("Failed to run tokio task")
    }

    pub async fn close(&self) -> Result<(), StorageError> {
        self.flush().await?;
        Ok(())
    }
}
