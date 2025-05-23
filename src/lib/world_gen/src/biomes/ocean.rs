use crate::errors::WorldGenError;
use crate::BiomeGenerator;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::edit_batch::EditBatch;
use noise::NoiseFn;
use std::collections::BTreeMap;

pub struct OceanBiome {
    sand_depth_noise: noise::Clamp<f64, noise::OpenSimplex, 2>,
}

impl Default for OceanBiome {
    fn default() -> Self {
        let sand_depth_noise = noise::OpenSimplex::new(0);
        OceanBiome {
            sand_depth_noise: noise::Clamp::new(sand_depth_noise).set_lower_bound(1.0).set_upper_bound(4.0),
        }
    }
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
        let sand_depth = self.sand_depth_noise.get([f64::from(x), f64::from(z)]);
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
        // Add water blocks to the top layer
        for i in y..=15 {
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
}