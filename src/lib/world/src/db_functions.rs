use crate::chunk_format::Chunk;
use crate::errors::WorldError;
use crate::World;
use ferrumc_storage::compressors::Compressor;
use std::hash::Hasher;
use tracing::trace;

impl World {
    /// Save a chunk to the storage backend
    ///
    /// This function will save a chunk to the storage backend and update the cache with the new
    /// chunk data. If the chunk already exists in the cache, it will be updated with the new data.
    pub async fn save_chunk(&self, chunk: Chunk) -> Result<(), WorldError> {
        self.cache
            .insert((chunk.x, chunk.z, chunk.dimension.clone()), chunk.clone())
            .await;
        save_chunk_internal(self, chunk).await
    }

    pub async fn load_chunk(&self, x: i32, z: i32, dimension: &str) -> Result<Chunk, WorldError> {
        if let Some(chunk) = self.cache.get(&(x, z, dimension.to_string())).await {
            return Ok(chunk);
        }
        let chunk = load_chunk_internal(self, &self.compressor, x, z, dimension).await;
        if let Ok(ref chunk) = chunk {
            self.cache
                .insert((x, z, dimension.to_string()), chunk.clone())
                .await;
        }
        chunk
    }

    pub async fn chunk_exists(&self, x: i32, z: i32, dimension: &str) -> Result<bool, WorldError> {
        if self.cache.contains_key(&(x, z, dimension.to_string())) {
            return Ok(true);
        }
        chunk_exists_internal(self, x, z, dimension).await
    }

    pub async fn delete_chunk(&self, x: i32, z: i32, dimension: &str) -> Result<(), WorldError> {
        self.cache.remove(&(x, z, dimension.to_string())).await;
        delete_chunk_internal(self, x, z, dimension).await
    }

    pub async fn sync(&self) -> Result<(), WorldError> {
        for (k, v) in self.cache.iter() {
            trace!("Syncing chunk: {:?}", (k.0, k.1));
            save_chunk_internal(self, v.clone()).await?;
        }
        sync_internal(self).await
    }

    pub async fn load_chunk_batch(
        &self,
        coords: Vec<(i32, i32, &str)>,
    ) -> Result<Vec<Chunk>, WorldError> {
        let mut found_chunks = Vec::new();
        let mut missing_chunks = Vec::new();
        for coord in coords.iter() {
            if let Some(chunk) = self
                .cache
                .get(&(coord.0, coord.1, coord.2.to_string()))
                .await
            {
                found_chunks.push(chunk);
            } else {
                missing_chunks.push(*coord);
            }
        }
        let fetched = load_chunk_batch_internal(self, missing_chunks).await?;
        for chunk in fetched {
            self.cache
                .insert((chunk.x, chunk.z, chunk.dimension.clone()), chunk.clone())
                .await;
            found_chunks.push(chunk);
        }
        Ok(found_chunks)
    }

    /// Pre-cache a chunk in the cache
    ///
    /// This function will load a chunk from the storage backend and insert it into the cache
    /// without returning the chunk. This is useful for preloading chunks into the cache before
    /// they are needed.
    pub async fn pre_cache(&self, x: i32, z: i32, dimension: &str) -> Result<(), WorldError> {
        if self
            .cache
            .get(&(x, z, dimension.to_string()))
            .await
            .is_none()
        {
            let chunk = load_chunk_internal(self, &self.compressor, x, z, dimension).await?;
            self.cache
                .insert((x, z, dimension.to_string()), chunk)
                .await;
        }
        Ok(())
    }
}

pub(crate) async fn save_chunk_internal(world: &World, chunk: Chunk) -> Result<(), WorldError> {
    let as_bytes = world.compressor.compress(&bitcode::encode(&chunk))?;
    let digest = create_key(chunk.dimension.as_str(), chunk.x, chunk.z);
    world
        .storage_backend
        .upsert("chunks".to_string(), digest, as_bytes)
        .await?;
    Ok(())
}

pub(crate) async fn load_chunk_internal(
    world: &World,
    compressor: &Compressor,
    x: i32,
    z: i32,
    dimension: &str,
) -> Result<Chunk, WorldError> {
    let digest = create_key(dimension, x, z);
    match world
        .storage_backend
        .get("chunks".to_string(), digest)
        .await?
    {
        Some(compressed) => {
            let data = compressor.decompress(&compressed)?;
            let chunk: Chunk = bitcode::decode(&data)
                .map_err(|e| WorldError::BitcodeDecodeError(e.to_string()))?;
            Ok(chunk)
        }
        None => Err(WorldError::ChunkNotFound),
    }
}

pub(crate) async fn load_chunk_batch_internal(
    world: &World,
    coords: Vec<(i32, i32, &str)>,
) -> Result<Vec<Chunk>, WorldError> {
    let digests = coords
        .into_iter()
        .map(|(x, z, dim)| create_key(dim, x, z))
        .collect();
    world
        .storage_backend
        .batch_get("chunks".to_string(), digests)
        .await?
        .iter()
        .map(|chunk| match chunk {
            Some(compressed) => {
                let data = world.compressor.decompress(compressed)?;
                let chunk: Chunk = bitcode::decode(&data)
                    .map_err(|e| WorldError::BitcodeDecodeError(e.to_string()))?;
                Ok(chunk)
            }
            None => Err(WorldError::ChunkNotFound),
        })
        .collect()
}

pub(crate) async fn chunk_exists_internal(
    world: &World,
    x: i32,
    z: i32,
    dimension: &str,
) -> Result<bool, WorldError> {
    let digest = create_key(dimension, x, z);
    Ok(world
        .storage_backend
        .exists("chunks".to_string(), digest)
        .await?)
}

pub(crate) async fn delete_chunk_internal(
    world: &World,
    x: i32,
    z: i32,
    dimension: &str,
) -> Result<(), WorldError> {
    let digest = create_key(dimension, x, z);
    world
        .storage_backend
        .delete("chunks".to_string(), digest)
        .await?;
    Ok(())
}

pub(crate) async fn sync_internal(world: &World) -> Result<(), WorldError> {
    world.storage_backend.flush().await?;
    Ok(())
}

fn create_key(dimension: &str, x: i32, z: i32) -> u128 {
    let mut key = 0u128;
    let mut hasher = wyhash::WyHash::with_seed(0);
    hasher.write_str(dimension);
    let dim_hash = hasher.finish();
    // Insert the dimension hash into the key as the first 32 bits
    key |= (dim_hash as u128) << 96;
    // Convert the x coordinate to a 48 bit integer and insert it into the key
    key |= ((x as u128) & 0x0000_0000_FFFF_FFFF) << 48;
    // Convert the z coordinate to a 48 bit integer and insert it into the key
    key |= (z as u128) & 0x0000_0000_FFFF_FFFF;

    key
}
