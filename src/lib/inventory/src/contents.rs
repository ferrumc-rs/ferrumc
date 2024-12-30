use crate::slot::Slot;
use dashmap::DashMap;

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

    //to store in chunk metadata: TAG 44: byte
    //to show: starts at slot 0 ALWAYS - > 26/53 smalll/large.
    //other inventories are to be implemented after.
}
