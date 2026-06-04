//! Initial height carving: turns the broad height-noise field into a per-column surface height,
//! centred on [`crate::BASELINE_HEIGHT`], and clears the stone above it.

use crate::terrain_noise::NoiseGenerator;
use crate::{BASELINE_HEIGHT, ColumnNoise, Heightmap, WorldGenerator};
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkPos;

/// Peak-to-trough amplitude (in blocks) of the base height field. The surface varies by roughly
/// `+/- HEIGHT_AMPLITUDE` around [`crate::BASELINE_HEIGHT`] before erosion. Chosen so plains have
/// visible rolling hills rather than looking superflat.
const HEIGHT_AMPLITUDE: f32 = 48.0;

/// Builds the height-noise sampler.
///
/// The frequency is the *effective* world-space frequency: a value of `0.0125` gives terrain
/// features on the order of ~80 blocks across (1 / 0.0125), i.e. gentle hills a player notices
/// while walking. (The previous value of 0.01 combined with an extra `/32` divisor at the call
/// site collapsed the effective frequency to ~0.0003 — features spanning thousands of blocks,
/// which read as completely flat.)
pub(crate) fn height_noise(seed: u64) -> NoiseGenerator {
    NoiseGenerator::new(seed, 0.0125, 4, None)
}

impl WorldGenerator {
    /// First carving pass: sets each column's surface height from the height noise and clears the
    /// stone above it. Writes the resulting heights into `heightmap` and records the raw noise in
    /// `col_noise`.
    pub(crate) fn apply_initial_height(
        &self,
        chunk: &mut Chunk,
        pos: ChunkPos,
        heightmap: &mut Heightmap,
        col_noise: &mut [[ColumnNoise; 16]; 16],
    ) {
        for local_x in 0..16u8 {
            for local_z in 0..16u8 {
                let global_x = pos.x() * 16 + i32::from(local_x);
                let global_z = pos.z() * 16 + i32::from(local_z);

                // Sample directly in world space; the sampler's frequency sets the feature scale.
                let height_noise = self.height_noise.get(global_x as f32, global_z as f32);

                // Map noise [0,1] -> a signed offset of +/- HEIGHT_AMPLITUDE around the baseline.
                let offset = ((height_noise * 2.0) - 1.0) * HEIGHT_AMPLITUDE;
                let surface_y = BASELINE_HEIGHT + offset as i16;

                heightmap[local_x as usize][local_z as usize] = surface_y;
                col_noise[local_x as usize][local_z as usize].height = height_noise;

                WorldGenerator::clear_above(chunk, local_x, local_z, surface_y);
            }
        }
    }
}
