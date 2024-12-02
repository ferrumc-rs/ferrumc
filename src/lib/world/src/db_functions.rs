use crate::chunk_format::Chunk;
use crate::errors::WorldError;
use crate::World;
use ferrumc_storage::compressors::Compressor;
use std::hash::Hasher;

impl World {
    pub async fn save_chunk(&self, chunk: Chunk) -> Result<(), WorldError> {
        save_chunk_internal(self, chunk).await
    }

    pub async fn load_chunk(&self, x: i32, z: i32, dimension: &str) -> Result<Chunk, WorldError> {
        load_chunk_internal(self, &self.compressor, x, z, dimension).await
    }

    pub async fn chunk_exists(&self, x: i32, z: i32, dimension: &str) -> Result<bool, WorldError> {
        chunk_exists_internal(self, x, z, dimension).await
    }

    pub async fn delete_chunk(&self, x: i32, z: i32, dimension: &str) -> Result<(), WorldError> {
        delete_chunk_internal(self, x, z, dimension).await
    }

    pub async fn sync(&self) -> Result<(), WorldError> {
        sync_internal(self).await
    }

    pub async fn load_chunk_batch(
        &self,
        coords: Vec<(i32, i32, &str)>,
    ) -> Result<Vec<Chunk>, WorldError> {
        load_chunk_batch_internal(self, coords).await
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
