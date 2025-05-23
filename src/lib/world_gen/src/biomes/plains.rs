use crate::errors::WorldGenError;
use crate::BiomeGenerator;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::edit_batch::EditBatch;
use noise::{Clamp, NoiseFn, OpenSimplex};
use std::collections::BTreeMap;

pub(crate) struct PlainsBiome {
    dirt_depth_noise: Clamp<f64, OpenSimplex, 2>,
}

impl Default for PlainsBiome {
    fn default() -> Self {
        let dirt_depth_noise = OpenSimplex::new(0);
        PlainsBiome {
            dirt_depth_noise: Clamp::new(dirt_depth_noise).set_lower_bound(1.0).set_upper_bound(4.0),
        }
    }
}

impl BiomeGenerator for PlainsBiome {
    fn _biome_id(&self) -> u8 {
        0
    }

    fn _biome_name(&self) -> String {
        "plains".to_string()
    }

    fn decorate(&self, chunk: &mut Chunk, x: u8, z: u8) -> Result<(), WorldGenError> {
        let heightmap = chunk.real_heightmap;

        let mut edit_batch = EditBatch::new(chunk);

        // Add grass blocks to the top layer
        let y = heightmap[x as usize][z as usize];
        edit_batch.set_block(
            i32::from(x),
            i32::from(y),
            i32::from(z),
            ferrumc_world::vanilla_chunk_format::BlockData {
                name: "minecraft:grass_block".to_string(),
                properties: Some(BTreeMap::from([("snowy".to_string(), "false".to_string())])),
            },
        );
        let dirt_depth = self.dirt_depth_noise.get([f64::from(x), f64::from(z)]);
        for i in 1..=dirt_depth as i32 {
            edit_batch.set_block(
                i32::from(x),
                i32::from(y) - i,
                i32::from(z),
                ferrumc_world::vanilla_chunk_format::BlockData {
                    name: "minecraft:dirt".to_string(),
                    properties: None,
                },
            );
        }
        // Apply the edit batch to the chunk
        edit_batch.apply()?;
        Ok(())
    }
}


