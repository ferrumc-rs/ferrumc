#![feature(more_float_constants)]
#![feature(new_range_api)]

mod biome;
mod biome_chunk;
pub mod block_can_survive;
pub mod blocktag;
mod cache;
mod common;
mod direction;
mod end;
pub mod errors;
mod nether;
mod noise_router;
mod overworld;
mod perlin_noise;
mod pos;
mod random;
use crate::end::end_generator::EndGenerator;
use crate::errors::WorldGenError;
use crate::overworld::overworld_generator::OverworldGenerator;
use crate::pos::BlockPos;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk_format::Chunk;

pub struct ChunkAccess {}

impl ChunkAccess {
    pub fn get_block_state(&self, pos: BlockPos) -> BlockStateId {
        todo!()
    }

    pub fn set_block_state(&mut self, pos: BlockPos, data: BlockStateId) {
        todo!()
    }
    pub fn set_block_state_flags(&mut self, pos: BlockPos, data: BlockStateId, flags: u32) {
        todo!()
    }

    fn get_height(&self, world_surface_wg: HeightmapType, max_x: i32, z: i32) -> i32 {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub enum HeightmapType {
    WorldSurfaceWg,
    MotionBlocking,
    MotionBlockingNoLeaves,
    WorldSurface,
    OceanFloor,
    OceanFloorWg,
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
