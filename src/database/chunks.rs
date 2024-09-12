use std::borrow::Cow;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;
use bincode::{Decode, Encode, config::standard};
use byteorder::LE;
use futures::channel::oneshot::{self, Canceled};
use heed::{types::U64, BytesDecode, BytesEncode, Env, MdbError};
use moka::future::Cache;
use tracing::{trace, warn};

use crate::{
    database::Database,
    utils::error::Error,
    utils::hash::hash,
    world::chunk_format::Chunk
};

use super::{LMDB_PAGE_SIZE, LMDB_PAGE_SIZE_INCREMENT, LMDB_READER_SYNC, LMDB_THREADPOOL};

pub struct Zstd<T>(PhantomData<T>);

impl<'a, T: Encode + 'a> BytesEncode<'a> for Zstd<T> {
    type EItem = T;

    fn bytes_encode(item: &'a Self::EItem) -> Result<Cow<'a, [u8]>, heed::BoxedError> {
        
        // Compress
        let mut bytes = Vec::new();
        let mut compressor = zstd::Encoder::new(&mut bytes, 6)?;
        bincode::encode_into_std_write(item, &mut compressor, standard())?;
        compressor.finish()?;
        
        Ok(Cow::Owned(bytes))
    }
}

impl<'a, T: Decode + 'a> BytesDecode<'a> for Zstd<T> {
    type DItem = T;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, heed::BoxedError> {
        
        let mut decompressor = zstd::Decoder::new(bytes)?;
        let decoded = bincode::decode_from_std_read(&mut decompressor, standard())?;
        Ok(decoded)
    }
}

/// LMDB will follow a linear growth as opposed to MDBX which
/// uses a geometric growth.
pub(super) fn new_page_size(old_size: usize) -> usize {
    old_size + LMDB_PAGE_SIZE_INCREMENT
}

// Will delegate a database operation to the database threadpool
pub(super) fn spawn_blocking_db<F, R>(db: Env, f: F) -> impl Future<Output = Result<Result<R,heed::Error>,Canceled>> 
where  
    F: Fn() -> Result<R,heed::Error> + Send + 'static,
    R: Send + 'static + std::fmt::Debug,
{
    let (tx,res) = oneshot::channel::<Result<R,heed::Error>>();
    
    let pool = LMDB_THREADPOOL.get().unwrap();
    pool.spawn(move || {
        
        let read_lock = LMDB_READER_SYNC.read()
            .expect("Database RWLock has been poisoned. A thread should have crashed somewhere.");
        
        let mut res = f();
        if let Err(heed::Error::Mdb(MdbError::MapFull)) = res {
            
            tracing::warn!("Database page is full. Resizing...");
            
            drop(read_lock);
            
            let _resize_guard = LMDB_READER_SYNC.write()
                .expect("Database RWLock has been poisoned. A thread should have crashed somewhere.");
            
            let mut global_size_lock = LMDB_PAGE_SIZE.lock().unwrap();
            let old_size = *global_size_lock;
            *global_size_lock = new_page_size(old_size);
            unsafe { db.resize(*global_size_lock).expect("Unable to resize LMDB environment.") };
            
            tracing::info!("Successfully resized LMDB page from {} MiB to {} MiB", (old_size / 1024usize.pow(2)), (*global_size_lock / 1024usize.pow(2)));
            
            drop(global_size_lock);
            drop(_resize_guard);
            
            res = f();
        } else {
            drop(read_lock)
        }
        
        if tx.send(res).is_err() {
            tracing::warn!("A database task has been unable to send its result because the receiver at other end have closed.")
        }
    });
    
    res
}

impl Database {
    
    // Close the database
    pub fn close(self) {
        let token = self.db.prepare_for_closing();
        token.wait();
    }
    
