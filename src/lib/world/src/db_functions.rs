use crate::chunk_format::Chunk;
use crate::errors::WorldError;
use crate::World;

impl World {
    pub async fn save_chunk(&mut self, chunk: Chunk) -> Result<(), WorldError> {
        let as_bytes = bitcode::encode(&chunk);
        let key = format!("{}-{}", chunk.x, chunk.z);
        let digest = ferrumc_general_purpose::hashing::hash(&key);
        self.storage_backend.upsert("chunks".to_string(), digest, as_bytes).await?;
        Ok(())
    }
    
    pub async fn load_chunk(&mut self, x: i32, z: i32) -> Result<Chunk, WorldError> {
        let key = format!("{}-{}", x, z);
        let digest = ferrumc_general_purpose::hashing::hash(&key);
        match self.storage_backend.get("chunks".to_string(), digest).await? {
            Some(data) => {
                let chunk: Chunk = bitcode::decode(&data).map_err(|e| WorldError::BitcodeDecodeError(e.to_string()))?;
                Ok(chunk)
            }
            None => Err(WorldError::ChunkNotFound),
        }
    }
    
    pub async fn chunk_exists(&mut self, x: i32, z: i32) -> Result<bool, WorldError> {
        let key = format!("{}-{}", x, z);
        let digest = ferrumc_general_purpose::hashing::hash(&key);
        Ok(self.storage_backend.exists("chunks".to_string(), digest).await?)
    }
    
    pub async fn delete_chunk(&mut self, x: i32, z: i32) -> Result<(), WorldError> {
        let key = format!("{}-{}", x, z);
        let digest = ferrumc_general_purpose::hashing::hash(&key);
        self.storage_backend.delete("chunks".to_string(), digest).await?;
        Ok(())
    }
}