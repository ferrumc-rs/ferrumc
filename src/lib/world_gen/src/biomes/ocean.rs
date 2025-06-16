use crate::errors::WorldGenError;
use crate::noise::NoiseGenerator;
use crate::{BiomeGenerator, BASELINE_HEIGHT};
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::edit_batch::EditBatch;
use rand::Rng;
use rand::SeedableRng;
use std::collections::BTreeMap;

pub struct OceanBiome {
    sand_depth_noise: NoiseGenerator,
    sand_height_offset_noise: NoiseGenerator,
    world_water_level: i16,
}

impl BiomeGenerator for OceanBiome {
    fn _biome_id(&self) -> u8 {
        1
    }

    fn _biome_name(&self) -> String {
        "ocean".to_string()
    }

    fn decorate(&self, chunk: &mut Chunk, x: u8, z: u8) -> Result<(), WorldGenError> {
        let heightmap = chunk.real_heightmap;

        let mut edit_batch = EditBatch::new(chunk);

        // Add grass blocks to the top layer
        let y = heightmap[x as usize][z as usize];
        let sand_depth = (self.sand_depth_noise.get(f32::from(x), f32::from(z)) * 3.0) + 3.0; // Scale the depth
        let sand_stone_depth = (self.sand_height_offset_noise.get(f32::from(x), f32::from(z)) * 2.0) as i32 + 5; // Offset for sandstone
        for i in 0..=sand_depth as i32 {
            edit_batch.set_block(
                i32::from(x),
                i32::from(y) - i,
                i32::from(z),
                ferrumc_world::vanilla_chunk_format::BlockData {
                    name: "minecraft:sand".to_string(),
                    properties: None,
                },
            );
        }
        // Set sandstone below the sand layer
        for i in 1..=sand_stone_depth {
            edit_batch.set_block(
                i32::from(x),
                i32::from(y) - sand_depth as i32 - i,
                i32::from(z),
                ferrumc_world::vanilla_chunk_format::BlockData {
                    name: "minecraft:sandstone".to_string(),
                    properties: None,
                },
            );
        }
        // Add water blocks to the top layer
        for i in y + 1..=self.world_water_level {
            edit_batch.set_block(
                i32::from(x),
                i32::from(i),
                i32::from(z),
                ferrumc_world::vanilla_chunk_format::BlockData {
                    name: "minecraft:water".to_string(),
                    properties: Some(BTreeMap::from([("level".to_string(), "0".to_string())])),
                },
            );
        }
        // Apply the edit batch to the chunk
        edit_batch.apply()?;
        Ok(())
    }

    fn new(seed: u64) -> Self
    where
        Self: Sized,
    {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        OceanBiome {
            sand_depth_noise: NoiseGenerator::new(seed, 0.1, 4, None),
            sand_height_offset_noise: NoiseGenerator::new(seed + 1, 0.1, 4, None),
            world_water_level: BASELINE_HEIGHT - rng.random_range(38..=42) as i16, // Random water level between 38 and 42 blocks below baseline
        }
    }
}
