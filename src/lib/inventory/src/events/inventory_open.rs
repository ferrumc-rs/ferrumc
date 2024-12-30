use ferrumc_macros::Event;
use crate::inventory::Inventory;

#[derive(Event, Debug)]
pub struct OpenInventoryEvent {
    pub conn_id: usize,
    pub inventory: Option<Inventory>,
}

impl OpenInventoryEvent {
    pub fn new(conn_id: usize) -> Self {
        Self { conn_id, inventory: None }
    }

    pub fn inventory(mut self, inventory: Inventory) -> Self {
        self.inventory = Some(inventory);
        self
    }
}