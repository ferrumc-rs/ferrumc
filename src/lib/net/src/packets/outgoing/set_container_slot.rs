use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::{Read, Write};

#[derive(NetEncode, Debug)]
pub struct NetworkSlot {
    pub item_count: VarInt,
    pub item_id: Option<VarInt>,
    pub num_of_components_to_add: Option<VarInt>,
    pub num_of_components_to_remove: Option<VarInt>,
}

impl NetDecode for NetworkSlot {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let item_count = VarInt::decode(reader, opts)?;
        if *item_count == 0 {
            Ok(Self::empty())
        } else {
            Ok(Self::new(item_count.val, *VarInt::decode(reader, opts)?))
        }
    }
}

impl NetworkSlot {
    pub fn new(item_count: i32, item_id: i32) -> Self {
        if item_count == 0 {
            Self {
                item_count: VarInt::new(0),
                item_id: None,
                num_of_components_to_add: None,
                num_of_components_to_remove: None,
            }
        } else {
            Self {
                item_count: VarInt::new(item_count),
                item_id: Some(VarInt::new(item_id)),
                num_of_components_to_add: Some(VarInt::new(0)),
                num_of_components_to_remove: Some(VarInt::new(0)),
            }
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
#[packet(packet_id = 0x15)]
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
