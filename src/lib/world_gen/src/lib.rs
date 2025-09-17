#![feature(maybe_uninit_array_assume_init)]

mod biome;
mod biome_chunk;
mod common;
mod end;
pub mod errors;
mod nether;
mod noise_router;
mod overworld;
mod perlin_noise;
mod pos;
mod random;
use crate::pos::BlockPos;
use crate::{errors::WorldGenError, overworld::overworld_generator::OverworldGenerator};
use ferrumc_world::{chunk_format::Chunk, vanilla_chunk_format::BlockData};

pub struct ChunkAccess {}

impl ChunkAccess {
    pub fn get_block_state(&self, pos: BlockPos) -> BlockData {
        todo!()
    }

    pub fn set_block_state(&mut self, pos: BlockPos, data: BlockData) {
        todo!()
    }

    fn get_height(&self, world_surface_wg: HeightmapType, max_x: i32, z: i32) -> i32 {
        todo!()
    }
}
pub enum HeightmapType {
    WorldSurfaceWg,
}
pub struct WorldGenerator {
    generator: OverworldGenerator,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            generator: OverworldGenerator::new(seed),
        }
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        self.generator.generate_chunk(x, z)
    }
}
