use ferrumc_core::items::inventory_slot::InventorySlot;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
#[packet(packet_id = "container_set_content", state = "play")]
pub struct SetContainerContent {
    pub window_id: VarInt,
    pub state_id: VarInt,
    pub slots: LengthPrefixedVec<InventorySlot>,
    pub carried_item: InventorySlot,
}
