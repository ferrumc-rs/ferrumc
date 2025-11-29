use crate::ids;
use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_SET_CARRIED_ITEM, state = "play")]
pub struct SetHeldItem {
    pub slot_index: i16,
}
