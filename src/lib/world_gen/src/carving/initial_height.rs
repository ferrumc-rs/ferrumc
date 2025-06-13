use crate::noise::NoiseGenerator;
use crate::{WorldGenerator, BASELINE_HEIGHT, MAX_GENERATED_HEIGHT};
use ferrumc_world::edit_batch::EditBatch;
use ferrumc_world::vanilla_chunk_format::BlockData;
use splines::{Interpolation, Key, Spline};

impl WorldGenerator {
    pub fn apply_initial_height(&self, chunk: &mut ferrumc_world::chunk_format::Chunk) -> Result<(), crate::errors::WorldGenError> {
        let mut new_heightmap = [[0i16; 16]; 16];
        let mut height_noise_array = chunk.noises.height_noise;
        let chunk_x = chunk.x;
        let chunk_z = chunk.z;
        let mut edit_batch = EditBatch::new(chunk);
        for local_x in 0..16u8 {
            for local_z in 0..16u8 {
                let global_x = i64::from(chunk_x) * 16 + i64::from(local_x);
                let global_z = i64::from(chunk_z) * 16 + i64::from(local_z);
                let height_noise = self
                    .height_noise
                    .get(global_x as f32 / 32.0, global_z as f32 / 32.0);
                height_noise_array[local_x as usize][local_z as usize] = height_noise.to_bits();
                let height_mod = ((1.0 - height_noise) * 2.0) - 1.0; // Normalize to [-1, 1]
                let height_mod = (height_mod * 5.0) as i16; // Adjust erosion strength as needed
                let total_height = BASELINE_HEIGHT - height_mod;
                new_heightmap[local_x as usize][local_z as usize] = total_height;
                for y in total_height..=MAX_GENERATED_HEIGHT {
                    edit_batch.set_block(
                        i32::from(local_x),
                        i32::from(y),
                        i32::from(local_z),
                        BlockData {
                            name: "minecraft:air".to_string(),
                            properties: None,
                        },
                    );
                }
            }
        }
        // Update the chunk's heightmap
        edit_batch.apply()?;
        chunk.real_heightmap = new_heightmap;
        chunk.noises.height_noise = height_noise_array;
        Ok(())
    }
}


pub fn get_initial_height_noise(seed: u64) -> NoiseGenerator {
    let spline = Spline::from_vec(vec![
        Key::new(0.0, 0.0, Interpolation::Linear),
        Key::new(1.0, 1.0, Interpolation::Linear),
    ]);
    NoiseGenerator::new(seed, 0.3, 4, spline)
}