    /// Fetch chunk from database
    fn get_chunk_from_database(db: &Env, key: &u64) -> Result<Option<Chunk>, heed::Error> {
        // Initialize read transaction and open chunks table
        let ro_tx = db.read_txn()?;
        let database = db
            .open_database::<U64<LE>, Zstd<Chunk>>(&ro_tx, Some("chunks"))?
            .expect("No table \"chunks\" found. The database should have been initialized");

        // Attempt to fetch chunk from table
        database.get(&ro_tx, key)
        //.map_err(|err| Error::DatabaseError(format!("Failed to get chunk: {err}")))
    }

    /// Insert a single chunk into database
    fn insert_chunk_into_database(db: &Env, chunk: &Chunk) -> Result<(), heed::Error> {
        // Initialize write transaction and open chunks table
        let mut rw_tx = db.write_txn()?;
        let database = db
            .open_database::<U64<LE>, Zstd<Chunk>>(&rw_tx, Some("chunks"))?
            .expect("No table \"chunks\" found. The database should have been initialized");

        // Calculate key
        let key = hash((chunk.dimension.as_ref().unwrap(), chunk.x_pos, chunk.z_pos));

        // Insert chunk
        let res = database.put(&mut rw_tx, &key, chunk);
        rw_tx.commit()?;
        // .map_err(|err| {
        //     Error::DatabaseError(format!("Unable to commit changes to database: {err}"))
        // })?;

        res
        // if let Err(err) = res {
        //     Err(Error::DatabaseError(format!(
        //         "Failed to insert or update chunk: {err}"
        //     )))
        // } else {
        //     Ok(())
        // }
    }

    /// Insert multiple chunks into database
    /// TODO: Find better name/disambiguation
    fn insert_chunks_into_database(db: &Env, chunks: &[Chunk]) -> Result<(), heed::Error> {
        // Initialize write transaction and open chunks table
        let mut rw_tx = db.write_txn()?;
        let database = db
            .open_database::<U64<LE>, Zstd<Chunk>>(&rw_tx, Some("chunks"))?
            .expect("No table \"chunks\" found. The database should have been initialized");

        // Update page
        for chunk in chunks {
            // Calculate key
            let key = hash((chunk.dimension.as_ref().unwrap(), chunk.x_pos, chunk.z_pos));

            // Insert chunk
            database.put(&mut rw_tx, &key, chunk)?;
        }
        // Commit changes
        rw_tx.commit()?;
        Ok(())
    }

    async fn load_into_cache(&self, key: u64) -> Result<(), Error> {
        Database::load_into_cache_standalone(self.db.clone(), self.cache.clone(), key).await
    }

    async fn load_into_cache_standalone(db: Env, cache: Arc<Cache<u64, Chunk>>, key: u64) -> Result<(), Error> {
        let tsk_db = db.clone();

        tokio::task::spawn(async move {

            // Check cache
            if cache.contains_key(&key) {
                trace!("Chunk already exists in cache: {:X}", key);
            }
            // If not in cache then search in database
            else if let Ok(chunk) =
                spawn_blocking_db(tsk_db, move || Self::get_chunk_from_database(&db, &key))
                    .await
                    .unwrap()
            {
                if let Some(chunk) = chunk {
                    cache.insert(key, chunk).await;
                } else {
                    warn!(
                        "Chunk does not exist in db, can't load into cache: {:X}",
                        key,
                    );
                }
            }
            // The chunk don't exist
            else {
                warn!("Error getting chunk: {:X}", key,);
            }
        })
            .await?;
        Ok(())
    }
    /// Insert a chunk into the database <br>
    /// This will also insert the chunk into the cache <br>
    /// If the chunk already exists, it will return an error
    /// # Arguments
    /// * `value` - The chunk to insert
    /// # Returns
    /// * `Result<(), Error>` - Ok if the chunk was inserted, Err if the chunk already exists
    /// # Example
    /// ```no_run
    /// use crate::world::chunkformat::Chunk;
    /// use crate::database::Database;
    /// use crate::utils::error::Error;
    ///
    /// async fn insert_chunk(database: Database, chunk: Chunk) -> Result<(), Error> {
    ///    database.insert_chunk(chunk).await
    /// }
    ///
    /// ```
    pub async fn insert_chunk(&self, value: Chunk) -> Result<(), Error> {
        // Calculate key of this chunk
        // WARNING: This key wasn't supposed to include value.dimension in the tuple, but it was different from the key used in persistent database most likely a bug.
        let key = hash((value.dimension.as_ref().unwrap(), value.x_pos, value.z_pos));

        // Insert chunk into persistent database
        let chunk = value.clone();
        let db = self.db.clone();
        let tsk_db = self.db.clone();
        spawn_blocking_db(tsk_db, move || Self::insert_chunk_into_database(&db, &chunk))
            .await
            .unwrap()?;

        // Insert into cache
        self.cache.insert(key, value).await;
        Ok(())
    }

