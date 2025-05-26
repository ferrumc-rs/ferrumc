use crate::errors::WorldGenError;
use crate::WorldGenerator;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::edit_batch::EditBatch;
use ferrumc_world::vanilla_chunk_format::BlockData;

impl WorldGenerator {
    pub(crate) fn height_gen(&self, x: i32, z: i32) -> Result<Chunk, WorldGenError> {
        let mut chunk = Chunk::new(x, z, "overworld".to_string());

        let mut edit_batch = EditBatch::new(&mut chunk);

        let mut min = i32::MAX;

        let mut min_section = i16::MAX;

        let mut heights = [[0i16; 16]; 16];

        for local_x in 0..16 {
            for local_z in 0..16 {
                let global_x = i64::from(x) * 16 + local_x;
                let global_z = i64::from(z) * 16 + local_z;
                let height_scaler = self.get_noise(global_x, global_z);
                let height_scaler = self.spline.sample(height_scaler).expect(
                    "Failed to sample height from spline",
                );
                let height = (height_scaler * 50.0) as i32 + 40;
                if height < min {
                    min = height;
                    min_section = (height >> 4) as i16;
                }
                heights[local_x as usize][local_z as usize] = height as i16;
            }
        }

        for local_x in 0..16 {
            for local_z in 0..16 {
                let height = heights[local_x as usize][local_z as usize];

                // Fill the sections above the lowest section with stone
                for section_y in min as i16..=height {
                    edit_batch.set_block(
                        local_x,
                        i32::from(section_y),
                        local_z,
                        BlockData {
                            name: "minecraft:stone".to_string(),
                            properties: None,
                        },
                    );
                }
            }
        }

        // Apply the edit batch to the chunk
        edit_batch.apply()?;

        // Fill the sections below the lowest section with stone
        for section_y in -4..min_section {
            chunk.set_section(
                section_y as i8,
                BlockData {
                    name: "minecraft:stone".to_string(),
                    properties: None,
                },
            )?;
        }

        chunk.real_heightmap = heights;

        Ok(chunk)
    }
}
