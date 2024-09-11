use tracing::{debug, warn};

use crate::database::Database;
use crate::utils::error::Error;
use crate::utils::hash::hash;
use crate::world::chunkformat::Chunk;

use bincode::config::standard;
use bincode::{decode_from_slice, encode_to_vec};

impl Database {
    async fn load_into_cache(&self, key: u64) -> Result<(), Error> {
        let db = self.db.clone();
        let cache = self.cache.clone();

        tokio::task::spawn(async move {
            // This is stupid, but it's the only way to get the lifetime checker to shut up
            let get_chunk = |db: &rocksdb::DB, key: u64| {
                let cf = db
                    .cf_handle("chunks")
                    .expect("Failed to get column family \"chunks\"");
                if let Ok(data) = db.get_cf(&cf, key.to_be_bytes()) {
                    if let Some(encoded) = data {
                        let chunk: (Chunk, usize) = decode_from_slice(&encoded, standard())
                            .expect("Failed to decode chunk from database");
                        Ok(Some(chunk.0))
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(Error::DatabaseError("Failed to get chunk".to_string()))
                }
            };

            if cache.contains_key(&key) {
                debug!("Chunk already exists in cache: {:X}", key);
            } else {
                if let Ok(c) = get_chunk(&db, key) {
                    if c.is_some() {
                        cache.insert(key, c.unwrap()).await;
                    } else {
                        warn!(
                            "Chunk does not exist in db, can't load into cache: {:X}",
                            key,
                        );
                    }
                } else {
                    warn!("Error getting chunk: {:X}", key,);
                }
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
        let key = hash((value.x_pos, value.z_pos));
        let result = self.internal_insert_chunk(value.clone()).await;
        if result.is_ok() {
            self.cache.insert(key, value).await;
            Ok(())
        } else {
            result
        }
    }
    async fn internal_insert_chunk(&self, value: Chunk) -> Result<(), Error> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let cf = db
                .cf_handle("chunks")
                .expect("Failed to get column family \"chunks\"");
            let encoded = encode_to_vec(&value, standard()).expect("Failed to encode chunk");
            let key = hash((value.dimension.unwrap(), value.x_pos, value.z_pos));
            db.put_cf(&cf, key.to_be_bytes(), encoded)
                .or(Err(Error::DatabaseError(
                    "Failed to insert chunk".to_string(),
                )))
        })
        .await?
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
        let key = hash((dimension, x, z));
        if self.cache.contains_key(&key) {
            Ok(self.cache.get(&key).await)
        } else {
            if let Ok(chunk) = self.internal_get_chunk(key).await {
                if chunk.is_some() {
                    self.cache.insert(key, chunk.clone().unwrap()).await;
                }
                Ok(chunk)
            } else {
                Ok(None)
            }
        }
    }

    async fn internal_get_chunk(&self, key: u64) -> Result<Option<Chunk>, Error> {
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            let cf = db
                .cf_handle("chunks")
                .expect("Failed to get column family \"chunks\"");
            if let Ok(data) = db.get_cf(&cf, key.to_be_bytes()) {
                if let Some(encoded) = data {
                    let chunk = decode_from_slice(&encoded, standard())
                        .expect("Failed to decode chunk from database");
                    Ok(Some(chunk.0))
                } else {
                    Ok(None)
                }
            } else {
                Err(Error::DatabaseError("Failed to get chunk".to_string()))
            }
        })
        .await?
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
        let key = hash((dimension, x, z));
        if self.cache.contains_key(&key) {
            Ok(true)
        } else {
            let res = self.internal_chunk_exists(key).await;
            self.load_into_cache(key).await?;
            res
        }
    }

    async fn internal_chunk_exists(&self, key: u64) -> Result<bool, Error> {
        let cf = self
            .db
            .cf_handle("chunks")
            .expect("Failed to get column family \"chunks\"");
        Ok(self.db.get_cf(&cf, key.to_be_bytes()).is_ok())
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
        let key = hash((value.x_pos, value.z_pos));
        let result = self.internal_update_chunk(value.clone()).await;
        if result.is_ok() {
            self.cache.insert(key, value).await;
            Ok(())
        } else {
            result
        }
    }

    async fn internal_update_chunk(&self, value: Chunk) -> Result<(), Error> {
        let db = self.db.clone();

        tokio::task::spawn_blocking(move || {
            let cf = db
                .cf_handle("chunks")
                .expect("Failed to get column family \"chunks\"");
            let encoded = encode_to_vec(&value, standard()).expect("Failed to encode chunk");
            let key = hash((value.dimension.unwrap(), value.x_pos, value.z_pos));
            db.put_cf(&cf, key.to_be_bytes(), encoded)
                .or(Err(Error::DatabaseError(
                    "Failed to update chunk".to_string(),
                )))
        })
        .await?
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
        // TODO: Ewwwww clones, disgusting, fix this
        let keys = values
            .iter()
            .map(|v| hash((v.dimension.as_ref().unwrap(), v.x_pos, v.z_pos)))
            .collect::<Vec<u64>>();
        let result = self.internal_batch_insert(values).await;
        if result.is_ok() {
            for key in keys {
                self.load_into_cache(key).await?;
            }
            Ok(())
        } else {
            result
        }
    }

    async fn internal_batch_insert(&self, values: Vec<Chunk>) -> Result<(), Error> {
        let db = self.db.clone();

        tokio::task::spawn_blocking(move || {
            let cf = db
                .cf_handle("chunks")
                .expect("Failed to get column family \"chunks\"");
            let mut batch = rocksdb::WriteBatch::default();
            for value in values {
                let encoded = encode_to_vec(&value, standard()).expect("Failed to encode chunk");
                let key = hash((value.dimension.unwrap(), value.x_pos, value.z_pos));
                batch.put_cf(&cf, key.to_be_bytes(), encoded);
            }
            db.write(batch).or(Err(Error::DatabaseError(
                "Failed to batch insert chunks".to_string(),
            )))
        })
        .await?
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
