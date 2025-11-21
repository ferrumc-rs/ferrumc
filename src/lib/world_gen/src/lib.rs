mod biomes;
mod carving;
pub mod errors;
mod noise;

use crate::carving::erosion::get_erosion_noise;
use crate::carving::initial_height::get_initial_height_noise;
use crate::errors::WorldGenError;
use crate::noise::NoiseGenerator;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk_format::Chunk;

pub static MAX_GENERATED_HEIGHT: i16 = 192;
pub static BASELINE_HEIGHT: i16 = 82;

/// Trait for generating a biome
///
/// Should be implemented for each biome's generator
pub(crate) trait BiomeGenerator {
    fn _biome_id(&self) -> u8;
    fn _biome_name(&self) -> String;
    fn decorate(&self, chunk: &mut Chunk, x: u8, z: u8) -> Result<(), WorldGenError>;
    fn new(seed: u64) -> Self
    where
        Self: Sized;
}

pub struct WorldGenerator {
    seed: u64,
    _humidity_noise: NoiseGenerator,
    _temperature_noise: NoiseGenerator,
    height_noise: NoiseGenerator,
    erosion_noise: NoiseGenerator,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        let humidity_noise = NoiseGenerator::new(seed, 0.01, 4, None);
        let temperature_noise = NoiseGenerator::new(seed + 1, 0.01, 4, None);
        let height_noise = get_initial_height_noise(seed + 2);
        let erosion_noise = get_erosion_noise(seed + 3);

        WorldGenerator {
            seed,
            _humidity_noise: humidity_noise,
            _temperature_noise: temperature_noise,
            height_noise,
            erosion_noise,
        }
    }

    fn get_biome(&self, chunk: &Chunk, x: usize, z: usize) -> Box<dyn BiomeGenerator> {
        // Implement biome selection here
        let _humidity = f32::from_bits(chunk.noises.humidity_noise[x][z]);
        let _temperature = f32::from_bits(chunk.noises.temperature_noise[x][z]);
        let erosion = f32::from_bits(chunk.noises.erosion_noise[x][z]);
        let _height = f32::from_bits(chunk.noises.height_noise[x][z]);
        let real_height = chunk.real_heightmap[x][z];
        if real_height < 50 {
            return Box::new(biomes::ocean::OceanBiome::new(self.seed));
        }
        if erosion <= 0.3 {
            return Box::new(biomes::mountain::MountainBiome::new(self.seed));
        }
        Box::new(biomes::plains::PlainsBiome::new(self.seed))
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new(x, z, "overworld".to_string());
        // Only fill the first 12 sections with stone
        chunk.sections.iter_mut().for_each(|sec| {
            if sec.y >= (f32::from(MAX_GENERATED_HEIGHT) / 16.0).floor() as i8 {
                return;
            }
            sec.fill(block!("stone")).unwrap()
        });
        self.carve_chunk(&mut chunk)?;
        for x in 0..16u8 {
            for z in 0..16u8 {
                let biome = self.get_biome(&chunk, x as usize, z as usize);
                biome.decorate(&mut chunk, x, z)?;
            }
        }
        Ok(chunk)
    }
}
