use crate::errors::WorldError;
use crate::World;
use tracing::trace;

impl World {
    /// Loads player data from the storage backend and decodes it.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A type that implements the `bitcode::DecodeOwned` trait, representing the structure of the decoded data.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The unique identifier of the player whose data is to be loaded.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(T))` - The decoded player data if it exists and can be successfully decoded.
    /// * `Ok(None)` - If no data is found for the given player.
    /// * `Err(WorldError)` - If an error occurs during the operation or decoding fails.
    pub fn load_player_data<T: bitcode::DecodeOwned>(
        &self,
        uuid: uuid::Uuid,
    ) -> Result<Option<T>, WorldError> {
        if !self
            .storage_backend
            .table_exists("player_data".to_string())?
        {
            trace!(
                "Player data table does not exist. Returning None for player {}",
                uuid
            );
            return Ok(None);
        }
        let data = self
            .storage_backend
            .get("player_data".to_string(), uuid.as_u128())
            .map_err(WorldError::DatabaseError);
        data.map(|opt_bytes| opt_bytes.and_then(|bytes| bitcode::decode(&bytes).ok()))
    }

    /// Saves player data to the storage backend after encoding it.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A type that implements the `bitcode::Encode` trait, representing the structure of the data to be encoded.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The unique identifier of the player whose data is to be saved.
    /// * `data` - A reference to the data to be encoded and saved.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If the data was successfully saved.
    /// * `Ok(false)` - If the data could not be saved.
    /// * `Err(WorldError)` - If an error occurs during the operation.
    pub fn save_player_data<T: bitcode::Encode>(
        &self,
        uuid: uuid::Uuid,
        data: &T,
    ) -> Result<bool, WorldError> {
        if !self
            .storage_backend
            .table_exists("player_data".to_string())?
        {
            self.storage_backend
                .create_table("player_data".to_string())
                .map_err(WorldError::DatabaseError)?;
        }
        self.storage_backend
            .upsert(
                "player_data".to_string(),
                uuid.as_u128(),
                bitcode::encode(data),
            )
            .map_err(WorldError::DatabaseError)
    }
}
