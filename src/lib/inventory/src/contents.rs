use dashmap::DashMap;

pub struct InventoryContents {
    pub contents: DashMap<i32, i32>,
}

impl InventoryContents {
    pub fn empty() -> Self {
        Self {
            contents: DashMap::new(),
        }
    }

    pub fn set_slot(&mut self, slot: i32, item: i32) -> &mut Self {
        self.contents.insert(slot, item);
        self
    }

    pub fn get_slot(&self, item: i32) -> Option<i32> {
        self.contents.get(&item).map(|v| *v)
    }
}
