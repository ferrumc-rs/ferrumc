use crate::errors::WorldGenError;
use crate::noise::NoiseGenerator;
use crate::{WorldGenerator, MAX_GENERATED_HEIGHT};
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::edit_batch::EditBatch;

impl WorldGenerator {
    #[inline(never)]
    pub fn apply_erosion(&self, chunk: &mut Chunk) -> Result<(), WorldGenError> {
        let mut new_heightmap = chunk.real_heightmap;
        let mut erosion_noise_array = chunk.noises.erosion_noise;
        let chunk_x = chunk.x;
        let chunk_z = chunk.z;
        let mut edit_batch = EditBatch::new(chunk);
        for local_x in 0..16u8 {
            for local_z in 0..16u8 {
                let global_x = i64::from(chunk_x) * 16 + i64::from(local_x);
                let global_z = i64::from(chunk_z) * 16 + i64::from(local_z);
                let erosion_value = self
                    .erosion_noise
                    .get(global_x as f32 / 32.0, global_z as f32 / 32.0);
                erosion_noise_array[local_x as usize][local_z as usize] = erosion_value.to_bits();
                let height_reduction = (erosion_value * 50.0) as i16; // Adjust erosion strength as needed
                let total_height =
                    new_heightmap[local_x as usize][local_z as usize] - height_reduction;
                new_heightmap[local_x as usize][local_z as usize] = total_height;
                for y in total_height..=MAX_GENERATED_HEIGHT {
                    edit_batch.set_block(
                        i32::from(local_x),
                        i32::from(y),
                        i32::from(local_z),
                        block!("air"),
                    );
                }
            }
        }
        // Update the chunk's heightmap
        edit_batch.apply()?;
        chunk.real_heightmap = new_heightmap;
        chunk.noises.erosion_noise = erosion_noise_array;
        Ok(())
    }
}

pub(crate) fn get_erosion_noise(seed: u64) -> NoiseGenerator {
    let spline = splines::Spline::from_vec(vec![
        splines::Key::new(0.0, 0.0, splines::Interpolation::Cosine),
        splines::Key::new(0.1, 0.35, splines::Interpolation::Linear),
        splines::Key::new(0.2, 0.5, splines::Interpolation::Linear),
        splines::Key::new(0.8, 0.5, splines::Interpolation::Linear),
        splines::Key::new(0.9, 0.6, splines::Interpolation::Linear),
        splines::Key::new(1.0, 1.0, splines::Interpolation::Linear),
    ]);
    NoiseGenerator::new(seed, 0.05, 4, Some(spline))
}
