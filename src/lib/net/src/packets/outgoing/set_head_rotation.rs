use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::angle::NetAngle;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(Debug, NetEncode, Clone)]
#[packet(packet_id = "rotate_head", state = "play")]
pub struct SetHeadRotationPacket {
    pub entity_id: VarInt,
    pub head_yaw: NetAngle,
}

impl SetHeadRotationPacket {
    pub fn new(entity_id: i32, head_yaw: NetAngle) -> Self {
        Self {
            entity_id: VarInt::new(entity_id),
            head_yaw,
        }
    }
}
