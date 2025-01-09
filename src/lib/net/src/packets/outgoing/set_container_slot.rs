use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

use crate::slot::NetworkSlot;

#[derive(NetEncode)]
#[packet(packet_id = "container_set_slot", state_id = "play")]
pub struct SetContainerSlotPacket {
    pub window_id: VarInt,
    pub state_id: VarInt,
    pub slot: i16,
    pub slot_data: NetworkSlot,
}

impl SetContainerSlotPacket {
    pub fn new(window_id: VarInt, slot: i16, slot_data: NetworkSlot) -> Self {
        Self {
            window_id,
            state_id: VarInt::new(0),
            slot,
            slot_data,
        }
    }
}
