//! Erosion carving: lowers the surface using an erosion-noise field reshaped by a spline,
//! producing flatter valleys and the occasional steeper drop. Runs after [`super::initial_height`].

use crate::terrain_noise::{NoiseGenerator, Spline};
use crate::{ColumnNoise, Heightmap, WorldGenerator};
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkPos;

/// Maximum number of blocks erosion can carve off a column at the high end of its range. Kept
/// well below the base [`super::initial_height::HEIGHT_AMPLITUDE`] so erosion *textures* the
/// terrain rather than dominating it and flattening everything to one level.
const MAX_EROSION_DEPTH: f32 = 16.0;

/// Builds the erosion-noise sampler.
///
/// The spline reshapes the raw `[0,1]` noise so the mid-range maps to a gentle plateau (most
/// terrain erodes only a little) with sharper carving at the high end. Frequency is higher than
/// the base height noise so erosion adds finer-grained variation on top of the broad hills.
pub(crate) fn erosion_noise(seed: u64) -> NoiseGenerator {
    let spline = Spline::new(vec![
        (0.0, 0.0),
        (0.3, 0.15),
        (0.5, 0.3),
        (0.7, 0.45),
        (0.9, 0.7),
        (1.0, 1.0),
    ]);
    NoiseGenerator::new(seed, 0.03, 4, Some(spline))
}

impl WorldGenerator {
    /// Second carving pass: reduces each column's surface height by an erosion amount derived from
    /// the (splined) erosion noise, clears the freshly exposed stone, and records the erosion
    /// value for biome selection.
    pub(crate) fn apply_erosion(
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

                let erosion_value = self.erosion_noise.get(global_x as f32, global_z as f32);

                let height_reduction = (erosion_value * MAX_EROSION_DEPTH) as i16;
                let surface_y = heightmap[local_x as usize][local_z as usize] - height_reduction;

                heightmap[local_x as usize][local_z as usize] = surface_y;
                col_noise[local_x as usize][local_z as usize].erosion = erosion_value;

                WorldGenerator::clear_above(chunk, local_x, local_z, surface_y);
            }
        }
    }
}
