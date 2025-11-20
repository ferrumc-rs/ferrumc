use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemID(pub i32); // Stored as native i32

impl ItemID {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn inner(&self) -> i32 {
        self.0
    }
}

impl Display for ItemID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemID({})", self.0)
    }
}

// Standard converters make life easier
impl From<i32> for ItemID {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<u32> for ItemID {
    fn from(id: u32) -> Self {
        Self(id as i32)
    }
}

impl From<ItemID> for i32 {
    fn from(id: ItemID) -> Self {
        id.0
    }
}
