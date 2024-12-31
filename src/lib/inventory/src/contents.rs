use crate::slot::Slot;
use dashmap::DashMap;
use ferrumc_net::packets::outgoing::set_container_slot::NetworkSlot;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;

#[derive(Debug, Clone)]
pub struct InventoryContents {
    pub contents: DashMap<i32, Slot>,
}

impl InventoryContents {
    pub fn empty(size: usize) -> Self {
        let mut empty = Self {
            contents: DashMap::with_capacity(size),
        };

        empty.fill(0);
        empty
    }

    pub fn fill(&mut self, item: i32) {
        for i in 0..(self.contents.capacity() as i32) {
            self.contents.insert(i, Slot::new(1, item));
        }
    }

    pub fn set_slot(&mut self, slot_id: i32, slot: Slot) -> &mut Self {
        self.contents.insert(slot_id, slot);
        self
    }

    pub fn get_slot(&self, item: i32) -> Option<Slot> {
        self.contents.get(&item).map(|v| *v)
    }

    pub fn get_size(&self) -> usize {
        self.contents.capacity()
    }

    pub(crate) fn construct_packet_contents(&self) -> LengthPrefixedVec<NetworkSlot> {
        let mut contents = LengthPrefixedVec::new(Vec::with_capacity(self.get_size()));
        self.contents.iter().for_each(|slot| {
            contents.data.push(slot.value().to_network_slot());
        });

        contents
    }

    //to store in chunk metadata: TAG 44: byte
    //to show: starts at slot 0 ALWAYS - > 26/53 smalll/large.
    //other inventories are to be implemented after.
}
