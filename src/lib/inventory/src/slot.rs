use ferrumc_net::packets::outgoing::set_container_slot::NetworkSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slot {
    pub count: i32,
    pub item: i32,
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
    pub fn to_network_slot(&self) -> NetworkSlot {
        NetworkSlot::new(self.count, self.item)
    }
}
