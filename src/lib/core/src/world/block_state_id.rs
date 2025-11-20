use serde::{Deserialize, Serialize};
use std::fmt;

// Note: I added these derives so `BlockStateId` can be saved/loaded
// directly inside a Chunk without extra conversion logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "deepsize", derive(deepsize::DeepSizeOf))]
pub struct BlockStateId(pub u32);

impl BlockStateId {
    /// Creates a new BlockStateId.
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the inner u32 value.
    pub fn inner(&self) -> u32 {
        self.0
    }
}

// --- Standard Converters ---

impl From<u32> for BlockStateId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl From<BlockStateId> for u32 {
    fn from(bs: BlockStateId) -> Self {
        bs.0
    }
}

impl From<i32> for BlockStateId {
    fn from(id: i32) -> Self {
        Self(id as u32)
    }
}

impl From<BlockStateId> for i32 {
    fn from(bs: BlockStateId) -> Self {
        bs.0 as i32
    }
}

impl fmt::Display for BlockStateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlockStateId({})", self.0)
    }
}
