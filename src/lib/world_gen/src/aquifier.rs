use std::ops::Add;

use itertools::Itertools;

use bevy_math::{FloatExt, IVec3};

use crate::random::{Rng, RngFactory};
pub(crate) struct Noise; //TODO

#[derive(Clone, Copy)]
struct ColumnPos {
    x: i32,
    z: i32,
}

impl ColumnPos {
    fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    fn block(self, y: i32) -> IVec3 {
        IVec3::new(self.x, y, self.z)
    }
}

/// a 16 by 16 by 12 Region
#[derive(Clone, Copy)]
struct SectionPos {
    pos: IVec3,
}

impl SectionPos {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            pos: (x, y, z).into(),
        }
    }

    fn block(self, x: u32, y: u32, z: u32) -> IVec3 {
        IVec3::new(
            self.pos.x * 16 + x as i32,
            self.pos.y * 12 + y as i32,
            self.pos.z * 16 + z as i32,
        )
    }
}

impl Add for SectionPos {
    type Output = SectionPos;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            pos: self.pos + rhs.pos,
        }
    }
}

impl From<IVec3> for SectionPos {
    fn from(value: IVec3) -> Self {
        Self::new(
            value.x.div_euclid(16),
            value.x.div_euclid(16),
            value.z.div_euclid(16),
        )
    }
}

impl Noise {
    pub fn compute<T: Into<(i32, i32, i32)>>(&self, _pos: T) -> f64 {
        todo!()
    }
} //TODO
const FLOWING_UPDATE_SIMILARITY: f64 = similarity(10 * 10, 12 * 12);
pub struct SubstanceSettings {
    pub sea_level: (i32, FluidType),
    pub min_y: i32,
    pub height: i32,
    pub cell_height: u32,
    initial_density_no_jaggedness: Noise,
    erosion: Noise,
    depth: Noise,
    fluid_level_flodedness: Noise,
    fluid_level_spread_noise: Noise,
    lava_noise: Noise,
    barrier_noise: Noise,
}
/// rng is legacy if not generating overworld, returns optional fluid type and if it should be
/// updated
#[allow(dead_code)]
pub(crate) fn compute_substance<R: Rng<RF>, RF: RngFactory<R>>(
    pos: IVec3,
    final_density: f64,
    rng: RF,
    settings: &SubstanceSettings,
) -> (Option<FluidType>, bool) {
    if final_density > 0.0 {
        return (None, false);
    }
    let final_density = -final_density;

    if simple_compute_fluid(pos.y, settings.sea_level) == FluidType::Lava {
        return (Some(FluidType::Lava), false);
    }

    let section: SectionPos = IVec3::new(pos.x - 5, pos.y + 1, pos.z - 5).into();

    let smallest: [(i32, IVec3); 4] = (0..=1)
        .rev()
        .cartesian_product((-1..=1).rev())
        .cartesian_product((0..=1).rev())
        .map(|((x, y), z)| section + SectionPos::new(x, y, z))
        .map(|offset_section| {
            let mut random = rng.with_pos(offset_section.pos); //TODO: perf: cache this
            let random_pos = offset_section.block(
                random.next_bounded(10),
                random.next_bounded(9),
                random.next_bounded(10),
            );
            (random_pos.distance_squared(pos), random_pos)
        })
        .k_smallest_by_key(4, |(dist, _)| *dist)
        .collect_array()
        .unwrap();

    let nearest_status = compute_fluid(smallest[0].1, settings);
    let block_state = at(nearest_status, pos.y);
    let similtarity = similarity(smallest[0].0, smallest[1].0);

    if similtarity <= 0.0 {
        // i believe this is the hot path
        return (
            Some(block_state),
            similtarity >= FLOWING_UPDATE_SIMILARITY
                && !compute_fluid(smallest[1].1, settings).eq(&nearest_status),
        );
    }
    if block_state == FluidType::Water
        && simple_compute_fluid(pos.y, settings.sea_level) == FluidType::Lava
    {
        return (Some(block_state), true);
    }
    let barrier = settings.barrier_noise.compute(pos);
    let second_nearest_status = compute_fluid(smallest[1].1, settings);
    if similtarity * pressure(pos, barrier, nearest_status, second_nearest_status) > final_density {
        return (None, false);
    }
    let third_nearest_status = compute_fluid(smallest[2].1, settings);
    let d2 = similarity(smallest[0].0, smallest[2].0);

    if d2 > 0.0
        && similtarity * d2 * pressure(pos, barrier, nearest_status, third_nearest_status)
            > final_density
    {
        return (None, false);
    }

    let d3 = similarity(smallest[1].0, smallest[2].0);
    if d3 > 0.0
        && similtarity * d3 * pressure(pos, barrier, second_nearest_status, third_nearest_status)
            > final_density
    {
        return (None, false);
    }

    (
        Some(block_state),
        (nearest_status != second_nearest_status)
            || (d3 >= FLOWING_UPDATE_SIMILARITY && second_nearest_status != third_nearest_status)
            || (d2 >= FLOWING_UPDATE_SIMILARITY && nearest_status != third_nearest_status)
            || (d2 >= FLOWING_UPDATE_SIMILARITY
                && similarity(smallest[0].0, smallest[3].0) >= FLOWING_UPDATE_SIMILARITY
                && nearest_status != compute_fluid(smallest[3].1, settings)),
    )
}

