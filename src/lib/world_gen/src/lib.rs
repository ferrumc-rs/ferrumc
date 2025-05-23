mod biomes;
pub mod errors;
mod height_gen;

use crate::errors::WorldGenError;
use ferrumc_world::chunk_format::Chunk;
use noise::{Clamp, NoiseFn, OpenSimplex};
use splines::{Interpolation, Key, Spline};

/// Trait for generating a biome
///
/// Should be implemented for each biome's generator
pub(crate) trait BiomeGenerator {
    fn _biome_id(&self) -> u8;
    fn _biome_name(&self) -> String;
    fn decorate(&self, chunk: &mut Chunk, x: u8, z: u8) -> Result<(), WorldGenError>;
}

pub struct WorldGenerator {
    _seed: u64,
    noise_layers: Vec<Clamp<f64, OpenSimplex, 2>>,
    spline: Spline<f64, f64>,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        let noise_layers = (1..=4)
            .map(|i| {
                let noise = OpenSimplex::new(seed as u32 * i);
                Clamp::new(noise).set_lower_bound(-1.0).set_upper_bound(1.0)
            })
            .collect::<Vec<_>>();
        let start = Key::new(-1.0, -1.0, Interpolation::Linear);
        let sea_level = Key::new(-0.8, -0.4, Interpolation::Linear);
        let plains_base = Key::new(0.0, 0.0, Interpolation::Linear);
        let plains_top = Key::new(0.4, 0.4, Interpolation::Linear);
        let mountain = Key::new(0.5, 0.8, Interpolation::Linear);
        let end = Key::new(1.0, 1.0, Interpolation::Linear);
        let spline = Spline::from_vec(vec![start, sea_level, plains_base, plains_top, mountain, end]);
        Self {
            _seed: seed,
            noise_layers,
            spline,
        }
    }

    fn get_biome(&self, y: i16) -> Box<dyn BiomeGenerator> {
        // Implement biome selection here
        if y > 20 {
            Box::new(biomes::plains::PlainsBiome::default())
        } else {
            Box::new(biomes::ocean::OceanBiome::default())
        }
    }

    fn get_noise(&self, x: i64, z: i64) -> f64 {
        let mut amplitude = 1.0;
        let mut frequency = 0.005;
        let mut noise = 0.0;
        for layer in &self.noise_layers {
            noise += layer.get([x as f64 * frequency, z as f64 * frequency]) * amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }
        noise
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        let mut chunk = self.height_gen(x, z)?;
        for x in 0..16u8 {
            for z in 0..16u8 {
                let y = chunk.real_heightmap[x as usize][z as usize];
                let biome = self.get_biome(y);
                biome.decorate(&mut chunk, x, z)?;
            }
        }
        Ok(chunk)
    }
}
