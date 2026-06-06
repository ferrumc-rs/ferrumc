//! Climate model for large-scale biome regions.
//!
//! The terrain shape and biome layout are driven by a small set of low-frequency noise fields,
//! mirroring the approach vanilla Minecraft uses to lay out big contiguous regions rather than a
//! patchwork of per-column choices:
//!
//! * **continentalness** — the broadest field. Maps through [`Climate::continental_height`] to a
//!   base surface height, which is what turns parts of the world into deep ocean basins and others
//!   into raised inland. Its low frequency is what makes oceans and continents *large* instead of
//!   the scattered dips the previous single height field produced.
//! * **temperature** / **humidity** — two more low-frequency fields that, together with the existing
//!   erosion field and the resulting surface height, classify each column into a biome (see
//!   [`crate::WorldGenerator::get_biome`]). Because all three are low-frequency, biomes come out as
//!   broad bands rather than per-column noise.
//!
//! Sampling is a pure function of the world seed and global coordinates (no cross-column state), so
//! any column can be evaluated in isolation — which the cross-chunk tree overscan relies on.

use crate::terrain_noise::{NoiseGenerator, Spline};

/// World water level. Every column whose surface falls below this is flooded up to it in a single
/// global pass during chunk generation, independent of biome — so coastlines and inland basins fill
/// naturally. Matches the vanilla overworld sea level.
pub const SEA_LEVEL: i16 = 63;

/// Half-width of a river, in blocks, measured from its centreline. The centreline distance is
/// normalised by the noise gradient (see [`Climate::river_strength`]), so rivers are this wide
/// everywhere rather than ballooning into lakes wherever the noise happens to be flat near its
/// contour.
const RIVER_HALF_WIDTH_BLOCKS: f32 = 5.0;

/// The climate values sampled for a single column, carried alongside its surface height for biome
/// selection. All fields are normalised to `[0, 1]`.
#[derive(Clone, Copy, Default)]
pub(crate) struct ClimateSample {
    /// Broad land/ocean field: low → ocean basin, high → raised inland.
    pub continentalness: f32,
    /// Climate temperature axis: low → snowy, high → desert/savanna.
    pub temperature: f32,
    /// Climate humidity axis: low → dry (desert), high → forest.
    pub humidity: f32,
    /// Surface ruggedness from the erosion field: low → rugged/hilly, high → flat.
    pub erosion: f32,
    /// River carve factor in `[0, 1]`: `0` away from rivers, rising toward `1` along a river's
    /// centreline. It is the strength used to lower the surface into the channel (see
    /// [`crate::WorldGenerator::column`]) and to classify a column as a river biome. Always `0` over
    /// ocean and high ground, so a non-zero value unambiguously marks a lowland river column.
    pub river: f32,
}

/// Holds the low-frequency climate samplers and the continentalness→height spline. Built once per
/// world (the samplers are not free to construct) and shared across all chunk generation.
pub(crate) struct Climate {
    continentalness: NoiseGenerator,
    temperature: NoiseGenerator,
    humidity: NoiseGenerator,
    /// Low-frequency field whose `0.5` contour traces river centrelines. Independent of the climate
    /// axes so rivers wander across biomes rather than following them.
    river: NoiseGenerator,
    /// Maps continentalness `[0, 1]` to an absolute base surface height. The control points place a
    /// deep ocean floor at the low end, a shelf and coastline around the sea level, and rising
    /// inland terrain at the high end.
    continental_spline: Spline,
}

impl Climate {
    pub(crate) fn new(seed: u64) -> Self {
        // Frequencies set the region scale: ~1/0.0015 ≈ 660-block continents/oceans, with the
        // climate axes a touch broader still so a biome band spans a comparable distance.
        Self {
            continentalness: NoiseGenerator::new(seed.wrapping_add(11), 0.0015, 4, None),
            temperature: NoiseGenerator::new(seed.wrapping_add(23), 0.0012, 3, None),
            humidity: NoiseGenerator::new(seed.wrapping_add(37), 0.0013, 3, None),
            // ~1/0.005 ≈ 200-block period sets how often rivers recur. Three octaves keep the field
            // from plateauing near its 0.5 contour — flat spots there would (even after gradient
            // normalisation) widen into lakes — while also giving the centreline natural wander.
            river: NoiseGenerator::new(seed.wrapping_add(91), 0.005, 3, None),
            continental_spline: Spline::new(vec![
                (0.00, 16.0),  // deep ocean floor (~47 below sea level)
                (0.18, 30.0),  // ocean basin
                (0.32, 48.0),  // continental shelf
                (0.42, 60.0),  // coastline, just below sea level
                (0.50, 68.0),  // low inland
                (0.70, 88.0),  // hills
                (1.00, 112.0), // high inland
            ]),
        }
    }

    /// Samples the temperature and humidity axes for a column. Continentalness is sampled separately
    /// via [`Climate::continentalness`] because it is needed earlier (to derive the base height)
    /// than the climate axes (needed only for biome selection).
    pub(crate) fn sample(&self, global_x: i32, global_z: i32) -> (f32, f32) {
        let temperature = self.temperature.get(global_x as f32, global_z as f32);
        let humidity = self.humidity.get(global_x as f32, global_z as f32);
        (temperature, humidity)
    }

    /// Raw continentalness value in `[0, 1]` for a column.
    pub(crate) fn continentalness(&self, global_x: i32, global_z: i32) -> f32 {
        self.continentalness.get(global_x as f32, global_z as f32)
    }

    /// River strength in `[0, 1]` for a column: `0` outside the river band, rising to `1` at the
    /// centreline (the river field's `0.5` contour). This is the proximity to the centreline only;
    /// the caller gates it to lowland columns before using it (see
    /// [`crate::WorldGenerator::column`]).
    ///
    /// The signed distance to the contour (`value - 0.5`) is divided by the local gradient of the
    /// field, turning it into an approximate distance *in blocks*. Normalising this way is what gives
    /// rivers a consistent width: without it, a fixed threshold on the raw value makes the river
    /// balloon into a lake wherever the noise is flat near `0.5` and pinch to nothing where it is
    /// steep.
    pub(crate) fn river_strength(&self, global_x: i32, global_z: i32) -> f32 {
        let x = global_x as f32;
        let z = global_z as f32;
        let centre = self.river.get(x, z) - 0.5;
        // Central-difference gradient of the field (value change per block).
        let dx = (self.river.get(x + 1.0, z) - self.river.get(x - 1.0, z)) * 0.5;
        let dz = (self.river.get(x, z + 1.0) - self.river.get(x, z - 1.0)) * 0.5;
        // Floor the gradient so a near-flat patch of the field cannot blow the distance up into a
        // wide river; such patches are treated as far from the contour (no river) instead.
        let grad = (dx * dx + dz * dz).sqrt().max(1e-3);
        let dist_blocks = centre.abs() / grad;
        ((RIVER_HALF_WIDTH_BLOCKS - dist_blocks) / RIVER_HALF_WIDTH_BLOCKS).max(0.0)
    }

    /// Absolute base surface height for a column from its continentalness value, before local detail
    /// and erosion shape it.
    pub(crate) fn continental_height(&self, continentalness: f32) -> f32 {
        self.continental_spline.sample(continentalness)
    }
}
