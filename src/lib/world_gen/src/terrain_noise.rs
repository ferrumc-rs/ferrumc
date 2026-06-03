//! Noise sampling for terrain generation.
//!
//! Ported from the `feature/nicer-terrain` branch's layered height/erosion approach, but adapted
//! to FerrumC's existing dependency set: it uses the `noise` crate (already a workspace
//! dependency) instead of the upstream branch's git-pinned `simdnoise` fork, and a small local
//! linear-interpolation spline instead of pulling in the `splines` crate. This keeps the
//! generator self-contained with no new external dependencies.

use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

/// A piecewise-linear curve used to reshape a raw noise value in `[0, 1]` into a desired
/// response. Replaces the upstream `splines::Spline` with a dependency-free equivalent; linear
/// interpolation between control points is sufficient for terrain shaping and keeps the behaviour
/// easy to reason about.
///
/// Control points must be supplied sorted by their `x` (input) coordinate.
#[derive(Clone)]
pub struct Spline {
    points: Vec<(f32, f32)>,
}

impl Spline {
    /// Builds a spline from `(input, output)` control points. Points are sorted by input so
    /// callers may pass them in any order.
    pub fn new(mut points: Vec<(f32, f32)>) -> Self {
        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        Self { points }
    }

    /// Samples the curve at `t`, clamping to the first/last control point outside the defined
    /// range and linearly interpolating between the surrounding points inside it.
    pub fn sample(&self, t: f32) -> f32 {
        match self.points.as_slice() {
            [] => t,
            [only] => only.1,
            points => {
                if t <= points[0].0 {
                    return points[0].1;
                }
                if t >= points[points.len() - 1].0 {
                    return points[points.len() - 1].1;
                }
                for window in points.windows(2) {
                    let (x0, y0) = window[0];
                    let (x1, y1) = window[1];
                    if t >= x0 && t <= x1 {
                        let span = x1 - x0;
                        if span.abs() < f32::EPSILON {
                            return y0;
                        }
                        let frac = (t - x0) / span;
                        return y0 + (y1 - y0) * frac;
                    }
                }
                points[points.len() - 1].1
            }
        }
    }
}

/// A 2D fractal-noise sampler returning values normalised to `[0, 1]`, with an optional reshaping
/// [`Spline`] applied to the output.
///
/// This mirrors the role of the upstream branch's `NoiseGenerator` (seed + frequency + octaves +
/// optional spline) while being backed by the `noise` crate's `Fbm<Perlin>`.
pub struct NoiseGenerator {
    fbm: Fbm<Perlin>,
    frequency: f64,
    spline: Option<Spline>,
}

impl NoiseGenerator {
    /// Creates a sampler. `frequency` scales the input coordinates (smaller = broader features);
    /// `octaves` controls fractal detail; `spline`, if present, reshapes the normalised output.
    pub fn new(seed: u64, frequency: f64, octaves: u8, spline: Option<Spline>) -> Self {
        let fbm = Fbm::<Perlin>::new(seed as u32).set_octaves(octaves.max(1) as usize);
        Self {
            fbm,
            frequency,
            spline,
        }
    }

    /// Samples the noise at `(x, z)` and returns a value in `[0, 1]` (after the optional spline).
    pub fn get(&self, x: f32, z: f32) -> f32 {
        let nx = f64::from(x) * self.frequency;
        let nz = f64::from(z) * self.frequency;
        // `noise` returns roughly [-1, 1]; normalise to [0, 1].
        let raw = self.fbm.get([nx, nz]);
        let normalised = ((raw * 0.5) + 0.5).clamp(0.0, 1.0) as f32;
        match &self.spline {
            Some(spline) => spline.sample(normalised),
            None => normalised,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spline_clamps_and_interpolates() {
        let spline = Spline::new(vec![(0.0, 0.0), (0.5, 1.0), (1.0, 2.0)]);
        // Below range clamps to first point.
        assert_eq!(spline.sample(-1.0), 0.0);
        // Above range clamps to last point.
        assert_eq!(spline.sample(2.0), 2.0);
        // Midpoint of first segment.
        assert!((spline.sample(0.25) - 0.5).abs() < 1e-5);
        // Exact control point.
        assert!((spline.sample(0.5) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn noise_output_is_normalised() {
        let generator = NoiseGenerator::new(42, 0.01, 4, None);
        for i in 0..50 {
            let v = generator.get(i as f32 * 13.0, i as f32 * 7.0);
            assert!((0.0..=1.0).contains(&v), "noise out of range: {v}");
        }
    }
}
