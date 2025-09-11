#![feature(maybe_uninit_array_assume_init)]

mod aquifer;
mod biome;
mod biome_chunk;
mod carver;
pub mod errors;
mod noise_biome_parameters;
mod noise_router;
mod overworld;
mod perlin_noise;
mod pos;
mod random;
mod surface;
use crate::biome_chunk::{BiomeChunk, BiomeNoise, NoisePoint};
use crate::carver::{CanyonCarver, Caver};
use crate::pos::{ChunkHeight, ChunkPos};
use crate::surface::Surface;
use crate::{biome::Biome, errors::WorldGenError};
use ferrumc_world::{chunk_format::Chunk, vanilla_chunk_format::BlockData};

pub struct SurfaceRule {} //TODO
impl SurfaceRule {
    fn try_apply(
        &self,
        biome: Biome,
        depth: i32,
        depth_from_stone: i32,
        fluid_level: Option<i32>,
        y: bevy_math::IVec3,
    ) -> Option<BlockData> {
        todo!()
    }
}

//TODO
pub struct DensityFunction;
impl DensityFunction {
    pub fn compute<T: Into<(i32, i32, i32)>>(&self, _pos: T) -> f64 {
        todo!()
    }
} //TODO

pub struct WorldGenerator {
    seed: u64,
    chunk_height: ChunkHeight,
    biome_noise: BiomeNoise,
    biomes: Vec<(NoisePoint, Biome)>,
    surface: Surface,
    cave_carver: Caver,
    extra_cave_carver: Caver,
    canyon_carver: CanyonCarver,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        todo!()
        // Self { _seed: seed }
    }

    fn generate_biomes(&self, pos: ChunkPos) -> BiomeChunk {
        BiomeChunk::generate(&self.biome_noise, &self.biomes, pos, self.chunk_height)
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        todo!()
    }
}
