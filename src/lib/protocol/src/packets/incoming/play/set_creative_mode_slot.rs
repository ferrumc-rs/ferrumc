use crate::ids;
use ferrumc_core::items::inventory_slot::InventorySlot;
use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_SET_CREATIVE_MODE_SLOT, state = "play")]
pub struct SetCreativeModeSlot {
    pub slot_index: i16,
    pub slot: InventorySlot,
}
