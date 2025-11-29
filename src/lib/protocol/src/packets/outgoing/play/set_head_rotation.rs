use crate::codec::net_types::angle::NetAngle;
use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use ferrumc_macros::{NetEncode, packet};

#[derive(Debug, NetEncode, Clone)]
#[packet(id = ids::PLAY_CLIENTBOUND_ROTATE_HEAD, state = "play")]
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
