use crate::vanilla_chunk_format::BlockData;
use ahash::RandomState;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;
use ferrumc_net_codec::net_types::var_int::VarInt;
use intmap::IntKey;
use lazy_static::lazy_static;
use std::collections::HashMap;
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Encode, Decode, DeepSizeOf)]
pub struct BlockId(pub u32);

impl IntKey for BlockId {
    type Int = u32;
    const PRIME: Self::Int = u32::PRIME;

    fn into_int(self) -> Self::Int {
        self.0
    }
}

impl BlockId {
    /// Given a BlockData, return a BlockId. Does not clone, should be quite fast.
    pub fn from_block_data(block_data: &BlockData) -> Self {
        let id = BLOCK2ID
            .get(block_data)
            .expect("Block data not found in block mappings file");
        BlockId(*id as u32)
    }

    /// Given a block ID, return a BlockData. Will clone, so don't use in hot loops.
    /// If the ID is not found, returns None.
    pub fn to_block_data(&self) -> Option<BlockData> {
        ID2BLOCK.get(self.0 as usize).cloned()
    }

    pub fn from_varint(var_int: VarInt) -> Self {
        BlockId(var_int.0 as u32)
    }

    pub fn to_varint(&self) -> VarInt {
        VarInt(self.0 as i32)
    }
}

impl BlockData {
    /// Converts a BlockData to a BlockId. Will panic if the ID is not found.
    pub fn to_block_id(&self) -> BlockId {
        BlockId::from_block_data(self)
    }

    /// Converts a BlockId to a BlockData. Will panic if the ID is not found.
    pub fn from_block_id(block_id: BlockId) -> BlockData {
        block_id
            .to_block_data()
            .expect("Block ID not found in block mappings file")
    }
}
impl From<BlockData> for BlockId {
    fn from(block_data: BlockData) -> Self {
        BlockId::from_block_data(&block_data)
    }
}
impl From<BlockId> for BlockData {
    /// Converts a BlockId to a BlockData. Will panic if the ID is not found.
    fn from(block_id: BlockId) -> Self {
        block_id
            .to_block_data()
            .expect("Block ID not found in block mappings file")
    }
}

impl From<VarInt> for BlockId {
    /// Converts a VarInt to a BlockId. Probably a no-op, but included for completeness.
    fn from(var_int: VarInt) -> Self {
        Self(var_int.0 as u32)
    }
}

impl From<BlockId> for VarInt {
    /// Converts a BlockId to a VarInt. Probably a no-op, but included for completeness.
    fn from(block_id: BlockId) -> Self {
        VarInt(block_id.0 as i32)
    }
}

impl From<i32> for BlockId {
    /// Converts an i32 to a BlockId. Will panic if the ID is negative.
    fn from(id: i32) -> Self {
        if id < 0 {
            panic!("Block ID cannot be negative");
        }
        Self(id as u32)
    }
}

impl From<BlockId> for i32 {
    /// Converts a BlockId to an i32. Will panic if the ID is greater than i32::MAX.
    fn from(block_id: BlockId) -> Self {
        if block_id.0 > i32::MAX as u32 {
            panic!("Block ID cannot be greater than i32::MAX");
        }
        block_id.0 as i32
    }
}

impl Default for BlockId {
    /// Returns a BlockId with ID 0, which is air.
    fn default() -> Self {
        Self(0)
    }
}
