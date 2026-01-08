use crate::vanilla_chunk_format::BlockData;
use ahash::RandomState;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;
use ferrumc_net_codec::net_types::var_int::VarInt;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;
use std::process::exit;
use tracing::error;

// The number of block entries in the mappings file
// Go to the .etc/blockstates.json file, see what the last ID is, and add 1 to it.
const BLOCK_ENTRIES: usize = 27914;

const BLOCKSFILE: &str = include_str!("../../../../assets/data/blockstates.json");

lazy_static! {
    pub static ref ID2BLOCK: Vec<BlockData> = {
        let string_keys: HashMap<String, BlockData, RandomState> =
            serde_json::from_str(BLOCKSFILE).unwrap();
        if string_keys.len() != BLOCK_ENTRIES {
            // Edit this number if the block mappings file changes
            error!("Block mappings file is not the correct length");
            error!("Expected {} entries, found {}", BLOCK_ENTRIES, string_keys.len());
            exit(1);
        }
        let mut id2block = Vec::with_capacity(BLOCK_ENTRIES);
        for _ in 0..BLOCK_ENTRIES {
            id2block.push(BlockData::default());
        }
        string_keys
            .iter()
            .map(|(k, v)| (k.parse::<i32>().unwrap(), v.clone()))
            .for_each(|(k, v)| id2block[k as usize] = v);
        id2block
    };
    pub static ref BLOCK2ID: HashMap<BlockData, i32, RandomState> = ID2BLOCK
        .iter()
        .enumerate()
        .map(|(k, v)| (v.clone(), k as i32))
        .collect();
}

/// An ID for a block, and it's state in the world. Use this over `BlockData` unless you need to
/// modify or read the block's name/properties directly.
///
/// This should be used over `BlockData` in most cases, as it's much more efficient to store and pass around.
/// You can also generate a block's id at runtime with the [ferrumc_macros::block!] macro.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Encode, Decode, DeepSizeOf)]
pub struct BlockStateId(u32);

impl BlockStateId {
    /// Do NOT use this by yourself. Instead use the block macro `block!("stone")` the item to block
    /// map
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// Given a BlockData, return a BlockStateId. Does not clone, should be quite fast.
    pub fn from_block_data(block_data: &BlockData) -> Self {
        let id = BLOCK2ID
            .get(block_data)
            .expect("Block data not found in block mappings file");
        BlockStateId(*id as u32)
    }

    /// Given a block state ID, return a BlockData. Will clone, so don't use in hot loops.
    /// If the ID is not found, returns None.
    pub fn to_block_data(&self) -> Option<BlockData> {
        ID2BLOCK.get(self.0 as usize).cloned()
    }

    pub fn from_varint(var_int: VarInt) -> Self {
        BlockStateId(var_int.0 as u32)
    }

    pub fn to_varint(&self) -> VarInt {
        VarInt(self.0 as i32)
    }

    /// Do Not use this by yourself. This is only useful for apis that use this as an index or key
    /// to get additionally information about this state.
    pub fn raw(&self) -> u32 {
        self.0
    }
}

impl Display for BlockStateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(block_data) = self.to_block_data() {
            write!(f, "BlockStateId({}: {:?})", self.0, block_data)
        } else {
            write!(f, "BlockStateId({}: Unknown)", self.0)
        }
    }
}

impl BlockData {
    /// Converts a BlockData to a BlockStateId. Will panic if the ID is not found.
    pub fn to_block_state_id(&self) -> BlockStateId {
        BlockStateId::from_block_data(self)
    }

    /// Converts a BlockStateId to a BlockData. Will panic if the ID is not found.
    pub fn from_block_state_id(block_state_id: BlockStateId) -> BlockData {
        block_state_id
            .to_block_data()
            .expect("Block state ID not found in block mappings file")
    }
}
impl From<BlockData> for BlockStateId {
    fn from(block_data: BlockData) -> Self {
        BlockStateId::from_block_data(&block_data)
    }
}
impl From<BlockStateId> for BlockData {
    /// Converts a BlockStateId to a BlockData. Will panic if the ID is not found.
    fn from(block_state_id: BlockStateId) -> Self {
        block_state_id
            .to_block_data()
            .expect("Block state ID not found in block mappings file")
    }
}

impl From<VarInt> for BlockStateId {
    /// Converts a VarInt to a BlockStateId. Probably a no-op, but included for completeness.
    fn from(var_int: VarInt) -> Self {
        Self(var_int.0 as u32)
    }
}

impl From<BlockStateId> for VarInt {
    /// Converts a BlockStateId to a VarInt. Probably a no-op, but included for completeness.
    fn from(block_state_id: BlockStateId) -> Self {
        VarInt(block_state_id.0 as i32)
    }
}

impl Default for BlockStateId {
    /// Returns a BlockStateId with ID 0, which is air.
    fn default() -> Self {
        Self(0)
    }
}
