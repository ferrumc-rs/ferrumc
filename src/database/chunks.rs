use redb::TableDefinition;
use tracing::{debug, trace, warn};

use crate::database::Database;
use crate::utils::binary_utils::{bzip_compress, bzip_decompress, human_readable_size};
use crate::utils::error::Error;
use crate::utils::hash::hash;
use crate::world::chunkformat::Chunk;

impl Database {
    pub async fn insert_chunk(&self, value: Chunk, dimension: String) -> Result<bool, Error> {
        let db = self.db.clone();
        let x = value.x_pos;
        let z = value.z_pos;
        let result = tokio::task::spawn_blocking(move || {
            let key = hash((value.x_pos, value.z_pos));
            let encoded = bincode::encode_to_vec(value, bincode::config::standard())
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
            let mut did_exist = false;
            let tablename = format!("chunks/{}", dimension);
            {
                let table: TableDefinition<u64, Vec<u8>> = TableDefinition::new(tablename.as_str());
                let mut transaction = tx.open_table(table).unwrap();
                match transaction.insert(key, compressed) {
                    Ok(val) => {
                        if val.is_some() {
                            did_exist = true;
                        }
                    }
                    Err(e) => {
                        warn!("Failed to insert chunk: {}", e);
                        did_exist = true;
                    }
                };
            }
            tx.commit().unwrap();
            did_exist
        })
        .await
        .expect("Failed to join tasks");
        Ok(result)
    }

    pub async fn get_chunk(
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

    pub async fn chunk_exists(&self, x: i32, z: i32, dimension: String) -> Result<bool, Error> {
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

    pub async fn update_chunk(&self, value: Chunk, dimension: String) -> Result<bool, Error> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let (x, z) = (value.x_pos, value.z_pos);
            let record_name = hash((x, z));
            let encoded = bincode::encode_to_vec(value, bincode::config::standard())
                .expect("Failed to encode");
            let compressed = bzip_compress(&encoded).expect("Failed to compress");
            let tablename = format!("chunks/{}", dimension);
            let tx = db.begin_write().unwrap();
            let mut did_exist = false;
            {
                let table: TableDefinition<u64, Vec<u8>> = TableDefinition::new(tablename.as_str());
                let mut transaction = tx.open_table(table).unwrap();
                match transaction.insert(record_name, compressed) {
                    Ok(val) => {
                        if val.is_some() {
                            did_exist = true;
                        }
                    }
                    Err(e) => {
                        warn!("Failed to update chunk: {}", e);
                        did_exist = true;
                    }
                };
            }
            tx.commit().unwrap();
            did_exist
        })
        .await;
        Ok(result.expect("Failed to join tasks"))
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
