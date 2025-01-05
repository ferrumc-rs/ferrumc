use std::io::Read;

use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::{
    decode::{NetDecode, NetDecodeOpts, NetDecodeResult},
    net_types::{length_prefixed_vec::LengthPrefixedVec, var_int::VarInt},
};

#[derive(NetEncode, Debug, Clone, Copy)]
pub enum SlotComponent {
    MaxStackSize { max_stack_size: VarInt },
}

#[derive(Debug, Clone, Copy)]
pub struct NetworkSlot {
    pub item_count: VarInt,
    pub item_id: Option<VarInt>,
    pub components_to_add: Option<LengthPrefixedVec<SlotComponent>>,
    pub components_to_remove: Option<LengthPrefixedVec<VarInt>>,
}

impl NetworkSlot {
    pub fn new(item_count: i32, item_id: i32) -> Self {
        Self::with_components(item_count, item_id, vec![])
    }

    pub fn with_components(item_count: i32, item_id: i32, components: Vec<SlotComponent>) -> Self {
        Self {
            item_count: VarInt::new(item_count),
            item_id: if item_count == 0 {
                None
            } else {
                Some(VarInt::new(item_id))
            },
            components_to_add: if item_count == 0 {
                None
            } else {
                Some(LengthPrefixedVec::new(components))
            },
            components_to_remove: if item_count == 0 {
                None
            } else {
                Some(LengthPrefixedVec::default())
            },
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
