use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use ferrumc_core::player::identity::PlayerIdentityData;
use ferrumc_core::player::transform::position::PositionData;
use ferrumc_core::player::transform::rotation::RotationData;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode, Clone)]
#[packet(id = ids::PLAY_CLIENTBOUND_ENTITY_POSITION_SYNC, state = "play")]
pub struct TeleportEntityPacket {
    pub entity_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

impl TeleportEntityPacket {
    pub fn new(
        entity_id: &PlayerIdentityData,
        position: &PositionData,
        angle: &RotationData,
        on_ground: bool,
    ) -> Self {
        // TODO: Add velocity parameters if needed
        Self {
            entity_id: VarInt::new(entity_id.short_uuid),
            x: position.x,
            y: position.y,
            z: position.z,
            vel_x: 0.0, // Placeholder for velocity in x direction
            vel_y: 0.0, // Placeholder for velocity in y direction
            vel_z: 0.0, // Placeholder for velocity in z direction
            yaw: angle.yaw,
            pitch: angle.pitch,
            on_ground,
        }
    }
}
