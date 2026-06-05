use crate::chunk::Chunk;
use crate::errors::WorldError;
use crate::errors::WorldError::CorruptedChunkData;
use crate::pos::ChunkPos;
// db_functions.rs
use crate::World;
use crate::{warn, MutChunk, RefChunk};
use ferrumc_config::server_config::get_global_config;
use std::hash::Hasher;
use tracing::trace;
use yazi::CompressionLevel;

impl World {
    /// Save a chunk to the storage backend
    ///
    /// This function will save a chunk to the storage backend and update the cache with the new
    /// chunk data. If the chunk already exists in the cache, it will be updated with the new data.
    pub fn insert_chunk(
        &self,
        pos: ChunkPos,
        dimension: &str,
        chunk: Chunk,
    ) -> Result<(), WorldError> {
        let mut chunk = chunk;
        chunk.sections.iter_mut().for_each(|c| c.dirty = false);
        save_chunk_internal(self, pos, dimension, &chunk)?;
        // self.cache.insert((pos, dimension.to_string()), chunk);
        Ok(())
    }

    /// Load a chunk from the storage backend. If the chunk is in the cache, it will be returned
    /// from the cache instead of the storage backend. If the chunk is not in the cache, it will be
    /// loaded from the storage backend and inserted into the cache.
    pub fn load_chunk(
        &'_ self,
        pos: ChunkPos,
        dimension: &str,
    ) -> Result<RefChunk<'_>, WorldError> {
        if let Some(chunk) = self.cache.get(&(pos, dimension.to_string())) {
            return Ok(chunk);
        }
        let chunk = load_chunk_internal(self, pos, dimension);
        match chunk {
            Ok(c) => {
                self.cache.insert((pos, dimension.to_string()), c);
                Ok(self
                    .cache
                    .get(&(pos, dimension.to_string()))
                    .expect("Chunk was just inserted into the cache"))
            }
            Err(e) => Err(e),
        }
    }

    /// Load a mutable chunk from the storage backend. If the chunk is in the cache, it will be returned
    /// from the cache instead of the storage backend. If the chunk is not in the cache, it will be
    /// loaded from the storage backend and inserted into the cache.
    pub fn load_chunk_mut(
        &'_ self,
        pos: ChunkPos,
        dimension: &'_ str,
    ) -> Result<MutChunk<'_>, WorldError> {
        if let Some(chunk) = self.cache.get_mut(&(pos, dimension.to_string())) {
            return Ok(chunk);
        }
        let chunk = load_chunk_internal(self, pos, dimension);
        match chunk {
            Ok(c) => {
                self.cache.insert((pos, dimension.to_string()), c);
                Ok(self
                    .cache
                    .get_mut(&(pos, dimension.to_string()))
                    .expect("Chunk was just inserted into the cache"))
            }
            Err(e) => Err(e),
        }
    }

    /// Returns a read guard for a chunk **only if it is already resident in the in-memory cache**,
    /// without ever loading from disk or generating. Used by hot paths (e.g. fluid ticks) that must
    /// only operate on actively loaded chunks and must never trigger generation/IO on the tick thread.
    pub fn cached_chunk(&'_ self, pos: ChunkPos, dimension: &str) -> Option<RefChunk<'_>> {
        self.cache.get(&(pos, dimension.to_string()))
    }

    /// Mutable counterpart of [`World::cached_chunk`]: a write guard only if the chunk is already in
    /// the cache, never loading or generating.
    pub fn cached_chunk_mut(&'_ self, pos: ChunkPos, dimension: &str) -> Option<MutChunk<'_>> {
        self.cache.get_mut(&(pos, dimension.to_string()))
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
    ///
    /// All dirty chunks are serialised and written in a single batched transaction rather than one
    /// transaction per chunk. A commit is the costly part of an LMDB write (it updates the meta
    /// page and, on a sync boundary, hits the disk), so collapsing a whole sync into one commit
    /// turns N commits into one and was a major source of the world-sync schedule overrunning its
    /// budget when many chunks were dirty.
    pub fn sync(&self) -> Result<(), WorldError> {
        // Serialise dirty chunks first (read-only over the cache), collecting their storage keys so
        // they can be marked clean once the single batched write has persisted them.
        let mut batch: Vec<(u128, Vec<u8>)> = Vec::new();
        let mut written: Vec<(ChunkPos, String)> = Vec::new();
        for pair in self.cache.iter() {
            if !pair.value().sections.iter().any(|c| c.dirty) {
                continue;
            }
            let pos = pair.key().0;
            let dim = pair.key().1.clone();
            trace!("Syncing chunk: {:?}", pos);
            let encoded = yazi::compress(
                &bitcode::encode(pair.value()),
                yazi::Format::Zlib,
                yazi::CompressionLevel::BestSpeed,
            )?;
            batch.push((create_key(&dim, pos), encoded));
            written.push((pos, dim));
        }

        if !batch.is_empty() {
            self.storage_backend
                .batch_upsert("chunks".to_string(), batch)?;
            // Mark the now-persisted chunks clean, so the next sync only writes chunks that changed
            // since. Without this every chunk ever modified stays dirty forever and every sync
            // re-serialises and rewrites the entire accumulated set — a cost that grows without bound
            // as the world is explored and was overrunning the sync budget by seconds. The sync runs
            // on the single tick-loop thread, so no chunk can be re-dirtied between serialising it
            // above and clearing the flag here.
            for (pos, dim) in written {
                if let Some(mut chunk) = self.cache.get_mut(&(pos, dim)) {
                    chunk.sections.iter_mut().for_each(|c| c.dirty = false);
                }
            }
        }

        sync_internal(self)
    }

    /// Load a batch of chunks from the storage backend.
    ///
    /// This function attempts to load as many chunks as it can find from the cache first, then fetches
    /// the missing chunks from the storage backend. The chunks are then inserted into the cache and
    /// returned as a vector.
    pub fn load_chunk_batch(
        &'_ self,
        coords: &'_ [(ChunkPos, &'_ str)],
    ) -> Result<Vec<RefChunk<'_>>, WorldError> {
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
            self.cache.insert((pos, dimension.to_string()), chunk);
            let found_chunk = self
                .cache
                .get(&(pos, dimension.to_string()))
                .expect("Chunk was just inserted into the cache");
            found_chunks.push(found_chunk);
        }
        Ok(found_chunks)
    }

    pub fn load_chunk_batch_mut(
        &'_ self,
        coords: &'_ [(ChunkPos, &'_ str)],
    ) -> Result<Vec<MutChunk<'_>>, WorldError> {
        let mut found_chunks = Vec::new();
        let mut missing_chunks = Vec::new();
        for coord in coords {
            if let Some(chunk) = self.cache.get_mut(&(coord.0, coord.1.to_string())) {
                found_chunks.push(chunk);
            } else {
                missing_chunks.push(*coord);
            }
        }
        let fetched = load_chunk_batch_internal(self, &missing_chunks)?;
        for (chunk, (pos, dimension)) in fetched.into_iter().zip(missing_chunks) {
            self.cache.insert((pos, dimension.to_string()), chunk);
            let found_chunk = self
                .cache
                .get_mut(&(pos, dimension.to_string()))
                .expect("Chunk was just inserted into the cache");
            found_chunks.push(found_chunk);
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
            self.cache.insert((pos, dimension.to_string()), chunk);
        }
        Ok(())
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
