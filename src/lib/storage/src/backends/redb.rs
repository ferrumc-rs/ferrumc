use crate::errors::StorageError;
use crate::DatabaseBackend;
use async_trait::async_trait;
use parking_lot::RwLock;
use redb::TableDefinition;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub struct RedbBackend {
    db: Arc<RwLock<redb::Database>>,
}

#[async_trait]
impl DatabaseBackend for RedbBackend {
    async fn initialize(store_path: Option<PathBuf>) -> Result<Self, StorageError> {
        if let Some(path) = store_path {
            let db = if path.exists() {
                redb::Database::open(path)
                    .map_err(|e| StorageError::DatabaseInitError(e.to_string()))?
            } else {
                redb::Database::create(path)
                    .map_err(|e| StorageError::DatabaseInitError(e.to_string()))?
            };
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
        if self.exists(table.clone(), key).await? {
            return Err(StorageError::KeyExists(key));
        }
        tokio::task::spawn_blocking(move || {
            let table_def: TableDefinition<u64, &[u8]> = TableDefinition::new(&table);
            {
                let tx = db
                    .read()
                    .begin_write()
                    .map_err(|e| StorageError::WriteError(e.to_string()))?;
                {
                    let mut open_table = tx
                        .open_table(table_def)
                        .map_err(|e| StorageError::WriteError(e.to_string()))?;

                    open_table
                        .insert(key, value.as_slice())
                        .map_err(|e| StorageError::WriteError(e.to_string()))?;
                }
                tx.commit()
                    .map_err(|e| StorageError::CommitError(e.to_string()))?;
                Ok::<(), StorageError>(())
            }
        })
        .await
        .expect("Failed to insert data into database")?;
        Ok(())
    }

    async fn get(&self, table: String, key: u64) -> Result<Option<Vec<u8>>, StorageError> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let table_def: TableDefinition<u64, &[u8]> = TableDefinition::new(&table);
            let tx = db
                .read()
                .begin_read()
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            let open_table = tx
                .open_table(table_def)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            let value = open_table
                .get(key)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            if let Some(value) = value {
                Ok(Some(value.value().to_vec()))
            } else {
                Ok(None)
            }
        })
        .await
        .expect("Failed to spawn task")?;
        Ok(result)
    }

    async fn delete(&self, table: String, key: u64) -> Result<(), StorageError> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let table_def: TableDefinition<u64, &[u8]> = TableDefinition::new(&table);
            let tx = db
                .read()
                .begin_write()
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            #[allow(unused_assignments)]
            let mut did_exist = false;
            {
                let mut open_table = tx
                    .open_table(table_def)
                    .map_err(|e| StorageError::WriteError(e.to_string()))?;
                let value = open_table
                    .remove(key)
                    .map_err(|e| StorageError::WriteError(e.to_string()))?;
                did_exist = value.is_some();
            }
            tx.commit()
                .map_err(|e| StorageError::CommitError(e.to_string()))?;
            if did_exist {
                Ok(())
            } else {
                Err(StorageError::KeyNotFound(key))
            }
        })
        .await
        .expect("Failed to spawn task")
    }

    async fn update(
        &self,
        table: String,
        key: u64,
        value: Vec<u8>,
    ) -> Result<(), StorageError> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let table_def: TableDefinition<u64, &[u8]> = TableDefinition::new(&table);
            let tx = db
                .read()
                .begin_write()
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            {
                let mut open_table = tx
                    .open_table(table_def)
                    .map_err(|e| StorageError::WriteError(e.to_string()))?;

                let res = open_table
                    .insert(key, value.as_slice())
                    .map_err(|e| StorageError::WriteError(e.to_string()))?;
                if res.is_none() {
                    return Err(StorageError::KeyNotFound(key));
                }
            }
            tx.commit()
                .map_err(|e| StorageError::CommitError(e.to_string()))?;
            Ok(())
        })
        .await
        .expect("Failed to spawn task")
        .map_err(|e| StorageError::UpdateError(e.to_string()))
    }

    async fn upsert(
        &self,
        table: String,
        key: u64,
        value: Vec<u8>,
    ) -> Result<bool, StorageError> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let table_def: TableDefinition<u64, &[u8]> = TableDefinition::new(&table);
            let tx = db
                .read()
                .begin_write()
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            {
                let mut open_table = tx
                    .open_table(table_def)
                    .map_err(|e| StorageError::WriteError(e.to_string()))?;

                let res = open_table
                    .insert(key, value.as_slice())
                    .map_err(|e| StorageError::WriteError(e.to_string()))?;
                if res.is_none() {
                    return Ok(true);
                }
            }
            tx.commit()
                .map_err(|e| StorageError::WriteError(e.to_string()))?;
            Ok(false)
        })
        .await
        .expect("Failed to spawn task")?;
        Ok(result)
    }

    async fn exists(&self, table: String, key: u64) -> Result<bool, StorageError> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let table_def: TableDefinition<u64, &[u8]> = TableDefinition::new(&table);
            let tx = db
                .read()
                .begin_read()
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            let open_table = tx
                .open_table(table_def)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            let value = open_table
                .get(key)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            Ok(value.is_some())
        })
        .await
        .expect("Failed to spawn task")?;
        Ok(result)
    }

    async fn details(&self) -> String {
        "Redb 2.1.3".to_string()
    }

    async fn batch_insert(
        &self,
        table: String,
        data: Vec<(u64, Vec<u8>)>,
    ) -> Result<(), StorageError> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let table_def: TableDefinition<u64, &[u8]> = TableDefinition::new(&table);
            {
                let tx = db
                    .read()
                    .begin_write()
                    .map_err(|e| StorageError::WriteError(e.to_string()))?;
                {
                    let mut open_table = tx
                        .open_table(table_def)
                        .map_err(|e| StorageError::WriteError(e.to_string()))?;
                    for (key, value) in data {
                        open_table
                            .insert(key, value.as_slice())
                            .map_err(|e| StorageError::WriteError(e.to_string()))?;
                    }
                }
                tx.commit()
                    .map_err(|e| StorageError::WriteError(e.to_string()))?;
                Ok::<(), StorageError>(())
            }
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
            let table_def: TableDefinition<u64, &[u8]> = TableDefinition::new(&table);
            let tx = db
                .read()
                .begin_read()
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            let open_table = tx
                .open_table(table_def)
                .map_err(|e| StorageError::ReadError(e.to_string()))?;
            let mut values = Vec::new();
            for key in keys {
                let value = open_table
                    .get(key)
                    .map_err(|e| StorageError::ReadError(e.to_string()))?;
                if let Some(value) = value {
                    values.push(Some(value.value().to_vec()));
                } else {
                    values.push(None);
                }
            }
            Ok(values)
        })
        .await
        .expect("Failed to spawn task")?;
        Ok(result)
    }

    async fn flush(&self) -> Result<(), StorageError> {
        let db = self.db.clone();
        match tokio::task::spawn_blocking(move || {
            db.write()
                .compact()
                .map_err(|e| StorageError::FlushError(e.to_string()))
        })
        .await
        .expect("Failed to flush database")
        {
            Ok(_) => Ok(()),
            Err(e) => Err(StorageError::FlushError(e.to_string())),
        }
    }

    async fn create_table(&self, table: String) -> Result<(), StorageError> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let table_def: TableDefinition<u64, &[u8]> = TableDefinition::new(&table);
            {
                let tx = db
                    .read()
                    .begin_write()
                    .map_err(|e| StorageError::TableError(e.to_string()))?;
                {
                    tx.open_table(table_def)
                        .map_err(|e| StorageError::TableError(e.to_string()))?;
                }
                tx.commit()
                    .map_err(|e| StorageError::CommitError(e.to_string()))?;
                Ok::<(), StorageError>(())
            }
        })
        .await
        .expect("Failed to create table")?;
        Ok(())
    }

    async fn close(&self) -> Result<(), StorageError> {
        Ok(())
    }
}
