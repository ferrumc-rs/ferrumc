use redb::TableDefinition;
use tracing::{debug, trace, warn};

use crate::database::Database;
use crate::utils::binary_utils::{bzip_compress, bzip_decompress, human_readable_size};
use crate::utils::error::Error;
use crate::utils::hash::hash;
use crate::world::chunkformat::Chunk;

impl Database {
    async fn load_into_cache(&self, x: i32, z: i32, dimension: String) -> Result<(), Error> {
        let key = hash((x, z));
        if self.cache.contains_key(&key) {
            debug!("Chunk already in cache: {}, {}", x, z);
            return Ok(());
        } else {
            let c = self.internal_get_chunk(x, z, dimension).await?;
            if c.is_some() {
                self.cache.insert(key, c.unwrap()).await;
            } else {
                warn!(
                    "Chunk does not exist in db, can't load into cache: {}, {}",
                    x, z
                );
            }
        }
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
        let x = value.x_pos;
        let z = value.z_pos;
        tokio::task::spawn_blocking(move || {
            let key = hash((value.x_pos, value.z_pos));
            let encoded = bincode::encode_to_vec(&value, bincode::config::standard())
                .expect("Failed to encode");
            let compressed = bzip_compress(&encoded).expect("Failed to compress");
            trace!(
                "Inserting chunk: {}, {} | Uncompressed: {} | Compressed: {}",
                x,
                z,
                human_readable_size(encoded.len() as u64),
                human_readable_size(compressed.len() as u64)
            );
            let tx = db.begin_write().unwrap();
            let tablename = format!("chunks/{}", value.dimension.unwrap());
            let res = {
                let table: TableDefinition<u64, Vec<u8>> = TableDefinition::new(tablename.as_str());
                let mut transaction = tx.open_table(table).unwrap();
                let res: Result<(), Error> = match transaction.insert(key, compressed) {
                    Ok(val) => match val {
                        Some(_) => {
                            warn!("Chunk already exists at {}, {}", x, z);
                            Err(Error::ChunkExists(x, z))
                        }
                        None => Ok(()),
                    },
                    Err(e) => Err(Error::Generic(format!("Failed to insert chunk: {}", e))),
                };
                res
            };
            return if res.is_ok() {
                tx.commit().unwrap();
                Ok(())
            } else {
                tx.abort().unwrap();
                res
            };
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
        dimension: impl Into<String>,
    ) -> Result<Option<Chunk>, Error> {
        let key = hash((x, z));
        if self.cache.contains_key(&key) {
            Ok(self.cache.get(&key).await)
        } else {
            if let Ok(chunk) = self.internal_get_chunk(x, z, dimension).await {
                if chunk.is_some() {
                    self.cache.insert(key, chunk.clone().unwrap()).await;
                }
                Ok(chunk)
            } else {
                Ok(None)
            }
        }
    }

    async fn internal_get_chunk(
        &self,
        x: i32,
        z: i32,
        dimension: impl Into<String>,
    ) -> Result<Option<Chunk>, Error> {
        let dimension = dimension.into();
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let key = hash((x, z));
            trace!("Getting chunk: {}, {}", x, z);
            let tablename = format!("chunks/{}", dimension);
            let tx = db.begin_read().unwrap();
            {
                let table: TableDefinition<u64, Vec<u8>> = TableDefinition::new(tablename.as_str());
                let transaction = tx.open_table(table).unwrap();
                match transaction.get(key) {
                    Ok(chunk) => match chunk {
                        Some(chunk) => {
                            let chunk = bzip_decompress(chunk.value().as_ref())
                                .expect("Failed to decompress");
                            let (chunk, len) = bincode::decode_from_slice(
                                chunk.as_slice(),
                                bincode::config::standard(),
                            )
                            .expect(
                                "Could not decode chunk from database. Has the format changed?",
                            );
                            trace!(
                                "Got chunk: {} {}, {} long",
                                x,
                                z,
                                human_readable_size(len as u64)
                            );
                            Some(chunk)
                        }
                        None => {
                            debug!("Could not find chunk {}, {}", x, z);
                            None
                        }
                    },
                    Err(e) => {
                        warn!("Failed to get chunk: {}", e);
                        None
                    }
                }
            }
        })
        .await
        .expect("Failed to join tasks");
        Ok(result)
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
        let key = hash((x, z));
        if self.cache.contains_key(&key) {
            Ok(true)
        } else {
            let res = self.internal_chunk_exists(x, z, dimension.clone()).await;
            self.load_into_cache(x, z, dimension).await?;
            res
        }
    }

