use bevy_ecs::component::Component;
use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::transform::position::Position;

// https://minecraft.fandom.com/wiki/Player.dat_format
#[derive(Serialize, Deserialize, Debug, Encode, Decode, Component, typename::TypeName)]
pub struct PlayerData {
    pub uuid: u128,
    pub pos: Position,
    pub dimension: String,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::new(0, Position::default(), "overworld")
    }
}

impl PlayerData {
    pub fn new(uuid: u128, pos: Position, dimension: &str) -> Self {
        Self {
            uuid,
            pos,
            dimension: dimension.to_string(),
        }
    }
}
