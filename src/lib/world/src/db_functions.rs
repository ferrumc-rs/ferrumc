use ferrumc_storage::compressors::Compressor;
use crate::chunk_format::Chunk;
use crate::errors::WorldError;
use crate::World;

impl World {
    pub async fn save_chunk(&self, chunk: Chunk) -> Result<(), WorldError> {
        save_chunk_internal(self, chunk).await
    }
    
    pub async fn load_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldError> {
        load_chunk_internal(self, &self.compressor, x, z).await
    }
    
    pub async fn chunk_exists(&self, x: i32, z: i32) -> Result<bool, WorldError> {
        chunk_exists_internal(self, x, z).await
    }
    
    pub async fn delete_chunk(&self, x: i32, z: i32) -> Result<(), WorldError> {
        delete_chunk_internal(self, x, z).await
    }

    pub async fn sync(&self) -> Result<(), WorldError> {
        sync_internal(self).await
    }
}

pub(crate) async fn save_chunk_internal(
    world: &World,
    chunk: Chunk,
) -> Result<(), WorldError> {
    let as_bytes = world.compressor.compress(&bitcode::encode(&chunk))?;
    let digest = ferrumc_general_purpose::hashing::hash((chunk.x, chunk.z));
    world.storage_backend.upsert("chunks".to_string(), digest, as_bytes).await?;
    Ok(())
}

pub(crate) async fn load_chunk_internal(
    world: &World,
    compressor: &Compressor,
    x: i32,
    z: i32,
) -> Result<Chunk, WorldError> {
    let digest = ferrumc_general_purpose::hashing::hash((x, z));
    match world.storage_backend.get("chunks".to_string(), digest).await? {
        Some(compressed) => {
            let data = compressor.decompress(&compressed)?;
            let chunk: Chunk = bitcode::decode(&data).map_err(|e| WorldError::BitcodeDecodeError(e.to_string()))?;
            Ok(chunk)
        }
        None => Err(WorldError::ChunkNotFound),
    }
}

pub(crate) async fn chunk_exists_internal(
    world: &World,
    x: i32,
    z: i32,
) -> Result<bool, WorldError> {
    let digest = ferrumc_general_purpose::hashing::hash((x, z));
    Ok(world.storage_backend.exists("chunks".to_string(), digest).await?)
}

pub(crate) async fn delete_chunk_internal(
    world: &World,
    x: i32,
    z: i32,
) -> Result<(), WorldError> {
    let digest = ferrumc_general_purpose::hashing::hash((x, z));
    world.storage_backend.delete("chunks".to_string(), digest).await?;
    Ok(())
}

pub(crate) async fn sync_internal(world: &World) -> Result<(), WorldError> {
    world.storage_backend.flush().await?;
    Ok(())
}