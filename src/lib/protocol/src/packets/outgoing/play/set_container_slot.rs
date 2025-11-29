use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use ferrumc_core::items::inventory_slot::InventorySlot;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_CONTAINER_SET_SLOT, state = "play")]
pub struct SetContainerSlot {
    pub window_id: VarInt,
    pub state_id: VarInt,
    pub slot_index: i16,
    pub slot: InventorySlot,
}
