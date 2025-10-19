use ferrumc_core::data::player::PlayerData;
use ferrumc_storage::database::Database;

use crate::{errors::PlayerDataError, World};

// Table name for player state data in SQLite
const TABLE_NAME: &str = "playerdata";

impl World {
    pub fn save_player_state(&self, key: u128, state: &PlayerData) -> Result<(), PlayerDataError> {
        self.player_state_backend.create_table(TABLE_NAME)?;
        self.player_state_backend.upsert(TABLE_NAME, key, state)?;
        tracing::info!("Saving position {} {} {}", state.pos.x, state.pos.y, state.pos.z);
        Ok(())
    }

    pub fn load_player_state(&self, key: u128) -> Result<Option<PlayerData>, PlayerDataError> {
        if let Some(player) = self.player_state_backend.get(TABLE_NAME, key)? {
            Ok(Some(player))
        } else {
            Ok(None)
        }
    }
}
