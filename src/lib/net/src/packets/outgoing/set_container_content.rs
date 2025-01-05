use super::set_container_slot::NetworkSlot;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::{length_prefixed_vec::LengthPrefixedVec, var_int::VarInt};
use std::io::Write;

#[derive(NetEncode, Debug)]
#[packet(packet_id = "container_set_content", state_id = "play")]
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
