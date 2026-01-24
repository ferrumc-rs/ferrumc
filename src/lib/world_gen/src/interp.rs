// worldgen/interp.rs
#[inline(always)]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

// Helps avoid "blocky" interpolation artifacts.
#[inline(always)]
pub fn smoothstep(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}

#[inline(always)]
pub fn bilerp(c00: f64, c10: f64, c01: f64, c11: f64, tx: f64, tz: f64) -> f64 {
    let x0 = lerp(c00, c10, tx);
    let x1 = lerp(c01, c11, tx);
    lerp(x0, x1, tz)
}

#[expect(clippy::too_many_arguments)]
#[inline(always)]
pub fn trilerp(
    c000: f64,
    c100: f64,
    c010: f64,
    c110: f64,
    c001: f64,
    c101: f64,
    c011: f64,
    c111: f64,
    tx: f64,
    ty: f64,
    tz: f64,
) -> f64 {
    let x00 = lerp(c000, c100, tx);
    let x10 = lerp(c010, c110, tx);
    let x01 = lerp(c001, c101, tx);
    let x11 = lerp(c011, c111, tx);

    let y0 = lerp(x00, x10, ty);
    let y1 = lerp(x01, x11, ty);

    lerp(y0, y1, tz)
}
