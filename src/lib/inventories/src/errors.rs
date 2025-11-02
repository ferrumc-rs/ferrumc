use thiserror::Error;

/// Inventory Errors
#[derive(Error, Debug)]
pub enum InventoryError {
    /// Error which gets thrown if the inventory is full.
    #[error("Inventory is full")]
    InventoryFull,
    /// Gets thrown when an item doesnt exist in the inventory, but is called.
    #[error("Item not found in inventory")]
    ItemNotFound,
    /// Gets thrown when an invalid slot gets called.
    #[error("Invalid slot index: {0}")]
    InvalidSlotIndex(usize),
    /// Gets thrown outside of the slot range.
    #[error("Outside maximum slot range of {0}: {1}")]
    OutsideMaxSlotRange(usize, usize),
}
