use bevy_ecs::prelude::{Entity, Message};

#[derive(Message)]
pub struct TeleportPlayer {
    pub entity: Entity,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
    pub yaw: f32,
    pub pitch: f32,
}
