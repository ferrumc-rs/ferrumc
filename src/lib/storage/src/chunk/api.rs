use crate::errors::WorldError;
use crate::storage;
use crate::ChunkStorage;
use ferrumc_core::world::chunk_format::Chunk;
use std::sync::Arc;
use tracing::trace;

// We implement the high-level public API on ChunkStorage here.
// This delegates to the low-level functions in `storage.rs`.

impl ChunkStorage {
    /// Save a chunk to the storage backend.
    ///
    /// This updates the in-memory cache and persists to disk (LMDB).
    pub fn save_chunk(&self, chunk: Arc<Chunk>) -> Result<(), WorldError> {
        storage::save_chunk(self, chunk)
    }

    /// Load a chunk from storage.
    ///
    /// Checks cache first, then disk. Returns an Arc for shared access.
    pub fn get_chunk(&self, x: i32, z: i32) -> Result<Arc<Chunk>, WorldError> {
        storage::load_chunk(self, x, z)
    }

    /// Load a chunk and clone it.
    ///
    /// Useful when you need mutable access to modify the chunk (Copy-on-Write).
    pub fn load_chunk_owned(&self, x: i32, z: i32) -> Result<Chunk, WorldError> {
        self.get_chunk(x, z).map(|arc| arc.as_ref().clone())
    }

    /// Check if a chunk exists (in cache or on disk).
    pub fn chunk_exists(&self, x: i32, z: i32) -> Result<bool, WorldError> {
        storage::chunk_exists(self, x, z)
    }

    /// Delete a chunk from cache and disk.
    pub fn delete_chunk(&self, x: i32, z: i32) -> Result<(), WorldError> {
        storage::delete_chunk(self, x, z)
    }

    /// Flush all pending database operations to disk.
    pub fn flush(&self) -> Result<(), WorldError> {
        storage::sync_database(self)
    }

    /// Pre-cache a chunk without returning it.
    ///
    /// Useful for pre-loading areas around a player.
    pub fn pre_cache(&self, x: i32, z: i32) -> Result<(), WorldError> {
        // We just call load_chunk, which populates the cache side-effect.
        let _ = self.get_chunk(x, z)?;
        Ok(())
    }

    /// Load a batch of chunks efficiently.
    ///
    /// This minimizes lock contention and potential disk seeks if optimized later.
    pub fn load_chunk_batch(&self, coords: &[(i32, i32)]) -> Result<Vec<Arc<Chunk>>, WorldError> {
        let mut chunks = Vec::with_capacity(coords.len());

        // For now, we just iterate.
        // Optimizing this to a batch LMDB read is a future task if performance demands it.
        for &(x, z) in coords {
            // We ignore errors for individual chunks in a batch?
            // Or do we fail the whole batch?
            // Usually better to return what we found or error if critical.
            // Let's propagate error for now.
            chunks.push(self.get_chunk(x, z)?);
        }

        Ok(chunks)
    }
}
