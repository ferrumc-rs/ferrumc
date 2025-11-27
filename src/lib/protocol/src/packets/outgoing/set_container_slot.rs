use ferrumc_core::items::inventory_slot::InventorySlot;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

#[derive(NetEncode)]
#[packet(packet_id = "container_set_slot", state = "play")]
pub struct SetContainerSlot {
    pub window_id: VarInt,
    pub state_id: VarInt,
    pub slot_index: i16,
    pub slot: InventorySlot,
}
