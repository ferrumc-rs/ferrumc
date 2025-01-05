use crate::{inventory::InventoryType, slot::Slot};
use ferrumc_net::packets::outgoing::set_container_slot::NetworkSlot;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct InventoryContents {
    pub contents: BTreeMap<i16, Slot>,
    pub size: i16,
}

impl InventoryContents {
    pub fn empty(inventory_type: InventoryType) -> Self {
        let mut empty = Self::new(inventory_type.get_size(), BTreeMap::new());
        empty.fill(Slot::empty());
        empty
    }

    pub fn new(size: i16, contents: BTreeMap<i16, Slot>) -> Self {
        Self { contents, size }
    }

    pub fn fill<S: Into<Slot> + Copy>(&mut self, slot: S) {
        for i in 0..self.size {
            self.contents.insert(i, slot.into());
        }
    }

    pub fn set_slot<S: Into<Slot>>(&mut self, slot_id: i16, slot: S) -> &mut Self {
        self.contents.insert(slot_id, slot.into());
        self
    }

    pub fn get_slot(&self, item: i16) -> Option<Slot> {
        self.contents.get(&item).copied()
    }

    pub(crate) fn construct_packet_contents(&self) -> LengthPrefixedVec<NetworkSlot> {
        let mut contents = vec![];
        self.contents.iter().for_each(|(_, slot)| {
            contents.push(slot.to_network_slot());
        });

        LengthPrefixedVec::new(contents)
    }

    //to store in chunk metadata: TAG 44: byte
    //to show: starts at slot 0 ALWAYS - > 26/53 smalll/large.
    //other inventories are to be implemented after.
}
