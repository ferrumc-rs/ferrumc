use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::{length_prefixed_vec::LengthPrefixedVec, var_int::VarInt};
use std::io::Write;

use super::set_container_slot::NetworkSlot;

#[derive(NetEncode)]
#[packet(packet_id = 0x13)]
pub struct SetContainerContentPacket {
    pub window_id: u8,
    pub state_id: VarInt,
    pub slot_data: LengthPrefixedVec<NetworkSlot>,
    pub carried_item: NetworkSlot,
}

impl SetContainerContentPacket {
    pub fn new(
        window_id: u8,
        slot_data: LengthPrefixedVec<NetworkSlot>,
        carried_item: NetworkSlot,
    ) -> Self {
        Self {
            window_id,
            state_id: VarInt::new(0),
            slot_data,
            carried_item,
        }
    }
}