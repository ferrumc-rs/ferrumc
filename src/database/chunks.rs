use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::database::Database;
use crate::utils::error::Error;
use crate::world::chunkformat::Chunk;

impl Database {
    pub async fn insert_chunk(&self, value: Chunk, dimension: String) -> Result<bool, Error> {
        let db = self.db.clone();
        let result = tokio::task::spawn_blocking(move || {
            let record_name = format!("{},{}", value.x_pos, value.z_pos);
            let mut ser = flexbuffers::FlexbufferSerializer::new();
            value.serialize(&mut ser).unwrap();
            let encoded = ser.take_buffer();
            db.open_tree(format!("chunks/{}", dimension))
                .unwrap()
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
            let chunk = db
                .open_tree(format!("chunks/{}", dimension))
                .unwrap()
                .get(record_name)
                .unwrap();
            match chunk {
                Some(chunk) => {
                    let chunk = chunk.as_ref();
                    let deserializer = flexbuffers::Reader::get_root(chunk).unwrap();
                    let chunk: Chunk = Chunk::deserialize(deserializer).unwrap();
                    Some(chunk)
                }
                None => None,
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
            let mut ser = flexbuffers::FlexbufferSerializer::new();
            value.serialize(&mut ser).unwrap();
            let encoded = ser.take_buffer();
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
