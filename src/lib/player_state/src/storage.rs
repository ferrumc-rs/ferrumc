use std::sync::Arc;

use ferrumc_core::data::player::PlayerData;
use ferrumc_storage::{database::Database, errors::StorageError, lmdb::LmdbBackend};
use yazi::CompressionLevel;

use crate::errors::PlayerDataError;

// Table name for player state data in LMDB
const TABLE_NAME: &str = "playerdata";

#[derive(Clone)]
pub struct PlayerStateStorage {
    storage_backend: Arc<LmdbBackend>,
}

impl PlayerStateStorage {
    pub fn new(storage_backend: Arc<LmdbBackend>) -> Self {
        Self { storage_backend }
    }

    pub fn save_player_state(&self, state: &PlayerData) -> Result<(), PlayerDataError> {
        if !self.storage_backend.table_exists(TABLE_NAME.to_string())? {
            self.storage_backend.create_table(TABLE_NAME)?;
        }
        let key = state.uuid;

        let encoded = yazi::compress(
            &bitcode::encode(state),
            yazi::Format::Zlib,
            CompressionLevel::BestSpeed,
        )?;
        self.storage_backend.upsert(TABLE_NAME, key, encoded)?;
        Ok(())
    }

    pub fn load_player_state(&self, key: u128) -> Result<Option<PlayerData>, PlayerDataError> {
        if let Some(bytes) = self.storage_backend.get(TABLE_NAME, key)? {
            let (data, _) = yazi::decompress(bytes.as_slice(), yazi::Format::Zlib)?;
            let player: PlayerData = bitcode::decode(&data)
                .map_err(|_| StorageError::ReadError("Failed to decode PlayerState".into()))?;
            Ok(Some(player))
        } else {
            Ok(None)
        }
    }
}
