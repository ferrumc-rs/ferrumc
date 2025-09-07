#![feature(maybe_uninit_array_assume_init)]

mod aquifier;
mod biome;
mod biomes;
pub mod errors;
mod noise_biome_parameters;
mod noise_router;
mod ore_veins;
mod perlin_noise;
mod random;
mod surface;
use crate::{aquifier::FluidPicker, errors::WorldGenError};
use ferrumc_world::{block_id::BlockId, chunk_format::Chunk, vanilla_chunk_format::BlockData};
use noise::{Clamp, NoiseFn, OpenSimplex};

pub struct NoiseGeneratorSettings {
    noise_settings: NoiseSettings,
    default_block: BlockId,
    noise_router: NoiseRouter,
    sea_level: FluidPicker,
    aquifers_enabled: bool,
    ore_veins_enabled: bool,
    use_legacy_random_source: bool,
    rule_source: SurfaceRule,
}

pub struct SurfaceRule {} //TODO
impl SurfaceRule {
    fn try_apply(
        &self,
        depth: i32,
        depth_from_stone: i32,
        fluid_level: Option<i32>,
        y: bevy_math::IVec3,
    ) -> Option<BlockData> {
        todo!()
    }
}

pub struct NoiseSettings {
    min_y: i32,
    height: u32,
    //cell width
    noise_size_horizontal: i32,
    //cell height
    noise_size_vertical: i32,
}
//TODO
pub struct DensityFunction;
impl DensityFunction {
    pub fn compute<T: Into<(i32, i32, i32)>>(&self, _pos: T) -> f64 {
        todo!()
    }
} //TODO
pub struct NoiseRouter {
    barrier_noise: DensityFunction,
    fluid_level_floodedness_noise: DensityFunction,
    fluid_level_spread_noise: DensityFunction,
    lava_noise: DensityFunction,
    temperature: DensityFunction,
    vegetation: DensityFunction,
    continents: DensityFunction,
    erosion: DensityFunction,
    depth: DensityFunction,
    ridges: DensityFunction,
    initial_density_without_jaggedness: DensityFunction,
    final_density: DensityFunction,
    vein_toggle: DensityFunction,
    vein_ridged: DensityFunction,
    vein_gap: DensityFunction,
}

pub fn generate_chunk(settings: &NoiseGeneratorSettings, x: i32, y: i32) -> Chunk {
    todo!()
}

/// Trait for generating a biome
///
/// Should be implemented for each biome's generator
pub(crate) trait BiomeGenerator {
    fn _biome_id(&self) -> u8;
    fn _biome_name(&self) -> String;
    fn generate_chunk(&self, pos: ChunkPos, noise: &NoiseGenerator)
    -> Result<Chunk, WorldGenError>;
}

pub(crate) struct NoiseGenerator {
    pub(crate) layers: Vec<Clamp<f64, OpenSimplex, 2>>,
}

pub struct WorldGenerator {
    _seed: u64,
    noise_generator: NoiseGenerator,
}

impl NoiseGenerator {
    pub fn new(seed: u64) -> Self {
        let mut layers = Vec::new();
        for i in 0..4 {
            let open_simplex = OpenSimplex::new((seed + i) as u32);
            let clamp = Clamp::new(open_simplex).set_bounds(-1.0, 1.0);
            layers.push(clamp);
        }
        Self { layers }
    }

    pub fn get_noise(&self, x: f64, z: f64) -> f64 {
        let mut noise = 0.0;
        for (c, layer) in self.layers.iter().enumerate() {
            let scale = 64.0_f64.powi(c as i32 + 1);
            noise += layer.get([x / scale, z / scale]);
        }
        noise / (self.layers.len() as f64 / 2.0)
    }
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            _seed: seed,
            noise_generator: NoiseGenerator::new(seed),
        }
    }

    fn get_biome(&self, _pos: ChunkPos) -> Box<dyn BiomeGenerator> {
        // Implement biome selection here
        Box::new(biomes::plains::PlainsBiome)
    }

    pub fn generate_chunk(&self, pos: ChunkPos) -> Result<Chunk, WorldGenError> {
        let biome = self.get_biome(pos);
        biome.generate_chunk(pos, &self.noise_generator)
    }
}
