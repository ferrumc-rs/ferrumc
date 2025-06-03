use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::angle::NetAngle;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode, Clone)]
#[packet(packet_id = "teleport_entity", state = "play")]
pub struct TeleportEntityPacket {
    pub entity_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
    pub yaw: NetAngle,
    pub pitch: NetAngle,
    pub on_ground: bool,
}

impl TeleportEntityPacket {
    pub fn new(
        entity_id: &PlayerIdentity,
        position: &Position,
        angle: &Rotation,
        on_ground: bool,
    ) -> Self {
        // Todo: Add velocity parameters if needed
        Self {
            entity_id: VarInt::new(entity_id.short_uuid),
            x: position.x,
            y: position.y,
            z: position.z,
            vel_x: 0.0, // Placeholder for velocity in x direction
            vel_y: 0.0, // Placeholder for velocity in y direction
            vel_z: 0.0, // Placeholder for velocity in z direction
            yaw: NetAngle::from_degrees(angle.yaw as f64),
            pitch: NetAngle::from_degrees(angle.pitch as f64),
            on_ground,
        }
    }
}
