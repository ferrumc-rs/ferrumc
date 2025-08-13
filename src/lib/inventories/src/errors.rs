use thiserror::Error;

#[derive(Error, Debug)]
pub enum InventoryError {
    #[error("Inventory is full")]
    InventoryFull,
    #[error("Item not found in inventory")]
    ItemNotFound,
    #[error("Invalid slot index: {0}")]
    InvalidSlotIndex(usize),
    #[error("Outside maximum slot range of {0}: {1}")]
    OutsideMaxSlotRange(usize, usize)
}