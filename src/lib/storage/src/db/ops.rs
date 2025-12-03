use crate::errors::WorldError;
use crate::errors::WorldError::CorruptedChunkData;
use crate::ChunkStorage;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::world::chunk_format::Chunk; // Core Type
use std::sync::Arc;
use tracing::{trace, warn};
use yazi::CompressionLevel;

// Helper: Create Key
fn create_key(x: i32, z: i32) -> u128 {
    let x_u64 = (x as u64) & 0xFFFFFFFF;
    let z_u64 = (z as u64) & 0xFFFFFFFF;
    ((x_u64 as u128) << 64) | (z_u64 as u128)
}

// --- CRUD ---

pub(crate) fn save_chunk(storage: &ChunkStorage, chunk: Arc<Chunk>) -> Result<(), WorldError> {
    // 1. Ensure Table
    if !storage.backend.table_exists("chunks".to_string())? {
        storage.backend.create_table("chunks".to_string())?;
    }

    // 2. Serialize & Compress
    let raw_bytes = bitcode::encode(chunk.as_ref());
    let compressed = yazi::compress(&raw_bytes, yazi::Format::Zlib, CompressionLevel::BestSpeed)
        .map_err(WorldError::from)?;

    // 3. Write
    let key = create_key(chunk.x, chunk.z);
    storage
        .backend
        .upsert("chunks".to_string(), key, compressed)?;

    // 4. Cache
    storage.cache.insert((chunk.x, chunk.z), chunk);
    Ok(())
}

pub(crate) fn load_chunk(storage: &ChunkStorage, x: i32, z: i32) -> Result<Arc<Chunk>, WorldError> {
    // 1. Cache
    if let Some(chunk) = storage.cache.get(&(x, z)) {
        return Ok(chunk);
    }

    // 2. Disk
    let key = create_key(x, z);
    match storage.backend.get("chunks".to_string(), key)? {
        Some(compressed) => {
            // 3. Decompress
            let (data, checksum) =
                yazi::decompress(&compressed, yazi::Format::Zlib).map_err(WorldError::from)?;

            // 4. Verify
            if get_global_config().database.verify_chunk_data {
                if let Some(expected) = checksum {
                    let real = yazi::Adler32::from_buf(&data).finish();
                    if real != expected {
                        return Err(CorruptedChunkData(real, expected));
                    }
                }
            }

            // 5. Deserialize
            let chunk: Chunk = bitcode::decode(&data)
                .map_err(|e| WorldError::BitcodeDecodeError(e.to_string()))?;

            let chunk_arc = Arc::new(chunk);
            storage.cache.insert((x, z), chunk_arc.clone());
            Ok(chunk_arc)
        }
        None => Err(WorldError::ChunkNotFound),
    }
}

pub(crate) fn chunk_exists(storage: &ChunkStorage, x: i32, z: i32) -> Result<bool, WorldError> {
    if storage.cache.contains_key(&(x, z)) {
        return Ok(true);
    }
    if !storage.backend.table_exists("chunks".to_string())? {
        return Ok(false);
    }
    let key = create_key(x, z);
    storage
        .backend
        .exists("chunks".to_string(), key)
        .map_err(Into::into)
}

pub(crate) fn delete_chunk(storage: &ChunkStorage, x: i32, z: i32) -> Result<(), WorldError> {
    storage.cache.remove(&(x, z));
    let key = create_key(x, z);
    storage
        .backend
        .delete("chunks".to_string(), key)
        .map_err(Into::into)
}

pub(crate) fn sync_database(storage: &ChunkStorage) -> Result<(), WorldError> {
    storage.backend.flush().map_err(Into::into)
}

// Key generation for players (UUID -> Byte Array)
fn create_player_key(uuid: Uuid) -> [u8; 16] {
    uuid.as_bytes().clone()
}

pub(crate) fn save_player(
    storage: &ChunkStorage,
    uuid: Uuid,
    data: &SavedPlayerData,
) -> Result<(), WorldError> {
    // 1. Ensure Table Exists
    if !storage.backend.table_exists("players".to_string())? {
        storage.backend.create_table("players".to_string())?;
    }

    // 2. Serialize (Bitcode is fast and small)
    let bytes = bitcode::encode(data);
    // We usually don't need to compress player data (it's small),
    // but you can if you want. LMDB handles small values well.

    // 3. Write
    let key = create_player_key(uuid);
    storage
        .backend
        .upsert_bytes("players".to_string(), &key, bytes)?;

    Ok(())
}

pub(crate) fn load_player(
    storage: &ChunkStorage,
    uuid: Uuid,
) -> Result<Option<SavedPlayerData>, WorldError> {
    let key = create_player_key(uuid);

    // Check if table exists first to avoid error
    if !storage.backend.table_exists("players".to_string())? {
        return Ok(None);
    }

    match storage.backend.get_bytes("players".to_string(), &key)? {
        Some(bytes) => {
            let data: SavedPlayerData = bitcode::decode(&bytes)
                .map_err(|e| WorldError::BitcodeDecodeError(e.to_string()))?;
            Ok(Some(data))
        }
        None => Ok(None),
    }
}
