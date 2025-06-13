mod biomes;
mod carving;
pub mod errors;
mod noise;

use crate::carving::erosion::get_erosion_noise;
use crate::carving::initial_height::get_initial_height_noise;
use crate::errors::WorldGenError;
use crate::noise::NoiseGenerator;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::vanilla_chunk_format::BlockData;

pub static MAX_GENERATED_HEIGHT: i16 = 192;

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
    humidity_noise: NoiseGenerator,
    temperature_noise: NoiseGenerator,
    height_noise: NoiseGenerator,
    erosion_noise: NoiseGenerator,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        let humidity_noise = NoiseGenerator::new(seed, 0.0, 1.0, 0.01, 4);
        let temperature_noise = NoiseGenerator::new(seed + 1, 0.0, 1.0, 0.01, 4);
        let height_noise = get_initial_height_noise(seed + 2);
        let erosion_noise = get_erosion_noise(seed + 3);

        WorldGenerator {
            _seed: seed,
            humidity_noise,
            temperature_noise,
            height_noise,
            erosion_noise,
        }
    }

    fn get_biome(&self, x: i32, z: i32) -> Box<dyn BiomeGenerator> {
        // Implement biome selection here
        let humidity = self.humidity_noise.get(x as f32, z as f32);
        let temperature = self.temperature_noise.get(x as f32, z as f32);
        let height = self.height_noise.get(x as f32, z as f32);
        if temperature < 0.3 {
            return Box::new(biomes::ocean::OceanBiome::default());
        }
        Box::new(biomes::plains::PlainsBiome::default())
    }

    pub fn generate_chunk(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new(x, z, "overworld".to_string());
        // Only fill the first 12 sections with stone
        chunk.sections.iter_mut().for_each(|sec| {
            if sec.y >= (MAX_GENERATED_HEIGHT / 16) as i8 {
                return;
            }
            sec.fill(BlockData {
                name: "minecraft:stone".to_string(),
                properties: None,
            })
                .unwrap()
        });
        self.carve_chunk(&mut chunk)?;
        // for x in 0..16u8 {
        //     for z in 0..16u8 {
        //         let y = chunk.real_heightmap[x as usize][z as usize];
        //         let biome = self.get_biome(i32::from(x), i32::from(z));
        //         biome.decorate(&mut chunk, x, z)?;
        //     }
        // }
        Ok(chunk)
    }
}
