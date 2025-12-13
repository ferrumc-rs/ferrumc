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
use crate::pos::{ChunkHeight, ChunkPos};
use crate::{aquifer::FluidPicker, biome::Biome, errors::WorldGenError};
use ferrumc_world::{block_id::BlockId, chunk_format::Chunk, vanilla_chunk_format::BlockData};

pub struct NoiseGeneratorSettings {
    noise_size_vertical: u32,
    default_block: BlockId,
    noise_router: Option<AquiferNoise>,
    vein_noise: Option<VeinNoise>,
    sea_level: FluidPicker,
    use_legacy_random_source: bool,
    initial_density_without_jaggedness: DensityFunction,
    final_density: DensityFunction,
    rule_source: SurfaceRule,
    biome_noise: BiomeNoise,
    chunk_height: ChunkHeight,
}

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
pub struct AquiferNoise {
    barrier_noise: DensityFunction,
    fluid_level_floodedness_noise: DensityFunction,
    fluid_level_spread_noise: DensityFunction,
    lava_noise: DensityFunction,
    vein_toggle: DensityFunction,
    vein_ridged: DensityFunction,
    vein_gap: DensityFunction,
}

pub struct VeinNoise {
    vein_toggle: DensityFunction,
    vein_ridged: DensityFunction,
    vein_gap: DensityFunction,
}

pub struct WorldGenerator {
    _seed: u64,
    chunk_height: ChunkHeight,
    biome_noise: BiomeNoise,
    biomes: Vec<(NoisePoint, Biome)>,
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