const fn similarity(first_distance: i32, second_distance: i32) -> f64 {
    1.0 - ((second_distance - first_distance).abs() as f64) / (5.0 * 5.0)
}

fn at(fluid: (i32, FluidType), y: i32) -> FluidType {
    if y < fluid.0 { fluid.1 } else { FluidType::Air }
}

fn pressure(
    pos: IVec3,
    barrier: f64,
    first_fluid: (i32, FluidType),
    second_fluid: (i32, FluidType),
) -> f64 {
    let block_state = at(first_fluid, pos.y);
    let block_state1 = at(second_fluid, pos.y);
    // Check lava/water mix edge case
    if !((block_state != FluidType::Lava || block_state1 != FluidType::Water)
        && (block_state1 != FluidType::Lava || block_state != FluidType::Water))
    {
        return 2.0;
    }

    let abs = (first_fluid.0 - second_fluid.0).abs();
    if abs == 0 {
        return 0.0;
    }

    let average = 0.5 * f64::from(first_fluid.0 + second_fluid.0);
    let d1 = f64::from(pos.y) + 0.5 - average;
    let d2 = f64::from(abs) / 2.0;
    let d9 = d2 - d1.abs();

    let d11 = if d1 > 0.0 {
        if d9 > 0.0 { d9 / 1.5 } else { d9 / 2.5 }
    } else {
        let d10 = 3.0 + d9;
        if d10 > 0.0 { d10 / 3.0 } else { d10 / 10.0 }
    };

    let d12 = if (-2.0..=2.0).contains(&d11) {
        barrier
    } else {
        0.0
    };

    2.0 * (d12 + d11)
}

fn simple_compute_fluid(y: i32, sea_level: (i32, FluidType)) -> FluidType {
    if y < sea_level.0.min(-54) {
        FluidType::Lava
    } else if y < sea_level.0 {
        sea_level.1
    } else {
        FluidType::Air
    }
}