    async fn internal_chunk_exists(
        &self,
        x: i32,
        z: i32,
        dimension: String,
    ) -> Result<bool, Error> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let record_name = hash((x, z));
            let tablename = format!("chunks/{}", dimension);
            let tx = db.begin_read().unwrap();
            let table: TableDefinition<u64, Vec<u8>> = TableDefinition::new(tablename.as_str());
            let transaction = tx.open_table(table).unwrap();
            transaction.get(record_name).is_ok()
        })
        .await
        .expect("Failed to join tasks");
        Ok(result)
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
            let (x, z) = (value.x_pos, value.z_pos);
            let record_name = hash((x, z));
            let encoded = bincode::encode_to_vec(&value, bincode::config::standard())
                .expect("Failed to encode");
            let compressed = bzip_compress(&encoded).expect("Failed to compress");
            let dim = value.dimension.unwrap();
            let tablename = format!("chunks/{}", dim);
            let tx = db.begin_write().unwrap();
            let res = {
                let table: TableDefinition<u64, Vec<u8>> = TableDefinition::new(tablename.as_str());
                let mut transaction = tx.open_table(table).unwrap();
                let res = match transaction.insert(record_name, compressed) {
                    Ok(val) => {
                        if val.is_none() {
                            Err(Error::ChunkNotFound(x, z))
                        } else {
                            Ok(())
                        }
                    }
                    Err(e) => Err(Error::Generic(format!("Failed to insert chunk: {}", e))),
                };
                res
            };
            if res.is_ok() {
                tx.commit().unwrap();
                Ok(())
            } else {
                tx.abort().unwrap();
                res
            }
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
        let result = self.internal_batch_insert(values.clone()).await;
        if result.is_ok() {
            for value in values {
                let key = hash((value.x_pos, value.z_pos));
                self.cache.insert(key, value).await;
            }
            Ok(())
        } else {
            result
        }
    }

    async fn internal_batch_insert(&self, values: Vec<Chunk>) -> Result<(), Error> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let tx = db.begin_write().unwrap();
            for value in values {
                let x = value.x_pos;
                let z = value.z_pos;
                let key = hash((value.x_pos, value.z_pos));
                let encoded = bincode::encode_to_vec(&value, bincode::config::standard())
                    .expect("Failed to encode");
                let compressed = bzip_compress(&encoded).expect("Failed to compress");
                trace!(
                    "Inserting chunk: {}, {} | Uncompressed: {} | Compressed: {}",
                    x,
                    z,
                    human_readable_size(encoded.len() as u64),
                    human_readable_size(compressed.len() as u64)
                );
                let tablename = format!("chunks/{}", value.dimension.unwrap());
                let res = {
                    let table: TableDefinition<u64, Vec<u8>> =
                        TableDefinition::new(tablename.as_str());
                    let mut transaction = tx.open_table(table).unwrap();
                    let res = match transaction.insert(key, compressed) {
                        Ok(val) => match val {
                            Some(_) => {
                                warn!("Chunk already exists at {}, {}", x, z);
                                Err(Error::ChunkExists(x, z))
                            }
                            None => Ok(()),
                        },
                        Err(e) => Err(Error::Generic(format!("Failed to insert chunk: {}", e))),
                    };
                    res
                };
                if res.is_err() {
                    tx.abort().unwrap();
                    return res;
                };
            }
            tx.commit().unwrap();
            Ok(())
        })
        .await;
        result.expect("Failed to join tasks")?;
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
