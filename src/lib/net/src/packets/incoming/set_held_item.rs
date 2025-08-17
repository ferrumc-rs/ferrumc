use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode)]
#[packet(packet_id = "set_carried_item", state = "play")]
pub struct SetHeldItem {
    pub slot_index: i16,
}