    /// Get a chunk from the database <br>
    /// This will also insert the chunk into the cache <br>
    /// If the chunk does not exist, it will return None
    /// # Arguments
    /// * `x` - The x position of the chunk
    /// * `z` - The z position of the chunk
    /// * `dimension` - The dimension of the chunk
    /// # Returns
    /// * `Result<Option<Chunk>, Error>` - Ok if the chunk was found, Err if the chunk does not exist
    /// # Example
    /// ```no_run
    /// use crate::world::chunkformat::Chunk;
    /// use crate::database::Database;
    /// use crate::utils::error::Error;
    ///
    /// async fn get_chunk(database: Database, x: i32, z: i32, dimension: String) -> Result<Option<Chunk>, Error> {
    ///   database.get_chunk(x, z, dimension).await
    /// }
    ///
    /// ```
    pub async fn get_chunk(
        &self,
        x: i32,
        z: i32,
        dimension: String,
    ) -> Result<Option<Chunk>, Error> {
        // Calculate key of this chunk and clone database pointer
        let key = hash((dimension, x, z));
        let tsk_db = self.db.clone();
        let db = self.db.clone();

        // First check cache
        if self.cache.contains_key(&key) {
            Ok(self.cache.get(&key).await)
        }
        // Attempt to get chunk from persistent database
        else if let Some(chunk) = spawn_blocking_db(tsk_db, move || Self::get_chunk_from_database(&db, &key))
            .await
            .unwrap()?
        {
            self.cache.insert(key, chunk.clone()).await;
            Ok(Some(chunk))
        }
        // Chunk do not exist
        else {
            Ok(None)
        }
    }

    /// Check if a chunk exists in the database
    /// # Arguments
    /// * `x` - The x position of the chunk
    /// * `z` - The z position of the chunk
    /// * `dimension` - The dimension of the chunk
    /// # Returns
    ///
    /// * `Result<bool, Error>` - Ok if the chunk exists, Err if the chunk does not exist
    /// # Example
    /// ```no_run
    /// use crate::database::Database;
    /// use crate::utils::error::Error;
    ///
    /// async fn chunk_exists(database: Database, x: i32, z: i32, dimension: String) -> Result<bool, Error> {
    ///  database.chunk_exists(x, z, dimension).await
    /// }
    ///
    /// ```
    pub async fn chunk_exists(&self, x: i32, z: i32, dimension: String) -> Result<bool, Error> {
        // Calculate key and copy database pointer
        let key = hash((dimension, x, z));
        let tsk_db = self.db.clone();
        let db = self.db.clone();

        // Check first cache
        if self.cache.contains_key(&key) {
            Ok(true)
        // Else check persistent database and load it into cache
        } else {
            let res = spawn_blocking_db(tsk_db, move || Self::get_chunk_from_database(&db, &key)).await.unwrap();

            // WARNING: The previous logic was to order the chunk to be loaded into cache whether it existed or not.
            // This has been replaced by directly loading the queried chunk into cache
            match res {
                Ok(opt) => {
                    let exist = opt.is_some();
                    if let Some(chunk) = opt {
                        self.cache.insert(key, chunk).await;
                    }
                    Ok(exist)
                }
                Err(err) => Err(Error::LmdbError(err)),
            }
        }
    }

