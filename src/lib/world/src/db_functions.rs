use crate::chunk_format::Chunk;
use crate::errors::WorldError;
use crate::errors::WorldError::CorruptedChunkData;
use crate::pos::ChunkPos;
use crate::structure::FerrumcChunk;
// db_functions.rs
use crate::warn;
use crate::World;
use ferrumc_config::server_config::get_global_config;
use std::hash::Hasher;
use std::sync::Arc;
use tracing::trace;
use yazi::CompressionLevel;

impl World {
    /// Save a chunk to the storage backend
    ///
    /// This function will save a chunk to the storage backend and update the cache with the new
    /// chunk data. If the chunk already exists in the cache, it will be updated with the new data.
    pub fn save_chunk(
        &self,
        pos: ChunkPos,
        dimension: &str,
        chunk: Arc<Chunk>,
    ) -> Result<(), WorldError> {
        let ret = save_chunk_internal(self, pos, dimension, &chunk);
        self.cache.insert((pos, dimension.to_string()), chunk);
        ret
    }

    /// Load a chunk from the storage backend. If the chunk is in the cache, it will be returned
    /// from the cache instead of the storage backend. If the chunk is not in the cache, it will be
    /// loaded from the storage backend and inserted into the cache.
    pub fn load_chunk(&self, pos: ChunkPos, dimension: &str) -> Result<Arc<Chunk>, WorldError> {
        if let Some(chunk) = self.cache.get(&(pos, dimension.to_string())) {
            return Ok(chunk);
        }
        let chunk = load_chunk_internal(self, pos, dimension);
        if let Ok(ref chunk) = chunk {
            self.cache
                .insert((pos, dimension.to_string()), Arc::from(chunk.clone()));
        }
        chunk.map(Arc::new)
    }

    pub fn load_chunk_owned(&self, pos: ChunkPos, dimension: &str) -> Result<Chunk, WorldError> {
        self.load_chunk(pos, dimension).map(|c| c.as_ref().clone())
    }

    /// Check if a chunk exists in the storage backend.
    ///
    /// It will first check if the chunk is in the cache and if it is, it will return true. If the
    /// chunk is not in the cache, it will check the storage backend for the chunk, returning true
    /// if it exists and false if it does not.
    pub fn chunk_exists(&self, pos: ChunkPos, dimension: &str) -> Result<bool, WorldError> {
        if self.cache.contains_key(&(pos, dimension.to_string())) {
            return Ok(true);
        }
        chunk_exists_internal(self, pos, dimension)
    }

    /// Delete a chunk from the storage backend.
    ///
    /// This function will remove the chunk from the cache and delete it from the storage backend.
    pub fn delete_chunk(&self, pos: ChunkPos, dimension: &str) -> Result<(), WorldError> {
        self.cache.remove(&(pos, dimension.to_string()));
        delete_chunk_internal(self, pos, dimension)
    }

