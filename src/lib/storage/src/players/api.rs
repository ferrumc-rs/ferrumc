use crate::errors::WorldError;
use crate::ChunkStorage; // The main struct
use bitcode;
use ferrumc_core::player::saved_data::SavedPlayerData;
use uuid::Uuid;

impl ChunkStorage {
    /// Loads player data from the database.
    pub fn load_player(&self, uuid: Uuid) -> Result<Option<SavedPlayerData>, WorldError> {
        // 1. Generate Key (UUID bytes)
        let key = uuid.as_bytes();

        // 2. Check if table exists to avoid errors on fresh db
        if !self.backend.table_exists("players".to_string())? {
            return Ok(None);
        }

        // 3. Get bytes
        // (Using .get_bytes or generic .get depending on the LMDB wrapper)
        // Assuming backend.get returns Option<Vec<u8>>
        let bytes_opt = self
            .backend
            .get("players".to_string(), u128::from_be_bytes(*key))?;

        // Wait, UUID is 128 bit, LMDB key in the wrapper is u128.
        // Perfect match! u128::from_be_bytes(*uuid.as_bytes())

        match bytes_opt {
            Some(bytes) => {
                // 4. Deserialize
                let data: SavedPlayerData = bitcode::decode(&bytes)
                    .map_err(|e| WorldError::BitcodeDecodeError(e.to_string()))?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    /// Saves player data to the database.
    pub fn save_player(&self, uuid: Uuid, data: &SavedPlayerData) -> Result<(), WorldError> {
        // 1. Ensure table
        if !self.backend.table_exists("players".to_string())? {
            self.backend.create_table("players".to_string())?;
        }

        // 2. Serialize
        let bytes = bitcode::encode(data);

        // 3. Write
        let key = u128::from_be_bytes(*uuid.as_bytes());
        self.backend.upsert("players".to_string(), key, &bytes)?;

        Ok(())
    }
}
