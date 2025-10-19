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
    pub on_ground: bool,
    pub dimension: String,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::new(Position::default(), false, "overworld", 0.0, 0.0)
    }
}

impl PlayerData {
    pub fn new(pos: Position, on_ground: bool, dimension: &str, yaw: f32, pitch: f32) -> Self {
        Self {
            pos,
            on_ground,
            dimension: dimension.to_string(),
            yaw,
            pitch,
        }
    }

    pub fn update_position(&mut self, new_position: Position) {
        self.pos = new_position;
    }

    pub fn update_on_ground(&mut self, on_ground: bool) {
        self.on_ground = on_ground;
    }

    pub fn update_yaw(&mut self, new_yaw: f32) {
        self.yaw = new_yaw;
    }

    pub fn update_pitch(&mut self, new_pitch: f32) {
        self.pitch = new_pitch;
    }
}
