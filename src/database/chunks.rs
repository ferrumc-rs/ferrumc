use byteorder::LE;
use heed::types::Bytes;
use heed::{types::U64, Env};
use moka::future::Cache;
use std::sync::Arc;
use tokio::runtime::Handle;
use tracing::{trace, warn};

use super::spawn_blocking_db;
use crate::database::encoding::ZstdCodec;
use crate::world::importing::SerializedChunk;
use crate::{
    database::Database, utils::error::Error, utils::hash::hash, world::chunk_format::Chunk,
};

impl Database {
    // Close the database
    pub fn close(self) {
        let token = self.db.prepare_for_closing();
        token.wait();
    }

    /// Fetch chunk from database
    async fn get_chunk_from_database(db: &Env, key: &u64) -> Result<Option<Chunk>, heed::Error> {
        let data = {
            // Initialize read transaction and open chunks table
            let ro_tx = db.read_txn()?;
            let database = db
                .open_database::<U64<LE>, Bytes>(&ro_tx, Some("chunks"))?
                .expect("No table \"chunks\" found. The database should have been initialized");

            // Attempt to fetch chunk from table
            let data = database.get(&ro_tx, key)?;

            data.map(|data| data.to_vec())
        };

        // Now, proceed with the async operation without holding `ro_tx`
        if let Some(data) = data {
            let chunk = ZstdCodec::decompress_data::<Chunk>(data.as_slice())
                .await
                .expect("Failed to decompress chunk");
            Ok(Some(chunk))
        } else {
            Ok(None)
        }
    }

    /// Insert a single chunk into database
    fn insert_chunk_into_database(db: &Env, chunk: &Chunk) -> Result<(), heed::Error> {
        // Initialize write transaction and open chunks table
        let mut rw_tx = db.write_txn()?;
        let database = db
            .open_database::<U64<LE>, Bytes>(&rw_tx, Some("chunks"))?
            .expect("No table \"chunks\" found. The database should have been initialized");

        // Calculate key
        let key = hash((chunk.dimension.as_ref().unwrap(), chunk.x_pos, chunk.z_pos));

        let chunk = chunk.clone();
        let chunk = Handle::current().block_on(async {
            ZstdCodec::compress_data(chunk)
                .await
                .expect("Failed to compress chunk")
        });

        // Insert chunk
        let res = database.put(&mut rw_tx, &key, chunk.as_slice());
        rw_tx.commit()?;

        res
    }

    /// Insert multiple chunks into database
    /// TODO: Find better name/disambiguation
    fn insert_chunks_into_database(
        db: &Env,
        chunks: &[SerializedChunk],
    ) -> Result<(), heed::Error> {
        // Initialize write transaction and open chunks table
        let mut rw_tx = db.write_txn()?;
        let database = db
            .open_database::<U64<LE>, Bytes>(&rw_tx, Some("chunks"))?
            .expect("No table \"chunks\" found. The database should have been initialized");

        // Update page
        for chunk in chunks {
            // Calculate key
            // let key = hash((chunk.dimension.as_ref().unwrap(), chunk.x_pos, chunk.z_pos));

            // Insert chunk
            database.put(&mut rw_tx, &chunk.hash(), chunk.data())?;
        }
        // Commit changes
        rw_tx.commit()?;
        Ok(())
    }

    #[allow(dead_code)]
    async fn load_into_cache(&self, key: u64) -> Result<(), Error> {
        Database::load_into_cache_standalone(self.db.clone(), self.cache.clone(), key).await
    }

