use super::item_id::ItemID;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InventorySlot {
    pub count: i32,
    pub item_id: Option<ItemID>,
    // NBT/Components can be raw bytes or a specific struct if you parse it
    // For pure data, keeping it generic or as raw bytes is often safest
    // until you need to interact with it.
    pub nbt: Option<Vec<u8>>,
}

impl InventorySlot {
    pub fn is_empty(&self) -> bool {
        self.count <= 0 || self.item_id.is_none()
    }
}

impl fmt::Display for InventorySlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.item_id {
            Some(id) => write!(f, "{}x {}", self.count, id),
            None => write!(f, "Empty"),
        }
    }
}