    /// Update a chunk in the database <br>
    /// This will also update the chunk in the cache <br>
    /// If the chunk does not exist, it will return an error
    /// # Arguments
    /// * `value` - The chunk to update
    /// # Returns
    /// * `Result<(), Error>` - Ok if the chunk was updated, Err if the chunk does not exist
    /// # Example
    /// ```no_run
    /// use crate::world::chunkformat::Chunk;
    /// use crate::database::Database;
    /// use crate::utils::error::Error;
    ///
    /// async fn update_chunk(database: Database, chunk: Chunk) -> Result<(), Error> {
    ///   database.update_chunk(chunk).await
    /// }
    ///
    /// ```
    pub async fn update_chunk(&self, value: Chunk) -> Result<(), Error> {
        // Calculate key of this chunk
        // WARNING: This key wasn't supposed to include value.dimension in the tuple, but it was different from the key used in persistent database most likely a bug.
        let key = hash((value.dimension.as_ref().unwrap(), value.x_pos, value.z_pos));

        // Insert new chunk state into persistent database
        let chunk = value.clone();
        let db = self.db.clone();
        let tsk_db = self.db.clone();
        spawn_blocking_db(tsk_db, move || Self::insert_chunk_into_database(&db, &chunk)).await.unwrap()?;

        // Insert new chunk state into cache
        self.cache.insert(key, value).await;
        Ok(())
    }

    /// Batch insert chunks into the database <br>
    /// This will also insert the chunks into the cache <br>
    /// If any of the chunks already exist, it will return an error
    /// # Arguments
    /// * `values` - The chunks to insert
    /// # Returns
    /// * `Result<(), Error>` - Ok if the chunks were inserted, Err if any of the chunks already exist
    /// # Example
    /// ```no_run
    /// use crate::world::chunkformat::Chunk;
    /// use crate::database::Database;
    /// use crate::utils::error::Error;
    ///
    /// async fn batch_insert_chunks(database: Database, chunks: Vec<Chunk>) -> Result<(), Error> {
    ///  database.batch_insert_chunks(chunks).await
    /// }
    ///
    /// ```
    pub async fn batch_insert(&self, values: Vec<Chunk>) -> Result<(), Error> {
        // Clone database pointer
        let db = self.db.clone();
        let tsk_db = self.db.clone();

        // Calculate all keys
        let keys = values
            .iter()
            .map(|v| hash((v.dimension.as_ref().unwrap_or_else(|| panic!("Invalid chunk @ ({},{})", v.x_pos, v.z_pos)), v.x_pos, v.z_pos)))
            .collect::<Vec<u64>>();

        // WARNING: The previous logic was to first insert in database and then insert in cache using load_into_cache fn.
        // This has been modified to avoid having to query database while we already have the data available.
        // First insert into cache

        for (key, chunk) in keys.into_iter().zip(&values) {
            let cache = self.cache.clone();
            let db = self.db.clone();
            let chunk = chunk.clone();
            tokio::spawn(async move {
                cache.insert(key, chunk).await;
                if let Err(e) = Database::load_into_cache_standalone(db, cache, key).await {
                    warn!("Error inserting chunk into database: {:?}", e);
                }
            });
        }
        
        // Then insert into persistent database
        spawn_blocking_db(tsk_db, move || Self::insert_chunks_into_database(&db, &values))
            .await
            .unwrap()?;
        
        Ok(())
    }
}

#[tokio::test]
#[ignore]
async fn dump_chunk() {
    use crate::utils::setup_logger;
    use tokio::net::TcpListener;
    setup_logger().unwrap();
    let state = crate::create_state(TcpListener::bind("0.0.0.0:0").await.unwrap())
        .await
        .unwrap();
    let chunk = state
        .database
        .get_chunk(0, 0, "overworld".to_string())
        .await
        .unwrap()
        .unwrap();
    let outfile = std::fs::File::create("chunk.json").unwrap();
    let mut writer = std::io::BufWriter::new(outfile);
    serde_json::to_writer(&mut writer, &chunk).unwrap();
}
