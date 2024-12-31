use std::io::Write;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::angle::NetAngle;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, NetEncode)]
#[packet(packet_id = 0x48)]
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