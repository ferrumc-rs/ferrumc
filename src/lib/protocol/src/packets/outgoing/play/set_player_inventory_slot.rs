use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use ferrumc_core::items::inventory_slot::InventorySlot;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_SET_PLAYER_INVENTORY, state = "play")]
/// # This packet is buggy and does not seem to work.
pub struct SetPlayerInventorySlot {
    pub slot_index: VarInt,
    pub slot: InventorySlot,
}
