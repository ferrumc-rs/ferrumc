//! Local detail noise and surface composition.
//!
//! The broad base height comes from the climate continentalness field ([`crate::climate`]); this
//! module owns the higher-frequency *detail* field layered on top of it and the per-column
//! composition that combines base height, detail, and erosion into a final surface height.

use crate::climate::ClimateSample;
use crate::terrain_noise::NoiseGenerator;
use crate::{Heightmap, WorldGenerator};
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkPos;

/// Maximum peak-to-baseline amplitude (in blocks) of the local detail field, reached on flat
/// inland (low erosion, high continentalness). Oceans and high-erosion regions scale well below
/// this. Large enough that inland terrain reads as proper hills rather than superflat.
const MAX_DETAIL_AMPLITUDE: f32 = 37.0;

/// Minimum detail amplitude (in blocks), applied even in deep ocean so the sea floor is not
/// perfectly flat.
const MIN_DETAIL_AMPLITUDE: f32 = 3.0;

/// Continentalness at and above which a column is treated as fully inland for the purpose of
/// scaling detail amplitude. Below it, amplitude ramps down toward the ocean floor.
const INLAND_CONTINENTALNESS: f32 = 0.42;

/// How much erosion damps the detail amplitude: at erosion 1.0 the amplitude is reduced by this
/// fraction, flattening high-erosion regions into plains/plateaus. Kept high so plains read as
/// genuinely flat; mountains stay tall because they are selected from *low*-erosion ground, where
/// this damping barely applies.
const EROSION_FLATTENING: f32 = 0.92;

/// How far erosion pulls the surface *down* (in blocks) at erosion 1.0. This couples elevation to
/// erosion the way vanilla does: high-erosion ground is both flat (above) and low-lying (here), so
/// plains sit low and gentle, while low-erosion mountains keep their full continental base height.
const EROSION_HEIGHT_DROP: f32 = 14.0;

/// Builds the local detail-noise sampler.
///
/// The frequency gives features on the order of ~80 blocks across (1 / 0.0125) — gentle hills a
/// player notices while walking — layered on top of the much broader continentalness base.
pub(crate) fn detail_noise(seed: u64) -> NoiseGenerator {
    NoiseGenerator::new(seed, 0.0125, 4, None)
}

impl WorldGenerator {
    /// Pure per-column surface height and climate sample. Composes the continentalness base height,
    /// the erosion-and-continentalness-scaled local detail, and records every climate axis for biome
    /// selection. Depends only on the world seed and global coordinates, so it can be evaluated for
    /// any column — including ones outside the chunk being generated.
    pub(crate) fn column(&self, global_x: i32, global_z: i32) -> (i16, ClimateSample) {
        let continentalness = self.climate.continentalness(global_x, global_z);
        let base = self.climate.continental_height(continentalness);

        let erosion = self.erosion_noise.get(global_x as f32, global_z as f32);
        let (temperature, humidity) = self.climate.sample(global_x, global_z);

        // Detail amplitude: ramps from MIN at/below the ocean toward MAX fully inland, then damped
        // by erosion so flat regions stay flat.
        let landness = (continentalness / INLAND_CONTINENTALNESS).clamp(0.0, 1.0);
        let amplitude = (MIN_DETAIL_AMPLITUDE
            + (MAX_DETAIL_AMPLITUDE - MIN_DETAIL_AMPLITUDE) * landness)
            * (1.0 - EROSION_FLATTENING * erosion);

        let detail =
            ((self.detail_noise.get(global_x as f32, global_z as f32) * 2.0) - 1.0) * amplitude;

        // Pull high-erosion (flat) ground down so plains sit low; mountains (low erosion) keep their
        // full base height.
        let erosion_drop = erosion * EROSION_HEIGHT_DROP;

        let surface = (base + detail - erosion_drop) as i16;
        (
            surface,
            ClimateSample {
                continentalness,
                temperature,
                humidity,
                erosion,
            },
        )
    }

    /// Carving pass: composes each column's final surface height (see [`WorldGenerator::column`]),
    /// clears the stone above it, and records the height and climate sample for biome selection.
    pub(crate) fn carve_surface(
        &self,
        chunk: &mut Chunk,
        pos: ChunkPos,
        heightmap: &mut Heightmap,
        col_climate: &mut [[ClimateSample; 16]; 16],
    ) {
        for local_x in 0..16u8 {
            for local_z in 0..16u8 {
                let global_x = pos.x() * 16 + i32::from(local_x);
                let global_z = pos.z() * 16 + i32::from(local_z);

                let (surface_y, sample) = self.column(global_x, global_z);

                heightmap[local_x as usize][local_z as usize] = surface_y;
                col_climate[local_x as usize][local_z as usize] = sample;

                WorldGenerator::clear_above(chunk, local_x, local_z, surface_y);
            }
        }
    }
}
