use crate::codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use crate::codec::net_types::var_int::VarInt;
use crate::ids;
use ferrumc_core::items::inventory_slot::InventorySlot;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_CONTAINER_SET_CONTENT, state = "play")]
pub struct SetContainerContent {
    pub window_id: VarInt,
    pub state_id: VarInt,
    pub slots: LengthPrefixedVec<InventorySlot>,
    pub carried_item: InventorySlot,
}
