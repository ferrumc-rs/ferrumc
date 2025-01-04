use ferrumc_core::transform::rotation::Rotation;
use ferrumc_ecs::entities::Entity;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::angle::NetAngle;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x30, state = "play")]
pub struct UpdateEntityRotationPacket {
    pub entity_id: VarInt,
    pub yaw: NetAngle,
    pub pitch: NetAngle,
    pub on_ground: bool,
}
impl UpdateEntityRotationPacket {
    pub fn new(entity_id: Entity, new_rot: &Rotation, on_ground: bool) -> Self {
        Self {
            entity_id: VarInt::new(entity_id as i32),
            yaw: NetAngle::from_degrees(new_rot.yaw as f64),
            pitch: NetAngle::from_degrees(new_rot.pitch as f64),
            on_ground,
        }
    }
}
