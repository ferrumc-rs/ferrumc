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
    humidity_noise: NoiseGenerator,
    temperature_noise: NoiseGenerator,
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
            humidity_noise,
            temperature_noise,
            height_noise,
            erosion_noise,
        }
    }

    fn get_biome(&self, chunk: &Chunk, x: usize, z: usize) -> Box<dyn BiomeGenerator> {
        // Implement biome selection here
        let humidity = f32::from_bits(chunk.noises.humidity_noise[x][z]);
        let temperature = f32::from_bits(chunk.noises.temperature_noise[x][z]);
        let erosion = f32::from_bits(chunk.noises.erosion_noise[x][z]);
        let height = f32::from_bits(chunk.noises.height_noise[x][z]);
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
            sec.fill(BlockData {
                name: "minecraft:stone".to_string(),
                properties: None,
            })
                .unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;
    use dashmap::DashMap;

    #[test]
    #[ignore]
    fn generate_world_image() {
        use rayon::prelude::*;

        let seed = 123456789;
        let img_size = 32;
        let mut image = image::ImageBuffer::new(img_size * 16, img_size * 16);
        let generator = WorldGenerator::new(seed);

        let pixel_map = DashMap::new();

        (0..img_size).into_par_iter().for_each(|x| {
            (0..img_size).for_each(|z| {
                let mut chunk = Chunk::new(x as i32, z as i32, "overworld".to_string());
                generator.carve_chunk(&mut chunk).unwrap();
                for chunk_x in 0..16 {
                    for chunk_z in 0..16 {
                        let biome = generator.get_biome(&chunk, chunk_x as usize, chunk_z as usize);
                        let color = match biome._biome_name().as_str() {
                            "mountain" => image::Rgb([128u8, 128, 128]),
                            "plains" => image::Rgb([0, 204, 102]),
                            "ocean" => image::Rgb([0, 102, 204]),
                            _ => image::Rgb([255, 255, 255]),
                        };
                        let pixel_x = (x * 16 + chunk_x);
                        let pixel_z = (z * 16 + chunk_z);
                        pixel_map.insert((pixel_x, pixel_z), color);
                        // draw a black border around the chunk
                        if chunk_x == 0 || chunk_x == 15 || chunk_z == 0 || chunk_z == 15 {
                            pixel_map.insert((pixel_x, pixel_z), image::Rgb([0, 0, 0]));
                        }
                    }
                }
            });
        });
        image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            if let Some(color) = pixel_map.get(&(x, y)) {
                *pixel = *color;
            } else {
                *pixel = image::Rgb([0, 0, 0]); // Default to black if no color found
            }
        });
        image.save("world_image.png").unwrap();
    }
}
