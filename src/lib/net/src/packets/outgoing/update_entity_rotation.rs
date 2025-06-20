use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::angle::NetAngle;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode, Clone)]
#[packet(packet_id = "move_entity_rot", state = "play")]
pub struct UpdateEntityRotationPacket {
    pub entity_id: VarInt,
    pub yaw: NetAngle,
    pub pitch: NetAngle,
    pub on_ground: bool,
}
impl UpdateEntityRotationPacket {
    pub fn new(entity_id: &PlayerIdentity, new_rot: &Rotation, on_ground: bool) -> Self {
        Self {
            entity_id: VarInt::new(entity_id.short_uuid),
            yaw: NetAngle::from_degrees(new_rot.yaw as f64),
            pitch: NetAngle::from_degrees(new_rot.pitch as f64),
            on_ground,
        }
    }
}
