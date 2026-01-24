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

#[inline(always)]
fn smoothstep01(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}

#[inline(always)]
fn hash2d_01(seed: u64, x: i32, z: i32) -> f64 {
    // very fast mix -> [0,1)
    let mut v = seed
        ^ (x as u64).wrapping_mul(0x9E3779B185EBCA87)
        ^ (z as u64).wrapping_mul(0xC2B2AE3D27D4EB4F);
    v ^= v >> 33;
    v = v.wrapping_mul(0xFF51AFD7ED558CCD);
    v ^= v >> 33;
    (v as f64) / (u64::MAX as f64)
}

/// Coherent “value-noise-ish” field from x/z only.
/// cell_size controls patch size in blocks: 16/32 = big regions, 8 = smaller mottling.
#[inline(always)]
pub fn dither_field(seed: u64, x: i32, z: i32, cell_size: i32) -> f64 {
    let cx0 = x.div_euclid(cell_size);
    let cz0 = z.div_euclid(cell_size);
    let cx1 = cx0 + 1;
    let cz1 = cz0 + 1;

    let fx = (x.rem_euclid(cell_size) as f64) / (cell_size as f64);
    let fz = (z.rem_euclid(cell_size) as f64) / (cell_size as f64);

    let tx = smoothstep01(fx);
    let tz = smoothstep01(fz);

    let v00 = hash2d_01(seed, cx0, cz0);
    let v10 = hash2d_01(seed, cx1, cz0);
    let v01 = hash2d_01(seed, cx0, cz1);
    let v11 = hash2d_01(seed, cx1, cz1);

    let a = lerp(v00, v10, tx);
    let b = lerp(v01, v11, tx);
    lerp(a, b, tz) // 0..1
}
