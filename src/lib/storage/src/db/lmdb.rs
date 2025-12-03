use crate::errors::StorageError;

use heed::byteorder::BigEndian;
use heed::types::{Bytes, U128};
use heed::{Database, Env, EnvOpenOptions, WithoutTls};
use parking_lot::Mutex;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LmdbBackend {
    env: Arc<Mutex<Env<WithoutTls>>>,
}

// Map Heed errors to our StorageError
impl From<heed::Error> for StorageError {
    fn from(err: heed::Error) -> Self {
        match err {
            heed::Error::Io(e) => StorageError::GenericIoError(e),
            heed::Error::Encoding(e) => StorageError::WriteError(e.to_string()),
            heed::Error::Decoding(e) => StorageError::ReadError(e.to_string()),
            _ => StorageError::DatabaseError(err.to_string()),
        }
    }
}

impl LmdbBackend {
    pub fn initialize(store_path: Option<PathBuf>) -> Result<Self, StorageError>
    where
        Self: Sized,
    {
        let Some(checked_path) = store_path else {
            return Err(StorageError::InvalidPath);
        };

        if !checked_path.exists() {
            std::fs::create_dir_all(&checked_path)?;
        }

        // Calculate map size (GB -> Bytes)
        let config = ferrumc_config::server_config::get_global_config();
        let map_size = config.database.map_size as usize * 1024 * 1024 * 1024;

        // Round to page size
        let page_size = page_size::get();
        let rounded_map_size = ((map_size + page_size - 1) / page_size) * page_size;

        unsafe {
            let env = EnvOpenOptions::new()
                .read_txn_without_tls()
                // Increased from 2 -> 32 to support players, stats, etc.
                .max_dbs(32)
                .map_size(rounded_map_size)
                .open(checked_path)
                .map_err(|e| StorageError::DatabaseInitError(e.to_string()))?;

            Ok(LmdbBacerror[E0308]: mismatched types
                --> src/lib/protocol/src/packets/outgoing/play/chunk_and_light_data.rs:71:25
                 |
              71 |             heightmaps: LengthPrefixedVec::default(),
                 |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `RawNbt`, found `LengthPrefixedVec<_>`
                 |
                 = note: expected struct `RawNbt`
                            found struct `LengthPrefixedVec<_>`
              
              kend {
                env: Arc::new(Mutex::new(env)),
            })
        }
    }

    /// Creates a table (database) if it doesn't exist.
    pub fn create_table(&self, table: String) -> Result<(), StorageError> {
        let env = self.env.lock();
        let mut rw_txn = env.write_txn()?;
        env.create_database::<U128<BigEndian>, Bytes>(&mut rw_txn, Some(&table))?;
        rw_txn.commit()?;
        Ok(())
    }

    /// Checks if a table exists.
    pub fn table_exists(&self, table: String) -> Result<bool, StorageError> {
        let env = self.env.lock();
        let ro_txn = env.read_txn()?;
        let db = env.open_database::<U128<BigEndian>, Bytes>(&ro_txn, Some(&table))?;
        Ok(db.is_some())
    }

    /// Inserts a value. Fails if key already exists.
    pub fn insert(&self, table: String, key: u128, value: &[u8]) -> Result<(), StorageError> {
        let env = self.env.lock();
        let mut rw_txn = env.write_txn()?;
        let db = env.create_database::<U128<BigEndian>, Bytes>(&mut rw_txn, Some(&table))?;

        if db.get(&rw_txn, &key)?.is_some() {
            return Err(StorageError::KeyExists(key as u64));
        }

        db.put(&mut rw_txn, &key, value)?;
        rw_txn.commit()?;
        Ok(())
    }

    /// Inserts or Updates a value. Always succeeds.
    pub fn upsert(&self, table: String, key: u128, value: &[u8]) -> Result<(), StorageError> {
        let env = self.env.lock();
        let mut rw_txn = env.write_txn()?;
        // create_database handles opening existing ones too
        let db = env.create_database::<U128<BigEndian>, Bytes>(&mut rw_txn, Some(&table))?;

        db.put(&mut rw_txn, &key, value)?;
        rw_txn.commit()?;
        Ok(())
    }

    /// Retrieves a value.
    pub fn get(&self, table: String, key: u128) -> Result<Option<Vec<u8>>, StorageError> {
        let env = self.env.lock();
        let ro_txn = env.read_txn()?;

        // We return error if table is missing, rather than just None, to be explicit
        let db = env
            .open_database::<U128<BigEndian>, Bytes>(&ro_txn, Some(&table))?
            .ok_or_else(|| StorageError::TableError(format!("Table '{}' not found", table)))?;

        let value = db.get(&ro_txn, &key)?;
        Ok(value.map(|v| v.to_vec()))
    }

    /// Deletes a value. Fails if key not found.
    pub fn delete(&self, table: String, key: u128) -> Result<(), StorageError> {
        let env = self.env.lock();
        let mut rw_txn = env.write_txn()?;

        let db = env
            .open_database::<U128<BigEndian>, Bytes>(&rw_txn, Some(&table))?
            .ok_or_else(|| StorageError::TableError(format!("Table '{}' not found", table)))?;

        if db.get(&rw_txn, &key)?.is_none() {
            return Err(StorageError::KeyNotFound(key as u64));
        }

        db.delete(&mut rw_txn, &key)?;
        rw_txn.commit()?;
        Ok(())
    }

    /// Retrieving multiple keys at once.
    pub fn batch_get(
        &self,
        table: String,
        keys: Vec<u128>,
    ) -> Result<Vec<Option<Vec<u8>>>, StorageError> {
        let env = self.env.lock();
        let ro_txn = env.read_txn()?;

        let db = env
            .open_database::<U128<BigEndian>, Bytes>(&ro_txn, Some(&table))?
            .ok_or_else(|| StorageError::TableError(format!("Table '{}' not found", table)))?;

        let mut values = Vec::with_capacity(keys.len());
        for key in keys {
            let value = db.get(&ro_txn, &key)?;
            values.push(value.map(|v| v.to_vec()));
        }
        Ok(values)
    }

    /// Flushes buffers to disk.
    pub fn flush(&self) -> Result<(), StorageError> {
        let env = self.env.lock();
        env.force_sync()?;
        Ok(())
    }

    /// Closes the environment (flushes first).
    pub fn close(&self) -> Result<(), StorageError> {
        self.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;
    use tempfile::tempdir;

    #[test]
    fn test_write_read() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();

        let backend = LmdbBackend::initialize(Some(path.clone())).unwrap();
        backend.create_table("test".to_string()).unwrap();

        let key = 12345u128;
        let value = vec![1, 2, 3, 4];

        backend.insert("test".to_string(), key, &value).unwrap();

        let result = backend.get("test".to_string(), key).unwrap();
        assert_eq!(result, Some(value));

        // Cleanup handled by tempdir
    }
}
