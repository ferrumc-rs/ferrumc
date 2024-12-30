use crate::slot::Slot;
use dashmap::DashMap;
use ferrumc_net::packets::outgoing::set_container_slot::NetworkSlot;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;

#[derive(Debug, Clone)]
pub struct InventoryContents {
    pub contents: DashMap<i32, Slot>,
}

impl InventoryContents {
    pub fn empty() -> Self {
        Self {
            contents: DashMap::new(),
        }
    }

    pub fn set_slot(&mut self, slot_id: i32, slot: Slot) -> &mut Self {
        self.contents.insert(slot_id, slot);
        self
    }

    pub fn get_slot(&self, item: i32) -> Option<Slot> {
        self.contents.get(&item).map(|v| *v)
    }

    pub(crate) fn construct_container_vec(&self, size: usize) -> LengthPrefixedVec<NetworkSlot> {
        let mut vec = LengthPrefixedVec::new(Vec::with_capacity(size));
        for i in 0..(size as i32) {
            let slot = match self.contents.get(&i) {
                Some(data) => data.value().to_network_slot(),
                None => NetworkSlot::empty(),
            };

            vec.data.insert(i as usize, slot);
        }

        vec
    }
}
