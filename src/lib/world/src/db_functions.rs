// db_functions.rs
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
    pub fn save_chunk(&self, chunk: Chunk) -> Result<(), WorldError> {
        let ret = save_chunk_internal(self, &chunk);
        self.cache
            .insert((chunk.x, chunk.z, chunk.dimension.clone()), chunk);
        ret
    }

    /// Load a chunk from the storage backend. If the chunk is in the cache, it will be returned
    /// from the cache instead of the storage backend. If the chunk is not in the cache, it will be
    /// loaded from the storage backend and inserted into the cache.
    pub fn load_chunk(&self, x: i32, z: i32, dimension: &str) -> Result<Chunk, WorldError> {
        if let Some(chunk) = self.cache.get(&(x, z, dimension.to_string())) {
            return Ok(chunk);
        }
        let chunk = load_chunk_internal(self, &self.compressor, x, z, dimension);
        if let Ok(ref chunk) = chunk {
            self.cache
                .insert((x, z, dimension.to_string()), chunk.clone());
        }
        chunk
    }

    /// Check if a chunk exists in the storage backend.
    ///
    /// It will first check if the chunk is in the cache and if it is, it will return true. If the
    /// chunk is not in the cache, it will check the storage backend for the chunk, returning true
    /// if it exists and false if it does not.
    pub fn chunk_exists(&self, x: i32, z: i32, dimension: &str) -> Result<bool, WorldError> {
        if self.cache.contains_key(&(x, z, dimension.to_string())) {
            return Ok(true);
        }
        chunk_exists_internal(self, x, z, dimension)
    }

    /// Delete a chunk from the storage backend.
    ///
    /// This function will remove the chunk from the cache and delete it from the storage backend.
    pub fn delete_chunk(&self, x: i32, z: i32, dimension: &str) -> Result<(), WorldError> {
        self.cache.remove(&(x, z, dimension.to_string()));
        delete_chunk_internal(self, x, z, dimension)
    }

    /// Sync the storage backend.
    ///
    /// This function will save all chunks in the cache to the storage backend and then sync the
    /// storage backend. This should be run after inserting or updating a large number of chunks
    /// to ensure that the data is properly saved to disk.
    pub fn sync(&self) -> Result<(), WorldError> {
        for (k, v) in self.cache.iter() {
            trace!("Syncing chunk: {:?}", (k.0, k.1));
            save_chunk_internal(self, &v)?;
        }
        sync_internal(self)
    }

    /// Load a batch of chunks from the storage backend.
    ///
    /// This function attempts to load as many chunks as it can find from the cache first, then fetches
    /// the missing chunks from the storage backend. The chunks are then inserted into the cache and
    /// returned as a vector.
    pub fn load_chunk_batch(
        &self,
        coords: Vec<(i32, i32, &str)>,
    ) -> Result<Vec<Chunk>, WorldError> {
        let mut found_chunks = Vec::new();
        let mut missing_chunks = Vec::new();
        for coord in coords.iter() {
            if let Some(chunk) = self.cache.get(&(coord.0, coord.1, coord.2.to_string())) {
                found_chunks.push(chunk);
            } else {
                missing_chunks.push(*coord);
            }
        }
        let fetched = load_chunk_batch_internal(self, missing_chunks)?;
        for chunk in fetched {
            self.cache
                .insert((chunk.x, chunk.z, chunk.dimension.clone()), chunk.clone());
            found_chunks.push(chunk);
        }
        Ok(found_chunks)
    }

    /// Pre-cache a chunk in the cache
    ///
    /// This function will load a chunk from the storage backend and insert it into the cache
    /// without returning the chunk. This is useful for preloading chunks into the cache before
    /// they are needed.
    pub fn pre_cache(&self, x: i32, z: i32, dimension: &str) -> Result<(), WorldError> {
        if self.cache.get(&(x, z, dimension.to_string())).is_none() {
            let chunk = load_chunk_internal(self, &self.compressor, x, z, dimension)?;
            self.cache.insert((x, z, dimension.to_string()), chunk);
        }
        Ok(())
    }
}

pub(crate) fn save_chunk_internal(world: &World, chunk: &Chunk) -> Result<(), WorldError> {
    if !world.storage_backend.table_exists("chunks".to_string())? {
        world.storage_backend.create_table("chunks".to_string())?;
    }
    let as_bytes = world.compressor.compress(&bitcode::encode(chunk))?;
    let digest = create_key(chunk.dimension.as_str(), chunk.x, chunk.z);
    world
        .storage_backend
        .upsert("chunks".to_string(), digest, as_bytes)?;
    Ok(())
}

pub(crate) fn save_chunk_internal_batch(world: &World, chunks: &[Chunk]) -> Result<(), WorldError> {
    // Prepare the batch data for the upsert
    let mut batch_data = Vec::new();

    for chunk in chunks.iter() {
        // Compress the chunk and encode it
        let as_bytes = world.compressor.compress(&bitcode::encode(chunk))?;
        // Create the key for the chunk
        let digest = create_key(chunk.dimension.as_str(), chunk.x, chunk.z);
        // Collect the key-value pair into the batch data
        batch_data.push((digest, as_bytes));
    }

    // Perform the batch upsert
    world
        .storage_backend
        .batch_upsert("chunks".to_string(), batch_data)?;

    Ok(())
}

pub(crate) fn load_chunk_internal(
    world: &World,
    compressor: &Compressor,
    x: i32,
    z: i32,
    dimension: &str,
) -> Result<Chunk, WorldError> {
    let digest = create_key(dimension, x, z);
    match world.storage_backend.get("chunks".to_string(), digest)? {
        Some(compressed) => {
            let data = compressor.decompress(&compressed)?;
            let chunk: Chunk = bitcode::decode(&data)
                .map_err(|e| WorldError::BitcodeDecodeError(e.to_string()))?;
            Ok(chunk)
        }
        None => Err(WorldError::ChunkNotFound),
    }
}

pub(crate) fn load_chunk_batch_internal(
    world: &World,
    coords: Vec<(i32, i32, &str)>,
) -> Result<Vec<Chunk>, WorldError> {
    let digests = coords
        .into_iter()
        .map(|(x, z, dim)| create_key(dim, x, z))
        .collect();
    world
        .storage_backend
        .batch_get("chunks".to_string(), digests)?
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

pub(crate) fn chunk_exists_internal(
    world: &World,
    x: i32,
    z: i32,
    dimension: &str,
) -> Result<bool, WorldError> {
    if !world.storage_backend.table_exists("chunks".to_string())? {
        return Ok(false);
    }
    let digest = create_key(dimension, x, z);
    Ok(world.storage_backend.exists("chunks".to_string(), digest)?)
}

pub(crate) fn delete_chunk_internal(
    world: &World,
    x: i32,
    z: i32,
    dimension: &str,
) -> Result<(), WorldError> {
    let digest = create_key(dimension, x, z);
    world.storage_backend.delete("chunks".to_string(), digest)?;
    Ok(())
}

pub(crate) fn sync_internal(world: &World) -> Result<(), WorldError> {
    world.storage_backend.flush()?;
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
