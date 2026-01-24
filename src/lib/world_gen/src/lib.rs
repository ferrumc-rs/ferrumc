mod biomes;
mod caves;
pub mod errors;
mod interp;

use crate::errors::WorldGenError;
use ferrumc_world::{chunk::Chunk, pos::ChunkPos};
use noise::{Clamp, MultiFractal, NoiseFn, OpenSimplex, RidgedMulti};

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
    pub(crate) terrain_layers: Vec<Clamp<f64, OpenSimplex, 2>>,
    pub(crate) caves_layer: RidgedMulti<OpenSimplex>,
}

pub struct WorldGenerator {
    _seed: u64,
    noise_generator: NoiseGenerator,
}

impl NoiseGenerator {
    pub fn new(seed: u64) -> Self {
        let mut terrain_layers = Vec::new();
        for i in 0..4 {
            let open_simplex = OpenSimplex::new((seed + i) as u32);
            let clamp = Clamp::new(open_simplex).set_bounds(-1.0, 1.0);
            terrain_layers.push(clamp);
        }
        Self {
            terrain_layers,
            caves_layer: RidgedMulti::new((seed + 100) as u32)
                .set_frequency(0.01)
                .set_lacunarity(2.5)
                .set_octaves(5)
                .set_persistence(0.8)
                .set_attenuation(0.3),
        }
    }

    pub fn get_noise(&self, x: f64, z: f64) -> f64 {
        let mut noise = 0.0;
        for (c, layer) in self.terrain_layers.iter().enumerate() {
            let scale = 64.0_f64.powi(c as i32 + 1);
            noise += layer.get([x / scale, z / scale]);
        }
        noise / (self.terrain_layers.len() as f64 / 2.0)
    }

    pub fn get_cave_noise(&self, x: f64, y: f64, z: f64) -> f64 {
        self.caves_layer.get([x, y, z])
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
        let mut chunk = biome.generate_chunk(pos, &self.noise_generator)?;
        caves::generate_caves(&mut chunk, pos, &self.noise_generator);
        Ok(chunk)
    }
}
