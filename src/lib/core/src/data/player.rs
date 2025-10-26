use std::fmt::Display;

use bevy_ecs::component::Component;
use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::transform::{position::Position, rotation::Rotation};

// https://minecraft.fandom.com/wiki/Player.dat_format
#[derive(
    Serialize, Deserialize, Clone, Debug, Encode, Decode, Component, typename::TypeName, PartialEq,
)]
pub struct PlayerData {
    pub pos: Position,
    pub on_ground: bool,
    pub dimension: String,
    pub rotation: (f32, f32),
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::new(
            &Position::default(),
            false,
            "overworld",
            &Rotation::default(),
        )
    }
}

impl PlayerData {
    pub fn new(pos: &Position, on_ground: bool, dimension: &str, rotation: &Rotation) -> Self {
        Self {
            pos: pos.to_owned(),
            on_ground,
            dimension: dimension.to_string(),
            rotation: (rotation.yaw, rotation.pitch),
        }
    }
}

impl Display for PlayerData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(pos: {:?}, on_ground: {}, dimension: {}, rotation: ({:.2}, {:.2}))",
            self.pos, self.on_ground, self.dimension, self.rotation.0, self.rotation.1
        )
    }
}
