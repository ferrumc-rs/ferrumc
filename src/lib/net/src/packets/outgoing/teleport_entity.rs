use ferrumc_ecs::entities::Entity;
use ferrumc_macros::{packet, NetEncode};
use std::io::Write;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_net_codec::net_types::angle::NetAngle;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
#[packet(packet_id = 0x70)]
pub struct TeleportEntityPacket {
    pub entity_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: NetAngle,
    pub pitch: NetAngle,
    pub on_ground: bool,
}


impl TeleportEntityPacket {
    pub fn new(entity_id: Entity, position: &Position, angle: &Rotation, on_ground: bool) -> Self {
        Self {
            entity_id: VarInt::new(entity_id as i32),
            x: position.x,
            y: position.y,
            z: position.z,
            yaw: NetAngle::from_degrees(angle.yaw as f64),
            pitch: NetAngle::from_degrees(angle.pitch as f64),
            on_ground,
        }
    }
}