//TODO this is cached with just section poses... idk why
fn compute_fluid(pos: IVec3, settings: &SubstanceSettings) -> (i32, FluidType) {
    const SURFACE_SAMPLING_OFFSETS_IN_CHUNKS: [(i32, i32); 13] = [
        (0, 0),
        (-2, -1),
        (-1, -1),
        (0, -1),
        (1, -1),
        (-3, 0),
        (-2, 0),
        (-1, 0),
        (1, 0),
        (-2, 1),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let fluid_status = if pos.y < settings.sea_level.0.min(-54) {
        (-54, FluidType::Lava)
    } else {
        settings.sea_level
    };

    let mut max_surface_level = i32::MAX;
    let i1 = pos.y + 12;
    let i2 = pos.y - 12;
    let mut fluid_present = false;

    for ints in SURFACE_SAMPLING_OFFSETS_IN_CHUNKS {
        let preliminary_surface_level = preliminary_surface_level(
            ColumnPos::new(pos.x + (ints.0 << 4), pos.z + (ints.1 << 4)),
            settings,
        )
        .unwrap_or(i32::MAX);
        let i6 = preliminary_surface_level + 8;

        if ints.0 == 0 && ints.1 == 0 && i2 > i6 {
            return fluid_status;
        }

        if i1 > i6 || ints.0 == 0 && ints.1 == 0 {
            let pos = (pos.x + (ints.0 << 4), i6, pos.z + (ints.1 << 4));
            let fluid_status1 = if pos.1 < settings.sea_level.0.min(-54) {
                (-54, FluidType::Lava)
            } else {
                settings.sea_level
            };
            if simple_compute_fluid(pos.1, settings.sea_level) != FluidType::Air {
                if i1 > i6 {
                    return fluid_status1;
                }
                fluid_present = true;
            }
        }

        max_surface_level = max_surface_level.min(preliminary_surface_level);
    }

    let serface_level = compute_surface_level(
        pos,
        fluid_status.0,
        max_surface_level,
        fluid_present,
        settings,
    );
    let res = if serface_level.is_some_and(|surface| surface <= -10) && is_lava(pos, settings) {
        FluidType::Lava
    } else {
        simple_compute_fluid(pos.y, settings.sea_level)
    };
    (serface_level.unwrap_or(-64 * 1000), res) //TODO
}

fn is_lava(pos: IVec3, settings: &SubstanceSettings) -> bool {
    settings
        .lava_noise
        .compute((
            pos.x.div_euclid(64),
            pos.y.div_euclid(40),
            pos.z.div_euclid(64),
        ))
        .abs()
        > 0.3
}

fn compute_surface_level(
    pos: IVec3,
    default_level: i32,
    max_surface_level: i32,
    fluid_present: bool,
    settings: &SubstanceSettings,
) -> Option<i32> {
    if is_deep_dark_region(settings, pos) {
        return None;
    }
    let d2 = if fluid_present {
        clamped_map(
            f64::from(max_surface_level + 8 - pos.y),
            0.0,
            64.0,
            1.0,
            0.0,
        )
    } else {
        0.0
    };

    let floodedness = settings
        .fluid_level_flodedness
        .compute(pos)
        .clamp(-1.0, 1.0);
    let d4 = d2.remap(1.0, 0.0, -0.3, 0.8);
    let d5 = d2.remap(1.0, 0.0, -0.8, 0.4);

    if floodedness > d4 {
        Some(default_level)
    } else if floodedness > d5 {
        Some(calc_pluid_spread(pos, settings).min(max_surface_level))
    } else {
        None
    }
}

fn calc_pluid_spread(pos: IVec3, settings: &SubstanceSettings) -> i32 {
    let spread = quantize(
        settings.fluid_level_spread_noise.compute((
            pos.x.div_euclid(16),
            pos.y.div_euclid(40),
            pos.z.div_euclid(16),
        )) * 10.0,
        3,
    );
    pos.y.div_euclid(40) * 40 + 20 + spread
}

fn quantize(value: f64, factor: i32) -> i32 {
    (value / f64::from(factor)).floor() as i32 * factor
}

pub(crate) fn clamped_map(v: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    v.clamp(in_min, in_max)
        .remap(in_min, in_max, out_min, out_max)
}

fn is_deep_dark_region(settings: &SubstanceSettings, pos: IVec3) -> bool {
    settings.erosion.compute(pos) < -0.225 && settings.depth.compute(pos) > 0.9
}

fn preliminary_surface_level(column: ColumnPos, settings: &SubstanceSettings) -> Option<i32> {
    let column = ColumnPos::new(column.x & !3, column.z & !3);
    (settings.min_y..settings.min_y + settings.height)
        .rev()
        .step_by(settings.cell_height as usize)
        .find(|y| {
            settings
                .initial_density_no_jaggedness
                .compute(column.block(*y))
                > 0.390625
        })
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FluidType {
    Air,
    Water,
    Lava,
}
