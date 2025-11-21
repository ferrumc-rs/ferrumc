use std::fmt::Display;

use bevy_ecs::component::Component;
use bevy_math::DVec3;

use crate::player::gamemode::GameMode;
use crate::transform::{position::Position, rotation::Rotation};

use ferrumc_storage::errors::StorageError;
use ferrumc_storage::sqlite::SqlStorable;

// https://minecraft.fandom.com/wiki/Player.dat_format
#[derive(Debug, Component, typename::TypeName, Clone)]
pub struct PlayerData {
    pub pos: DVec3,
    pub on_ground: bool,
    pub dimension: String,
    pub rotation: (f32, f32),
    pub player_game_type: u8,
}

impl Default for PlayerData {
    fn default() -> Self {
        let rot = Rotation::default();
        let pos = Position::default();
        Self {
            pos: pos.into(),
            on_ground: false,
            dimension: "overworld".to_string(),
            rotation: (rot.yaw, rot.pitch),
            player_game_type: GameMode::Survival as u8,
        }
    }
}

impl Display for PlayerData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(pos: {:?}, on_ground: {}, dimension: {}, rotation: ({:.2}, {:.2}), player_game_type: {})",
            self.pos, self.on_ground, self.dimension, self.rotation.0, self.rotation.1, self.player_game_type
        )
    }
}

impl SqlStorable for PlayerData {
    fn schema() -> &'static str {
        "key TEXT PRIMARY KEY, pos_x REAL NOT NULL, pos_y REAL NOT NULL, pos_z REAL NOT NULL, on_ground INTEGER NOT NULL, dimension TEXT NOT NULL, rotation_yaw REAL NOT NULL, rotation_pitch REAL NOT NULL, player_game_type INTEGER NOT NULL"
    }

    fn columns() -> &'static str {
        "pos_x, pos_y, pos_z, on_ground, dimension, rotation_yaw, rotation_pitch, player_game_type"
    }

    fn value_placeholders() -> &'static str {
        "?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9"
    }

    fn bind_values(
        &self,
        stmt: &mut rusqlite::Statement,
        start_idx: usize,
    ) -> Result<(), StorageError> {
        stmt.raw_bind_parameter(start_idx, self.pos.x)?;
        stmt.raw_bind_parameter(start_idx + 1, self.pos.y)?;
        stmt.raw_bind_parameter(start_idx + 2, self.pos.z)?;
        stmt.raw_bind_parameter(start_idx + 3, if self.on_ground { 1 } else { 0 })?;
        stmt.raw_bind_parameter(start_idx + 4, self.dimension.as_str())?;
        stmt.raw_bind_parameter(start_idx + 5, self.rotation.0)?;
        stmt.raw_bind_parameter(start_idx + 6, self.rotation.1)?;
        stmt.raw_bind_parameter(start_idx + 7, self.player_game_type)?;
        Ok(())
    }

    fn from_row(row: &rusqlite::Row) -> Result<Self, StorageError> {
        Ok(PlayerData {
            pos: DVec3::new(row.get(1)?, row.get(2)?, row.get(3)?),
            on_ground: row.get::<_, i32>(4)? != 0,
            dimension: row.get(5)?,
            rotation: (row.get(6)?, row.get(7)?),
            player_game_type: row.get(8)?,
        })
    }
}
