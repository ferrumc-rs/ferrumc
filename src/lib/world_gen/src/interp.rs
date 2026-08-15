//! Small interpolation helpers used by cave carving.

#[inline(always)]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

// Helps avoid "blocky" interpolation artifacts.
#[inline(always)]
pub fn smoothstep(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}
