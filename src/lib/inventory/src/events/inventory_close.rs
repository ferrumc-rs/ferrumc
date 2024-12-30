use ferrumc_macros::Event;

#[derive(Event, Debug)]
pub struct CloseInventoryEvent {
    pub conn_id: usize,
    pub inventory_id: Option<i32>,
}

impl CloseInventoryEvent {
    pub fn new(conn_id: usize) -> Self {
        Self {
            conn_id,
            inventory_id: None,
        }
    }

    pub fn inventory_id(mut self, inventory_id: i32) -> Self {
        self.inventory_id = Some(inventory_id);
        self
    }
}
