use bevy_math::DVec2;
use bevy_math::DVec3;
use bevy_math::FloatExt;
use bevy_math::Vec3Swizzles;

pub fn clamped_map(v: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    v.clamp(in_min, in_max)
        .remap(in_min, in_max, out_min, out_max)
}

pub fn lerp2(delta: DVec2, start1: f64, end1: f64, start2: f64, end2: f64) -> f64 {
    start1
        .lerp(end1, delta.x)
        .lerp(start2.lerp(end2, delta.x), delta.y)
}

#[allow(clippy::too_many_arguments)]
pub fn lerp3(
    delta: DVec3,
    start1: f64,
    end1: f64,
    start2: f64,
    end2: f64,
    start3: f64,
    end3: f64,
    start4: f64,
    end4: f64,
) -> f64 {
    lerp2(delta.xy(), start1, end1, start2, end2)
        .lerp(lerp2(delta.xy(), start3, end3, start4, end4), delta.z)
}
