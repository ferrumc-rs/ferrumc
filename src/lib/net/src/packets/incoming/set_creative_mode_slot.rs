use ferrumc_core::items::inventory_slot::InventorySlot;
use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode)]
#[packet(packet_id = "set_creative_mode_slot", state = "play")]
pub struct SetCreativeModeSlot {
    pub slot_index: i16,
    pub slot: InventorySlot,
}
