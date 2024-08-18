use tracing::{debug, warn};

use crate::database::Database;
use crate::utils::error::Error;
use crate::world::chunkformat::Chunk;

impl Database {
    pub async fn insert_chunk(&self, value: Chunk, dimension: String) -> Result<bool, Error> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let record_name = format!("{},{}", value.x_pos, value.z_pos);
            let encoded = bincode::encode_to_vec(value, bincode::config::standard())
                .expect("Failed to encode");
            db.open_tree(format!("chunks/{}", dimension))?
                .insert(record_name, encoded)
        })
        .await
        .expect("Failed to join tasks")
        .expect("Failed to insert chunk");
        match result {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn get_chunk(
        &self,
        x: i32,
        z: i32,
        dimension: String,
    ) -> Result<Option<Chunk>, Error> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let record_name = format!("{},{}", x, z);
            debug!("Getting chunk: {}", record_name);
            let chunk = db
                .open_tree(format!("chunks/{}", dimension))
                .unwrap()
                .get(&record_name)
                .unwrap();
            match chunk {
                Some(chunk) => {
                    let chunk = chunk.as_ref();
                    let (chunk, len) =
                        bincode::decode_from_slice(chunk, bincode::config::standard()).unwrap();
                    debug!("Got chunk: {}, {} bytes long", record_name, len);
                    Some(chunk)
                }
                None => {
                    debug!("Could not find chunk {}", record_name);
                    None
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
            let record_name = format!("{},{}", x, z);
            db.open_tree(format!("chunks/{}", dimension))
                .unwrap()
                .contains_key(record_name)
        })
        .await
        .expect("Failed to join tasks")
        .expect("Failed to check if chunk exists");
        Ok(result)
    }

    pub async fn update_chunk(&self, value: Chunk, dimension: String) -> Result<bool, Error> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let record_name = format!("{},{}", value.x_pos, value.z_pos);
            let encoded = bincode::encode_to_vec(value, bincode::config::standard())
                .expect("Failed to encode");
            if db
                .open_tree("chunks")
                .unwrap()
                .remove(&record_name)
                .unwrap()
                .is_none()
            {
                warn!("Attempted to update non-existent chunk: {}", record_name);
            }
            db.open_tree(format!("chunks/{}", dimension))
                .unwrap()
                .insert(record_name, encoded)
        })
        .await
        .expect("Failed to join tasks")
        .expect("Failed to update chunk");
        match result {
            Some(_) => Ok(true),
            None => Ok(false),
        }
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
        .get_chunk(-34, 1, "overworld".to_string())
        .await
        .unwrap()
        .unwrap();
    let outfile = std::fs::File::create("chunk.dump").unwrap();
    let mut writer = std::io::BufWriter::new(outfile);
    serde_json::to_writer(&mut writer, &chunk).unwrap();
}
