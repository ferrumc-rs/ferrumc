#![feature(maybe_uninit_array_assume_init)]

mod biome;
mod biome_chunk;
mod common;
mod end;
pub mod errors;
mod nether;
mod noise_biome_parameters;
mod noise_router;
mod overworld;
mod perlin_noise;
mod pos;
mod random;
use crate::errors::WorldGenError;
use crate::pos::BlockPos;
use ferrumc_world::{chunk_format::Chunk, vanilla_chunk_format::BlockData};

//TODO
pub struct DensityFunction;
impl DensityFunction {
    pub fn compute<T: Into<(i32, i32, i32)>>(&self, _pos: T) -> f64 {
        todo!()
    }
} //TODO

pub struct ChunkAccess {}

impl ChunkAccess {
    pub fn get_block_state(&self, pos: BlockPos) -> BlockData {
        todo!()
    }

    pub fn set_block_state(&mut self, pos: BlockPos, data: BlockData) {
        todo!()
    }
}
pub struct WorldGenerator {
    seed: u64,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        todo!()
        // Self { _seed: seed }
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        todo!()
    }
}
