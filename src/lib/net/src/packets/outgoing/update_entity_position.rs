use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetEncode, Clone)]
#[packet(packet_id = "move_entity_pos", state = "play")]
pub struct UpdateEntityPositionPacket {
    pub entity_id: VarInt,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub on_ground: bool,
}

impl UpdateEntityPositionPacket {
    pub fn new(
        entity_id: &PlayerIdentity,
        delta_positions: (i16, i16, i16),
        on_ground: bool,
    ) -> Self {
        Self {
            entity_id: VarInt::new(entity_id.short_uuid),
            delta_x: delta_positions.0,
            delta_y: delta_positions.1,
            delta_z: delta_positions.2,
            on_ground,
        }
    }
}
