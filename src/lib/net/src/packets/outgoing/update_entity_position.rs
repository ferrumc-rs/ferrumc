use ferrumc_components::player::identity::PlayerIdentity;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

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
