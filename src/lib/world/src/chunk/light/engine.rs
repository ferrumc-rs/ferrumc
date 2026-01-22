use crate::chunk::light::sky_light::SkyLightEngine;
use crate::{
    block_state_id::{BlockStateId, ID2BLOCK},
    pos::BlockPos,
    vanilla_chunk_format::BlockData,
};
use bevy_math::IVec3;
use ferrumc_macros::block;
use std::cmp::min;
use std::str::FromStr;
use thiserror::Error;

pub(crate) const PROPAGATION_DIRECTIONS: [BlockPos; 6] = [
    BlockPos::of(1, 0, 0),
    BlockPos::of(-1, 0, 0),
    BlockPos::of(0, 1, 0),
    BlockPos::of(0, -1, 0),
    BlockPos::of(0, 0, 1),
    BlockPos::of(0, 0, -1),
];

#[derive(Clone)]
pub struct ChunkLightingEngine {
    pub sky: SkyLightEngine,
}

impl ChunkLightingEngine {
    pub fn empty() -> Self {
        Self {
            sky: SkyLightEngine::new(-64, 320),
        }
    }

    pub fn new(min_world_y: i32, max_world_y: i32) -> Self {
        Self {
            sky: SkyLightEngine::new(min_world_y, max_world_y),
        }
    }
}

pub trait LightEngine {
    fn opacity(id: BlockStateId) -> u8 {
        // TODO: uh, make this actually work, we too lazy to create a actual block system 😂
        match id.raw() {
            0 => 0,
            _ => 15,
        }
    }

    fn fill(&mut self, level: u8);
}

pub type LightResult<T> = Result<T, LightEngineError>;

#[derive(Debug, Error)]
pub enum LightEngineError {
    #[error("Index {0} is out of bounds of lighting array.")]
    OutOfBounds(u8),

    #[error("Heightmap returned `None`. In {0}")]
    UnknownHeightmap(String),

    #[error("Chunk Section `{0}` is out of bounds, or doesn't exist (chunk: {1}, {2})")]
    UnknownChunkSection(i16, i32, i32),
}

#[derive(Clone, Copy)]
pub struct LightNode {
    pub pos: BlockPos,
    pub level: u8,
}

impl LightNode {
    pub fn new(pos: BlockPos, level: u8) -> Self {
        Self { pos, level }
    }
}
