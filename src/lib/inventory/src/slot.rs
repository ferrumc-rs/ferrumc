use ferrumc_net::slot::NetworkSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slot {
    pub count: i32,
    pub item: i32,
}

impl Default for Slot {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<i32> for Slot {
    fn from(value: i32) -> Self {
        Self::with_item(value)
    }
}

impl From<(i32, i32)> for Slot {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Slot {
    pub fn new(count: i32, item: i32) -> Self {
        Self { count, item }
    }

    pub fn with_item(item: i32) -> Self {
        Self::new(1, item)
    }

    pub fn empty() -> Self {
        Self::new(0, 0)
    }

    pub fn from_network_slot(slot: &NetworkSlot) -> Self {
        match slot.item_id {
            Some(item) => Self::new(*slot.item_count, *item),
            None => Self::empty(),
        }
    }

    pub fn to_network_slot(&self) -> NetworkSlot {
        NetworkSlot::new(self.count, self.item)
    }
}
