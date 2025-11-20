use ferrumc_core::items::inventory_slot::InventorySlot;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
#[packet(packet_id = "set_player_inventory", state = "play")]
/// # This packet is buggy and does not seem to work.
pub struct SetPlayerInventorySlot {
    pub slot_index: VarInt,
    pub slot: InventorySlot,
}