    /// Sync the storage backend.
    ///
    /// This function will save all chunks in the cache to the storage backend and then sync the
    /// storage backend. This should be run after inserting or updating a large number of chunks
    /// to ensure that the data is properly saved to disk.
    pub fn sync(&self) -> Result<(), WorldError> {
        for (k, v) in self.cache.iter() {
            trace!("Syncing chunk: {:?}", k.0);
            save_chunk_internal(self, k.0, &k.1, &v)?;
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
        coords: &[(ChunkPos, &str)],
    ) -> Result<Vec<Arc<Chunk>>, WorldError> {
        let mut found_chunks = Vec::new();
        let mut missing_chunks = Vec::new();
        for coord in coords {
            if let Some(chunk) = self.cache.get(&(coord.0, coord.1.to_string())) {
                found_chunks.push(chunk);
            } else {
                missing_chunks.push(*coord);
            }
        }
        let fetched = load_chunk_batch_internal(self, &missing_chunks)?;
        for (chunk, (pos, dimension)) in fetched.into_iter().zip(missing_chunks) {
            let chunk = Arc::new(chunk);
            self.cache
                .insert((pos, dimension.to_string()), chunk.clone());
            found_chunks.push(chunk);
        }
        Ok(found_chunks)
    }

    /// Pre-cache a chunk in the cache
    ///
    /// This function will load a chunk from the storage backend and insert it into the cache
    /// without returning the chunk. This is useful for preloading chunks into the cache before
    /// they are needed.
    pub fn pre_cache(&self, pos: ChunkPos, dimension: &str) -> Result<(), WorldError> {
        if self.cache.get(&(pos, dimension.to_string())).is_none() {
            let chunk = load_chunk_internal(self, pos, dimension)?;
            self.cache
                .insert((pos, dimension.to_string()), Arc::new(chunk));
        }
        Ok(())
    }

    // ========================================================================
    // FerrumcChunk (packet-optimized format) methods
    // ========================================================================

    /// Save a FerrumcChunk to the storage backend.
    ///
    /// Uses the "ferrumc_chunks" table to store packet-optimized chunks separately
    /// from the old chunk format.
    pub fn save_ferrumc_chunk(
        &self,
        pos: ChunkPos,
        dimension: &str,
        chunk: &FerrumcChunk,
    ) -> Result<(), WorldError> {
        save_ferrumc_chunk_internal(self, pos, dimension, chunk)
    }

    /// Load a FerrumcChunk from the storage backend.
    ///
    /// Returns `WorldError::ChunkNotFound` if the chunk doesn't exist.
    pub fn load_ferrumc_chunk(
        &self,
        pos: ChunkPos,
        dimension: &str,
    ) -> Result<FerrumcChunk, WorldError> {
        load_ferrumc_chunk_internal(self, pos, dimension)
    }

    /// Check if a FerrumcChunk exists in the storage backend.
    pub fn ferrumc_chunk_exists(&self, pos: ChunkPos, dimension: &str) -> Result<bool, WorldError> {
        ferrumc_chunk_exists_internal(self, pos, dimension)
    }

    /// Delete a FerrumcChunk from the storage backend.
    pub fn delete_ferrumc_chunk(&self, pos: ChunkPos, dimension: &str) -> Result<(), WorldError> {
        delete_ferrumc_chunk_internal(self, pos, dimension)
    }
}

pub(crate) fn save_chunk_internal(
    world: &World,
    pos: ChunkPos,
    dimension: &str,
    chunk: &Chunk,
) -> Result<(), WorldError> {
    if !world.storage_backend.table_exists("chunks".to_string())? {
        world.storage_backend.create_table("chunks".to_string())?;
    }
    let as_bytes = yazi::compress(
        &bitcode::encode(chunk),
        yazi::Format::Zlib,
        CompressionLevel::BestSpeed,
    )?;
    let digest = create_key(dimension, pos);
    world
        .storage_backend
        .upsert("chunks".to_string(), digest, as_bytes)?;
    Ok(())
}

pub(crate) fn load_chunk_internal(
    world: &World,
    pos: ChunkPos,
    dimension: &str,
) -> Result<Chunk, WorldError> {
    let digest = create_key(dimension, pos);
    match world.storage_backend.get("chunks".to_string(), digest)? {
        Some(compressed) => {
            let (data, checksum) = yazi::decompress(compressed.as_slice(), yazi::Format::Zlib)?;
            if get_global_config().database.verify_chunk_data {
                if let Some(expected_checksum) = checksum {
                    let real_checksum = yazi::Adler32::from_buf(data.as_slice()).finish();
                    if real_checksum != expected_checksum {
                        return Err(CorruptedChunkData(real_checksum, expected_checksum));
                    }
                } else {
                    warn!("Chunk data does not have a checksum, skipping verification.");
                }
            }
            let chunk: Chunk = bitcode::decode(&data)
                .map_err(|e| WorldError::BitcodeDecodeError(e.to_string()))?;
            Ok(chunk)
        }
        None => Err(WorldError::ChunkNotFound),
    }
}

pub(crate) fn load_chunk_batch_internal(
    world: &World,
    coords: &[(ChunkPos, &str)],
) -> Result<Vec<Chunk>, WorldError> {
    let digests = coords
        .iter()
        .map(|&(pos, dim)| create_key(dim, pos))
        .collect();
    world
        .storage_backend
        .batch_get("chunks".to_string(), digests)?
        .iter()
        .map(|chunk| match chunk {
            Some(compressed) => {
                let (data, checksum) = yazi::decompress(compressed, yazi::Format::Zlib)?;
                if get_global_config().database.verify_chunk_data {
                    if let Some(expected_checksum) = checksum {
                        let real_checksum = yazi::Adler32::from_buf(data.as_slice()).finish();
                        if real_checksum != expected_checksum {
                            return Err(CorruptedChunkData(real_checksum, expected_checksum));
                        }
                    } else {
                        warn!("Chunk data does not have a checksum, skipping verification.");
                    }
                }
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
    pos: ChunkPos,
    dimension: &str,
) -> Result<bool, WorldError> {
    if !world.storage_backend.table_exists("chunks".to_string())? {
        return Ok(false);
    }
    let digest = create_key(dimension, pos);
    Ok(world.storage_backend.exists("chunks".to_string(), digest)?)
}

pub(crate) fn delete_chunk_internal(
    world: &World,
    pos: ChunkPos,
    dimension: &str,
) -> Result<(), WorldError> {
    let digest = create_key(dimension, pos);
    world.storage_backend.delete("chunks".to_string(), digest)?;
    Ok(())
}

pub(crate) fn sync_internal(world: &World) -> Result<(), WorldError> {
    world.storage_backend.flush()?;
    Ok(())
}

fn create_key(dimension: &str, pos: ChunkPos) -> u128 {
    let mut hasher = wyhash::WyHash::with_seed(0);
    hasher.write(dimension.as_bytes());
    hasher.write_u8(0xFF);
    let dim_hash = hasher.finish();
    (dim_hash as u128) << 96 | pos.pack() as u128
}

// ============================================================================
// FerrumcChunk Internal Functions
// ============================================================================

const FERRUMC_CHUNKS_TABLE: &str = "ferrumc_chunks";

pub(crate) fn save_ferrumc_chunk_internal(
    world: &World,
    pos: ChunkPos,
    dimension: &str,
    chunk: &FerrumcChunk,
) -> Result<(), WorldError> {
    if !world
        .storage_backend
        .table_exists(FERRUMC_CHUNKS_TABLE.to_string())?
    {
        world
            .storage_backend
            .create_table(FERRUMC_CHUNKS_TABLE.to_string())?;
    }
    let as_bytes = yazi::compress(
        &bitcode::encode(chunk),
        yazi::Format::Zlib,
        CompressionLevel::BestSpeed,
    )?;
    let digest = create_key(dimension, pos);
    world
        .storage_backend
        .upsert(FERRUMC_CHUNKS_TABLE.to_string(), digest, as_bytes)?;
    Ok(())
}

pub(crate) fn load_ferrumc_chunk_internal(
    world: &World,
    pos: ChunkPos,
    dimension: &str,
) -> Result<FerrumcChunk, WorldError> {
    // Check if table exists first - if not, chunk definitely doesn't exist
    if !world
        .storage_backend
        .table_exists(FERRUMC_CHUNKS_TABLE.to_string())?
    {
        return Err(WorldError::ChunkNotFound);
    }
    let digest = create_key(dimension, pos);
    match world
        .storage_backend
        .get(FERRUMC_CHUNKS_TABLE.to_string(), digest)?
    {
        Some(compressed) => {
            let (data, checksum) = yazi::decompress(compressed.as_slice(), yazi::Format::Zlib)?;
            if get_global_config().database.verify_chunk_data {
                if let Some(expected_checksum) = checksum {
                    let real_checksum = yazi::Adler32::from_buf(data.as_slice()).finish();
                    if real_checksum != expected_checksum {
                        return Err(CorruptedChunkData(real_checksum, expected_checksum));
                    }
                } else {
                    warn!("Chunk data does not have a checksum, skipping verification.");
                }
            }
            let chunk: FerrumcChunk = bitcode::decode(&data)
                .map_err(|e| WorldError::BitcodeDecodeError(e.to_string()))?;
            Ok(chunk)
        }
        None => Err(WorldError::ChunkNotFound),
    }
}

pub(crate) fn ferrumc_chunk_exists_internal(
    world: &World,
    pos: ChunkPos,
    dimension: &str,
) -> Result<bool, WorldError> {
    if !world
        .storage_backend
        .table_exists(FERRUMC_CHUNKS_TABLE.to_string())?
    {
        return Ok(false);
    }
    let digest = create_key(dimension, pos);
    Ok(world
        .storage_backend
        .exists(FERRUMC_CHUNKS_TABLE.to_string(), digest)?)
}

pub(crate) fn delete_ferrumc_chunk_internal(
    world: &World,
    pos: ChunkPos,
    dimension: &str,
) -> Result<(), WorldError> {
    let digest = create_key(dimension, pos);
    world
        .storage_backend
        .delete(FERRUMC_CHUNKS_TABLE.to_string(), digest)?;
    Ok(())
}
