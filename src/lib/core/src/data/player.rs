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
