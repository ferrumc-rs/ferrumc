//! Local detail noise and surface composition.
//!
//! The broad base height comes from the climate continentalness field ([`crate::climate`]); this
//! module owns the higher-frequency *detail* field layered on top of it and the per-column
//! composition that combines base height, detail, and erosion into a final surface height.

use crate::climate::{ClimateSample, SEA_LEVEL};
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

/// How far below [`SEA_LEVEL`] a river's centreline bed is carved. The channel fills with water from
/// the global sea-level flood, so a few blocks gives a visible river without deep gorges.
const RIVER_DEPTH: i16 = 4;

/// Land heights between which rivers fade out: at or below [`RIVER_FADE_LOW`] they carve at full
/// strength, at or above [`RIVER_FADE_HIGH`] not at all. Confining rivers to lowlands stops deep
/// canyons from slicing through hills and mountains.
const RIVER_FADE_LOW: f32 = 66.0;
const RIVER_FADE_HIGH: f32 = 88.0;

/// Combines a river's centreline proximity with land-height gating: rivers only carve on land above
/// sea level, and fade to nothing as the land rises toward mountains. Returns the carve/biome factor
/// in `[0, 1]`.
fn river_carve_factor(strength: f32, land_surface: f32) -> f32 {
    if strength <= 0.0 || land_surface <= f32::from(SEA_LEVEL) + 1.0 {
        return 0.0;
    }
    let height_fade =
        ((RIVER_FADE_HIGH - land_surface) / (RIVER_FADE_HIGH - RIVER_FADE_LOW)).clamp(0.0, 1.0);
    strength * height_fade
}

/// Gentle plains elevation, a little above [`crate::climate::SEA_LEVEL`], that flat (high-erosion)
/// land is pulled down toward. Decoupling plains from the continental base height is what stops them
/// from riding high just because they border raised or mountainous ground — they settle at their own
/// low level instead of ramping smoothly up to the peaks.
const PLAINS_LEVEL: f32 = 70.0;

/// Maps the erosion value to a "flatness" factor in `[0, 1]`: `0` = fully rugged (the column keeps
/// its continental base height and full local detail, i.e. mountains), `1` = fully flat (the column
/// is pulled down to [`PLAINS_LEVEL`] with only minimal detail, i.e. plains).
///
/// The steep middle section is deliberate: as the smooth erosion field crosses it, the surface height
/// changes over just a few blocks, turning what would be a gentle ramp into an abrupt terrace edge —
/// a "fault" between low plains and the rugged high ground. The riser sits around the same erosion
/// value the biome classifier uses to pick mountains, so the height step and the biome change line up.
fn flatness(erosion: f32) -> f32 {
    // (erosion, factor) control points; the steep 0.18→0.30 segment is the fault.
    const PTS: [(f32, f32); 4] = [(0.10, 0.0), (0.18, 0.40), (0.30, 0.92), (1.00, 1.0)];
    if erosion <= PTS[0].0 {
        return PTS[0].1;
    }
    let last = PTS.len() - 1;
    if erosion >= PTS[last].0 {
        return PTS[last].1;
    }
    for window in PTS.windows(2) {
        let (x0, y0) = window[0];
        let (x1, y1) = window[1];
        if erosion >= x0 && erosion <= x1 {
            return y0 + (y1 - y0) * ((erosion - x0) / (x1 - x0));
        }
    }
    PTS[last].1
}

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

        // Erosion drives both elevation and ruggedness through one factor. High-erosion land is
        // pulled down toward PLAINS_LEVEL and flattened; low-erosion land keeps its full continental
        // base height and detail (mountains). The steep `flatness` curve makes the transition abrupt
        // — a fault between low plains and high ground rather than a smooth ramp.
        let flat = flatness(erosion);
        let ruggedness = 1.0 - flat;

        // Pull only *downward*: blend toward the plains level when it is below the base, so raised
        // inland flattens into low plains but ocean floors are never lifted.
        let target = base.min(PLAINS_LEVEL);
        let land_base = base + (target - base) * flat;

        // Detail amplitude: ramps from MIN at/below the ocean toward MAX fully inland, with the
        // variable part scaled by ruggedness so plains stay gentle while mountains keep full hills.
        // The MIN floor is always present so even flat ground (and the sea bed) is never dead flat.
        let landness = (continentalness / INLAND_CONTINENTALNESS).clamp(0.0, 1.0);
        let amplitude = MIN_DETAIL_AMPLITUDE
            + (MAX_DETAIL_AMPLITUDE - MIN_DETAIL_AMPLITUDE) * landness * ruggedness;

        let detail =
            ((self.detail_noise.get(global_x as f32, global_z as f32) * 2.0) - 1.0) * amplitude;

        // Base land surface before rivers.
        let base_surface = land_base + detail;

        // Rivers lower the surface into a channel along the river contour, which the global
        // sea-level flood then fills with water. Gated to lowland columns (never the sea, fading out
        // toward mountains), so rivers wind through plains and forests rather than gashing peaks.
        // The same factor classifies the column as a river biome, so channel and biome coincide.
        let river = river_carve_factor(
            self.climate.river_strength(global_x, global_z),
            base_surface,
        );
        let bed = f32::from(SEA_LEVEL - RIVER_DEPTH);
        let surface = (base_surface + (bed - base_surface) * river) as i16;

        (
            surface,
            ClimateSample {
                continentalness,
                temperature,
                humidity,
                erosion,
                river,
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
