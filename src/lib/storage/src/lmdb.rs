use crate::errors::StorageError;
use heed;
use heed::byteorder::BigEndian;
use heed::types::{Bytes, U128};
use heed::{Database, Env, EnvFlags, EnvOpenOptions, WithoutTls};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// All tables share the same key/value encoding: a 128-bit big-endian key (a hashed chunk/player
/// identifier) mapping to an opaque byte blob.
type TableDb = Database<U128<BigEndian>, Bytes>;

/// LMDB-backed key/value store.
///
/// The environment is shared directly (`Arc<Env>`) rather than behind a `Mutex`. LMDB is built for
/// concurrency: any number of read transactions run in parallel with no locking (MVCC), and writes
/// are serialised by LMDB's own single-writer lock inside [`Env::write_txn`]. Wrapping the whole
/// `Env` in a `Mutex` would serialise *reads* behind *writes* as well, throwing that away — which is
/// why this type holds the `Env` directly and lets LMDB do the synchronisation.
///
/// Database handles (`dbi`s) are opened once and cached. Opening a handle requires a transaction, so
/// re-opening it on every operation (as an earlier version did) added a needless transaction and a
/// named-database lookup to every read and write. A [`Database`] is a cheap `Copy` handle valid for
/// the life of the environment, so it is opened lazily on first use and reused thereafter.
#[derive(Debug, Clone)]
pub struct LmdbBackend {
    env: Arc<Env<WithoutTls>>,
    databases: Arc<RwLock<HashMap<String, TableDb>>>,
}

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
    pub fn initialize(store_path: Option<PathBuf>, map_size: usize) -> Result<Self, StorageError>
    where
        Self: Sized,
    {
        let Some(checked_path) = store_path else {
            return Err(StorageError::InvalidPath);
        };
        if !checked_path.exists() {
            std::fs::create_dir_all(&checked_path)?;
        }
        let rounded_map_size = ((map_size as f64 / page_size::get() as f64).round()
            * page_size::get() as f64) as usize;

        let mut opts = EnvOpenOptions::new().read_txn_without_tls();
        // Change `max_dbs` as more tables are needed.
        opts.max_dbs(3).map_size(rounded_map_size);
        // SAFETY: `NO_SYNC` trades per-commit durability for throughput. Each commit is still
        // written to the OS page cache (so it is immediately visible to all readers and survives a
        // *process* crash); only an OS/power loss can drop commits made since the last
        // `force_sync`. The world sync schedule calls `flush` (→ `force_sync`) periodically, which
        // bounds that window, and chunk data is regenerable, so the trade is favourable. On a
        // filesystem with ordered writeback `NO_SYNC` only ever loses whole recent commits — it
        // never corrupts the database.
        unsafe {
            opts.flags(EnvFlags::NO_SYNC);
        }
        // SAFETY: `open` is unsafe because LMDB requires that a single environment path not be
        // opened twice with incompatible settings; this is the sole opener for `checked_path`.
        let env = unsafe {
            opts.open(&checked_path)
                .map_err(|e| StorageError::DatabaseInitError(e.to_string()))?
        };

        Ok(LmdbBackend {
            env: Arc::new(env),
            databases: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Returns the cached handle for `table`, opening it from disk on first use. Returns `Ok(None)`
    /// if the table does not exist; the caller decides whether a missing table is an error.
    fn open_table(&self, table: &str) -> Result<Option<TableDb>, StorageError> {
        if let Some(db) = self.databases.read().get(table) {
            return Ok(Some(*db));
        }
        let mut cache = self.databases.write();
        // Re-check under the write lock: another thread may have opened the table between dropping
        // the read lock above and acquiring the write lock here.
        if let Some(db) = cache.get(table) {
            return Ok(Some(*db));
        }
        // The handle must be opened inside a *write* transaction that is then committed. LMDB ties a
        // newly opened database handle to the transaction that opened it: it stays private to that
        // transaction until commit, and is closed automatically if the transaction is aborted. A
        // read transaction has no commit (dropping it aborts), so a handle opened in one would be
        // closed the moment the transaction is dropped — leaving a dangling `dbi` that fails with
        // `EINVAL` when reused in a later transaction. Opening in a committed write transaction
        // promotes the handle into the shared environment so it remains valid for the env's lifetime.
        let wtxn = self.env.write_txn()?;
        let opened = self
            .env
            .open_database::<U128<BigEndian>, Bytes>(&wtxn, Some(table))?;
        wtxn.commit()?;
        if let Some(db) = opened {
            cache.insert(table.to_string(), db);
        }
        Ok(opened)
    }

    /// Returns the cached handle for `table`, creating the table if it does not yet exist.
    fn open_or_create_table(&self, table: &str) -> Result<TableDb, StorageError> {
        if let Some(db) = self.databases.read().get(table) {
            return Ok(*db);
        }
        let mut cache = self.databases.write();
        if let Some(db) = cache.get(table) {
            return Ok(*db);
        }
        let mut wtxn = self.env.write_txn()?;
        let db = self
            .env
            .create_database::<U128<BigEndian>, Bytes>(&mut wtxn, Some(table))?;
        wtxn.commit()?;
        cache.insert(table.to_string(), db);
        Ok(db)
    }

    pub fn insert(&self, table: String, key: u128, value: Vec<u8>) -> Result<(), StorageError> {
        let db = self.open_or_create_table(&table)?;
        let mut rw_txn = self.env.write_txn()?;
        if db.get(&rw_txn, &key)?.is_some() {
            return Err(StorageError::KeyExists(key as u64));
        }
        db.put(&mut rw_txn, &key, &value)?;
        rw_txn.commit()?;
        Ok(())
    }

    pub fn get(&self, table: String, key: u128) -> Result<Option<Vec<u8>>, StorageError> {
        let Some(db) = self.open_table(&table)? else {
            return Err(StorageError::TableError("Table not found".to_string()));
        };
        let ro_txn = self.env.read_txn()?;
        let value = db.get(&ro_txn, &key)?.map(<[u8]>::to_vec);
        Ok(value)
    }

    pub fn delete(&self, table: String, key: u128) -> Result<(), StorageError> {
        let Some(db) = self.open_table(&table)? else {
            return Err(StorageError::TableError("Table not found".to_string()));
        };
        let mut rw_txn = self.env.write_txn()?;
        if db.get(&rw_txn, &key)?.is_none() {
            return Err(StorageError::KeyNotFound(key as u64));
        }
        db.delete(&mut rw_txn, &key)?;
        rw_txn.commit()?;
        Ok(())
    }

    pub fn update(&self, table: String, key: u128, value: Vec<u8>) -> Result<(), StorageError> {
        let Some(db) = self.open_table(&table)? else {
            return Err(StorageError::TableError("Table not found".to_string()));
        };
        let mut rw_txn = self.env.write_txn()?;
        if db.get(&rw_txn, &key)?.is_none() {
            return Err(StorageError::KeyNotFound(key as u64));
        }
        db.put(&mut rw_txn, &key, &value)?;
        rw_txn.commit()?;
        Ok(())
    }

    pub fn upsert(&self, table: String, key: u128, value: Vec<u8>) -> Result<bool, StorageError> {
        let Some(db) = self.open_table(&table)? else {
            return Err(StorageError::TableError("Table not found".to_string()));
        };
        let mut rw_txn = self.env.write_txn()?;
        db.put(&mut rw_txn, &key, &value)?;
        rw_txn.commit()?;
        Ok(true)
    }

    pub fn batch_upsert(
        &self,
        table: String,
        data: Vec<(u128, Vec<u8>)>,
    ) -> Result<(), StorageError> {
        let db = self.open_or_create_table(&table)?;
        let mut rw_txn = self.env.write_txn()?;

        // Insert in sorted key order. LMDB stores keys in B-tree order, so feeding sorted keys keeps
        // page splits sequential and the write cheaper than random insertion order.
        let keymap: HashMap<u128, &Vec<u8>> = data.iter().map(|(k, v)| (*k, v)).collect();
        let mut sorted_keys: Vec<u128> = keymap.keys().copied().collect();
        sorted_keys.sort_unstable();

        for key in sorted_keys {
            db.put(&mut rw_txn, &key, keymap[&key])?;
        }

        rw_txn.commit()?;
        Ok(())
    }

    pub fn exists(&self, table: String, key: u128) -> Result<bool, StorageError> {
        let Some(db) = self.open_table(&table)? else {
            return Err(StorageError::TableError("Table not found".to_string()));
        };
        let ro_txn = self.env.read_txn()?;
        Ok(db.get(&ro_txn, &key)?.is_some())
    }

    pub fn table_exists(&self, table: String) -> Result<bool, StorageError> {
        Ok(self.open_table(&table)?.is_some())
    }

    pub fn details(&self) -> String {
        format!("LMDB (heed 0.22): {:?}", self.env.info())
    }

    pub fn batch_insert(
        &self,
        table: String,
        data: Vec<(u128, Vec<u8>)>,
    ) -> Result<(), StorageError> {
        let db = self.open_or_create_table(&table)?;
        let mut rw_txn = self.env.write_txn()?;

        let keymap: HashMap<u128, &Vec<u8>> = data.iter().map(|(k, v)| (*k, v)).collect();
        let mut sorted_keys: Vec<u128> = keymap.keys().copied().collect();
        sorted_keys.sort_unstable();

        for key in sorted_keys {
            if db.get(&rw_txn, &key)?.is_some() {
                return Err(StorageError::KeyExists(key as u64));
            }
            db.put(&mut rw_txn, &key, keymap[&key])?;
        }
        rw_txn.commit()?;
        Ok(())
    }

    pub fn batch_get(
        &self,
        table: String,
        keys: Vec<u128>,
    ) -> Result<Vec<Option<Vec<u8>>>, StorageError> {
        let Some(db) = self.open_table(&table)? else {
            return Err(StorageError::TableError("Table not found".to_string()));
        };
        let ro_txn = self.env.read_txn()?;
        let mut values = Vec::with_capacity(keys.len());
        for key in keys {
            values.push(db.get(&ro_txn, &key)?.map(<[u8]>::to_vec));
        }
        Ok(values)
    }

    pub fn flush(&self) -> Result<(), StorageError> {
        self.env.clear_stale_readers()?;
        self.env.force_sync()?;
        Ok(())
    }

    pub fn create_table(&self, table: String) -> Result<(), StorageError> {
        self.open_or_create_table(&table)?;
        Ok(())
    }

    pub fn close(&self) -> Result<(), StorageError> {
        self.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;
    use std::hash::Hasher;
    use tempfile::tempdir;

    fn hash_2_to_u128(a: u64, b: u64) -> u128 {
        let mut hasher = wyhash::WyHash::with_seed(0);
        hasher.write_u64(a);
        hasher.write_u64(b);
        hasher.finish() as u128
    }

    #[test]
    fn test_write() {
        let path = tempdir().unwrap().keep();
        {
            let backend =
                LmdbBackend::initialize(Some(path.clone()), 10 * 1024 * 1024 * 1024).unwrap();
            backend.create_table("test_table".to_string()).unwrap();
            let key = 12345678901234567890u128;
            let value = vec![1, 2, 3, 4, 5];
            backend
                .insert("test_table".to_string(), key, value.clone())
                .unwrap();
            let retrieved_value = backend.get("test_table".to_string(), key).unwrap();
            assert_eq!(retrieved_value, Some(value));
        }
        remove_dir_all(path).unwrap();
    }

    /// Reopening an existing environment must be able to read tables that were created by a
    /// previous process. This exercises [`LmdbBackend::open_table`], the path taken for a table that
    /// already exists on disk but is not yet in the in-process handle cache. A handle opened inside
    /// a read transaction is closed when that transaction is aborted, so caching and reusing it
    /// later fails with `EINVAL`; opening it in a committed write transaction keeps it valid.
    #[test]
    fn test_reopen_reads_existing_table() {
        let path = tempdir().unwrap().keep();
        let key = 12345678901234567890u128;
        let value = vec![9, 8, 7, 6];
        {
            let backend =
                LmdbBackend::initialize(Some(path.clone()), 10 * 1024 * 1024 * 1024).unwrap();
            backend.create_table("test_table".to_string()).unwrap();
            backend
                .insert("test_table".to_string(), key, value.clone())
                .unwrap();
            backend.flush().unwrap();
        }
        // Fresh backend over the same path with an empty handle cache, mirroring a server restart.
        {
            let backend =
                LmdbBackend::initialize(Some(path.clone()), 10 * 1024 * 1024 * 1024).unwrap();
            assert!(backend.exists("test_table".to_string(), key).unwrap());
            let retrieved_value = backend.get("test_table".to_string(), key).unwrap();
            assert_eq!(retrieved_value, Some(value));
        }
        remove_dir_all(path).unwrap();
    }

    #[test]
    fn test_batch_insert() {
        let path = tempdir().unwrap().keep();
        {
            let backend =
                LmdbBackend::initialize(Some(path.clone()), 10 * 1024 * 1024 * 1024).unwrap();
            backend.create_table("test_table".to_string()).unwrap();
            let data = vec![
                (12345678901234567890u128, vec![1, 2, 3]),
                (12345678901234567891u128, vec![4, 5, 6]),
            ];
            backend
                .batch_insert("test_table".to_string(), data.clone())
                .unwrap();
            for (key, value) in data {
                let retrieved_value = backend.get("test_table".to_string(), key).unwrap();
                assert_eq!(retrieved_value, Some(value));
            }
        }
        remove_dir_all(path).unwrap();
    }

    #[test]
    fn test_concurrent_write() {
        let path = tempdir().unwrap().keep();
        {
            let backend =
                LmdbBackend::initialize(Some(path.clone()), 10 * 1024 * 1024 * 1024).unwrap();
            backend.create_table("test_table".to_string()).unwrap();
            let mut threads = vec![];
            for thread_iter in 0..10 {
                let handle = std::thread::spawn({
                    let backend = backend.clone();
                    move || {
                        for iter in 0..100 {
                            let key = hash_2_to_u128(iter, thread_iter);
                            let value = vec![rand::random::<u8>(); 10];
                            backend
                                .insert("test_table".to_string(), key, value)
                                .unwrap();
                        }
                    }
                });
                threads.push(handle);
            }
            for handle in threads {
                handle.join().unwrap();
            }
        }
        remove_dir_all(path).unwrap();
    }

    #[test]
    fn test_concurrent_read() {
        let path = tempdir().unwrap().keep();
        {
            let backend =
                LmdbBackend::initialize(Some(path.clone()), 10 * 1024 * 1024 * 1024).unwrap();
            backend.create_table("test_table".to_string()).unwrap();
            for thread_iter in 0..10 {
                for iter in 0..100 {
                    let value = vec![rand::random::<u8>(); 10];
                    let key = hash_2_to_u128(iter, thread_iter);
                    backend
                        .insert("test_table".to_string(), key, value)
                        .unwrap();
                }
            }
            let mut threads = vec![];
            for thread_iter in 0..10 {
                let handle = std::thread::spawn({
                    let backend = backend.clone();
                    move || {
                        for iter in 0..100 {
                            let key = hash_2_to_u128(iter, thread_iter);
                            let _ = backend.get("test_table".to_string(), key).unwrap();
                        }
                    }
                });
                threads.push(handle);
            }
            for handle in threads {
                handle.join().unwrap();
            }
        }
        remove_dir_all(path).unwrap();
    }
}
