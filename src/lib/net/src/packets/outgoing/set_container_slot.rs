use ferrumc_macros::{packet, NetDecode, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;

#[derive(NetDecode, NetEncode, Debug, Clone, Copy)]
pub struct NetworkSlot {
    pub item_count: VarInt,
    #[net(optional_trigger = *item_count > 0)]
    pub item_id: Option<VarInt>,
    #[net(optional_trigger = *item_count > 0)]
    pub num_of_components_to_add: Option<VarInt>,
    #[net(optional_trigger = *item_count > 0)]
    pub num_of_components_to_remove: Option<VarInt>,
}

impl NetworkSlot {
    pub fn new(item_count: i32, item_id: i32) -> Self {
        let components = if item_count == 0 {
            None
        } else {
            Some(VarInt::new(0))
        };
        Self {
            item_count: VarInt::new(item_count),
            item_id: if item_count == 0 {
                None
            } else {
                Some(VarInt::new(item_id))
            },
            num_of_components_to_add: components,
            num_of_components_to_remove: components,
        }
    }

    pub fn empty() -> Self {
        Self::new(0, 0)
    }

    pub fn item_id(&mut self, item_id: VarInt) -> &mut Self {
        self.item_id = Some(item_id);
        self
    }
}

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
