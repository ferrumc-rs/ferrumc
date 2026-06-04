//! Erosion noise field.
//!
//! Erosion is one of the climate axes that shape the terrain. Rather than directly subtracting
//! height (as an earlier version did), it now damps the local detail amplitude during surface
//! composition (see [`super::initial_height`]): high-erosion regions flatten into plains and
//! plateaus, low-erosion regions stay rugged. The raw erosion value is also recorded per column for
//! biome selection. This module owns only the sampler construction.

use crate::terrain_noise::{NoiseGenerator, Spline};

/// Builds the erosion-noise sampler.
///
/// The spline reshapes the raw `[0, 1]` noise so the mid-range maps to a gentle plateau (most
/// terrain erodes only a little) with sharper response at the high end. Frequency is higher than
/// the continentalness base so erosion adds finer-grained variation on top of the broad regions.
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
