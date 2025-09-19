use bevy_ecs::component::Component;
use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::transform::position::Position;

// https://minecraft.fandom.com/wiki/Player.dat_format
#[derive(
    Serialize, Deserialize, Clone, Debug, Encode, Decode, Component, typename::TypeName, PartialEq,
)]
pub struct PlayerData {
    pub pos: Position,
    pub dimension: String,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::new(Position::default(), "overworld")
    }
}

impl PlayerData {
    pub fn new(pos: Position, dimension: &str) -> Self {
        Self {
            pos,
            dimension: dimension.to_string(),
        }
    }
}
