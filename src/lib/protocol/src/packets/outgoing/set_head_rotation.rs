use ferrumc_macros::{packet, NetEncode};
use ferrumc_protocol::codec::net_types::angle::NetAngle;
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

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
