use crate::errors::WorldGenError;
use crate::{BiomeGenerator, NoiseGenerator};
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::edit_batch::EditBatch;
use std::collections::BTreeMap;

pub(crate) struct PlainsBiome {
    dirt_depth_noise: NoiseGenerator,
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
            block!("grass_block", {snowy: false}),
        );
        let dirt_depth = (self.dirt_depth_noise.get(f32::from(x), f32::from(z)) * 5.0) + 3.0;
        for i in 1..=dirt_depth as i32 {
            edit_batch.set_block(
                i32::from(x),
                i32::from(y) - i,
                i32::from(z),
                block!("dirt"),
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
        let dirt_depth_noise = NoiseGenerator::new(seed, 0.1, 4, None);
        PlainsBiome {
            dirt_depth_noise,
        }
    }
}


