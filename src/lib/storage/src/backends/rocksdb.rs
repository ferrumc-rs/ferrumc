use crate::errors::StorageError;
use crate::DatabaseBackend;
use parking_lot::RwLock;
use rocksdb::DB;
use std::path::PathBuf;
use std::sync::Arc;

pub struct RocksDBBackend {
    db: Arc<RwLock<DB>>,
}

impl DatabaseBackend for RocksDBBackend {
    async fn initialize(store_path: Option<PathBuf>) -> Result<Self, StorageError>
    where
        Self: Sized,
    {
        let mut options = rocksdb::Options::default();
        options.create_if_missing(true);
        options.create_missing_column_families(true);
        options.set_compression_options_parallel_threads(4);
        options.set_max_background_jobs(4);
        options.set_max_open_files(1000);
        options.increase_parallelism(4);
        options.set_allow_mmap_writes(true);
        options.set_allow_mmap_reads(true);
        if let Some(path) = store_path {
            let db = DB::open(&options, path)
                .map_err(|e| StorageError::DatabaseInitError(e.to_string()))?;
            Ok(Self {
                db: Arc::new(RwLock::new(db)),
            })
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
            let db = db.read();
            let cf = db.cf_handle(&table).unwrap();
            db.put_cf(cf, key.to_be_bytes(), &value)
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
            let db = db.read();
            let cf = db.cf_handle(&table).unwrap();
            let value = db
                .get_cf(cf, key.to_be_bytes())
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

    async fn delete(&mut self, table: String, key: u64) -> Result<(), StorageError> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let db = db.read();
            let cf = db.cf_handle(&table).unwrap();
            db.delete_cf(cf, key.to_be_bytes())
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            Ok::<(), StorageError>(())
        })
        .await
        .expect("Failed to delete data from database")?;
        Ok(())
    }

    async fn update(
        &mut self,
        table: String,
        key: u64,
        value: Vec<u8>,
    ) -> Result<(), StorageError> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let db = db.read();
            let cf = db.cf_handle(&table).unwrap();
            db.put_cf(cf, key.to_be_bytes(), &value)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            Ok::<(), StorageError>(())
        })
        .await
        .expect("Failed to update data in database")?;
        Ok(())
    }

    async fn upsert(
        &mut self,
        table: String,
        key: u64,
        value: Vec<u8>,
    ) -> Result<bool, StorageError> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let db = db.read();
            let cf = db.cf_handle(&table).unwrap();
            db.put_cf(cf, key.to_be_bytes(), &value)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            if let Ok(Some(_)) = db.get_cf(cf, key.to_be_bytes()) {
                Ok(true)
            } else {
                Ok(false)
            }
        })
        .await
        .expect("Failed to upsert data in database")?;
        Ok(result)
    }

    async fn exists(&mut self, table: String, key: u64) -> Result<bool, StorageError> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let db = db.read();
            let cf = db.cf_handle(&table).unwrap();
            let value = db
                .get_cf(cf, key.to_be_bytes())
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            Ok(value.is_some())
        })
        .await
        .expect("Failed to check if key exists in database")?;
        Ok(result)
    }

    async fn details(&mut self) -> String {
        "RocksDB 0.22.0".to_string()
    }

    async fn batch_insert(
        &mut self,
        table: String,
        data: Vec<(u64, Vec<u8>)>,
    ) -> Result<(), StorageError> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let db = db.read();
            let cf = db.cf_handle(&table).unwrap();
            let mut batch = rocksdb::WriteBatch::default();
            for (key, value) in data {
                batch.put_cf(cf, key.to_be_bytes(), &value);
            }
            db.write(batch)
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            Ok::<(), StorageError>(())
        })
        .await
        .expect("Failed to batch insert data into database")?;
        Ok(())
    }

    async fn batch_get(
        &mut self,
        table: String,
        keys: Vec<u64>,
    ) -> Result<Vec<Option<Vec<u8>>>, StorageError> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let db = db.read();
            let cf = db.cf_handle(&table).unwrap();
            let mut values = Vec::new();
            for key in keys {
                let value = db
                    .get_cf(cf, key.to_be_bytes())
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
        .expect("Failed to batch get data from database")?;
        Ok(result)
    }

    async fn flush(&mut self) -> Result<(), StorageError> {
        self.db
            .read()
            .flush()
            .map_err(|e| StorageError::WriteError(e.to_string()))?;
        Ok(())
    }

    async fn create_table(&mut self, table: String) -> Result<(), StorageError> {
        self.db
            .write()
            .create_cf(&table, &rocksdb::Options::default())
            .map_err(|e| StorageError::WriteError(e.to_string()))?;
        Ok(())
    }

    async fn close(&mut self) -> Result<(), StorageError> {
        self.flush().await?;
        self.db
            .read()
            .flush_wal(true)
            .map_err(|e| StorageError::WriteError(e.to_string()))?;
        Ok(())
    }
}