    async fn load_into_cache_standalone(
        db: Env,
        cache: Arc<Cache<u64, Chunk>>,
        key: u64,
    ) -> Result<(), Error> {
        // let tsk_db = db.clone();

        let db = db.clone();
        tokio::task::spawn(async move {
            // Check cache
            if cache.contains_key(&key) {
                trace!("Chunk already exists in cache: {:X}", key);
            }
            // If not in cache then search in database
            else if let Ok(chunk) = Self::get_chunk_from_database(&db, &key).await
            /*spawn_blocking_db(tsk_db, move || Self::get_chunk_from_database(&db, &key))
            .await
            .unwrap()*/
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
    /// ```ignore
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
        spawn_blocking_db(tsk_db, move || {
            Self::insert_chunk_into_database(&db, &chunk)
        })
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
    /// ```ignore
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
        let db = self.db.clone();

        let res = Self::get_chunk_from_database(&db, &key).await?;

        Ok(res)

        /* // First check cache
        if self.cache.contains_key(&key) {
            Ok(self.cache.get(&key).await)
        }
        // Attempt to get chunk from persistent database
        else if let Some(chunk) = Self::get_chunk_from_database(&db, &key).await?
            /*
            spawn_blocking_db(tsk_db, move || Self::get_chunk_from_database(&db, &key))
                .await
                .unwrap()?*/
        {
            // self.cache.insert(key, chunk.clone()).await;
            Ok(Some(chunk))
        }
        // Chunk do not exist
        else {
            Ok(None)
        }*/
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
    /// ```ignore
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
        let db = self.db.clone();

        // Check first cache
        if self.cache.contains_key(&key) {
            Ok(true)
        // Else check persistent database and load it into cache
        } else {
            /*let res = spawn_blocking_db(tsk_db, move || Self::get_chunk_from_database(&db, &key))
            .await
            .unwrap();*/
            let Some(res) = Self::get_chunk_from_database(&db, &key).await? else {
                return Ok(false);
            };

            // WARNING: The previous logic was to order the chunk to be loaded into cache whether it existed or not.
            // This has been replaced by directly loading the queried chunk into cache

            // Load chunk into cache
            self.cache.insert(key, res.clone()).await;
            Ok(true)

            /* match res {
                Ok(opt) => {
                    let exist = opt.is_some();
                    if let Some(chunk) = opt {
                        self.cache.insert(key, chunk).await;
                    }
                    Ok(exist)
                }
                Err(err) => Err(Error::LmdbError(err)),
            }*/
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
    /// ```ignore
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
        spawn_blocking_db(tsk_db, move || {
            Self::insert_chunk_into_database(&db, &chunk)
        })
        .await
        .unwrap()?;

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
    /// ```ignore
    /// use crate::world::chunkformat::Chunk;
    /// use crate::database::Database;
    /// use crate::utils::error::Error;
    ///
    /// async fn batch_insert_chunks(database: Database, chunks: Vec<Chunk>) -> Result<(), Error> {
    ///  database.batch_insert_chunks(chunks).await
    /// }
    ///
    /// ```
    pub async fn batch_insert(&self, values: Vec<SerializedChunk>) -> Result<(), Error> {
        // Clone database pointer
        let db = self.db.clone();
        let tsk_db = self.db.clone();

        // Calculate all keys
        /*      let keys = values
                    .iter()
                    .map(|v| hash((v.dimension.as_ref().unwrap_or_else(|| panic!("Invalid chunk @ ({},{})", v.x_pos, v.z_pos)), v.x_pos, v.z_pos)))
                    .collect::<Vec<u64>>();
        */
        // let keys = values.iter().map(|v| v.hash()).collect::<Vec<u64>>();

        // WARNING: The previous logic was to first insert in database and then insert in cache using load_into_cache fn.
        // This has been modified to avoid having to query database while we already have the data available.
        // First insert into cache

        // TODO: Renable cache. Currently disabled because we only get serialized bytes with the hash.
        // to save in the database
        /*for (chunk) in values.iter() {
            let cache = self.cache.clone();
            let db = self.db.clone();
            let key = chunk.hash();
            let chunk = chunk.data().clone();
            tokio::spawn(async move {
                cache.insert(key, chunk).await;
                if let Err(e) = Database::load_into_cache_standalone(db, cache, key).await {
                    warn!("Error inserting chunk into database: {:?}", e);
                }
            });
        }
        */
        // Then insert into persistent database
        spawn_blocking_db(tsk_db, move || {
            Self::insert_chunks_into_database(&db, &values)
        })
        .await
        .unwrap()?;

        Ok(())
    }
}

#[tokio::test]
#[ignore]
async fn dump_chunk() {
    use crate::utils::setup_logger;
    use nbt_lib::NBTSerialize;
    use tokio::net::TcpListener;
    setup_logger().unwrap();
    let state = crate::create_state(TcpListener::bind("0.0.0.0:0").await.unwrap())
        .await
        .unwrap();
    let chunk = state
        .database
        .get_chunk(2, 2, "overworld".to_string())
        .await
        .unwrap()
        .unwrap();
    let outfile = std::fs::File::create("chunk.nbt").unwrap();
    let mut writer = std::io::BufWriter::new(outfile);
    chunk.nbt_serialize(&mut writer).unwrap();
}
