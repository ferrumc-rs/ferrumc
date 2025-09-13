use crate::common::aquifer::{FluidPicker, FluidType};
use crate::common::surface::PreliminarySurface;
use crate::overworld::noise_biome_parameters::is_deep_dark_region;
use crate::overworld::noise_depth::OverworldBiomeNoise;
use crate::perlin_noise::{
    AQUIFER_BARRIER, AQUIFER_FLUID_LEVEL_FLOODEDNESS, AQUIFER_FLUID_LEVEL_SPREAD, AQUIFER_LAVA,
    NormalNoise,
};
use crate::pos::ChunkPos;
use crate::random::Xoroshiro128PlusPlusFactory;
use std::ops::Add;

use itertools::Itertools;

use bevy_math::{DVec3, FloatExt, IVec2, IVec3, Vec3Swizzles};

use crate::random::{Rng, RngFactory};

pub struct Aquifer {
    pub sea_level: FluidPicker,
    random: Xoroshiro128PlusPlusFactory,
    is_overworld: bool,
    barrier_noise: NormalNoise<1>,
    fluid_level_floodedness_noise: NormalNoise<1>,
    fluid_level_spread_noise: NormalNoise<1>,
    lava_noise: NormalNoise<1>,
}

/// a 16 by 16 by 12 Region
#[derive(Clone, Copy)]
struct AquiferSectionPos {
    pos: IVec3,
}

impl AquiferSectionPos {
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

impl Add for AquiferSectionPos {
    type Output = AquiferSectionPos;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            pos: self.pos + rhs.pos,
        }
    }
}

impl From<IVec3> for AquiferSectionPos {
    fn from(value: IVec3) -> Self {
        Self::new(
            value.x.div_euclid(16),
            value.x.div_euclid(12),
            value.z.div_euclid(16),
        )
    }
}

/// returns optional fluid type and if it should be updated
impl Aquifer {
    pub fn new(sea_level: FluidPicker, random: Xoroshiro128PlusPlusFactory) -> Self {
        let random = random.with_hash("minecraft:aquifer").fork_positional();
        Self {
            sea_level,
            random,
            is_overworld: true,
            barrier_noise: AQUIFER_BARRIER.init(random),
            fluid_level_floodedness_noise: AQUIFER_FLUID_LEVEL_FLOODEDNESS.init(random),
            fluid_level_spread_noise: AQUIFER_FLUID_LEVEL_SPREAD.init(random),
            lava_noise: AQUIFER_LAVA.init(random),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn at(
        &self,
        preliminary_surface: &PreliminarySurface,
        biome_noise: &OverworldBiomeNoise,
        pos: IVec3,
        final_density: f64,
    ) -> (Option<FluidType>, bool) {
        const FLOWING_UPDATE_SIMILARITY: f64 = similarity(10 * 10, 12 * 12);
        if final_density > 0.0 {
            return (None, false);
        }
        if !self.is_overworld {
            return (Some(simple_compute_fluid(pos.y, self.sea_level)), false);
        }
        let final_density = -final_density;

        if simple_compute_fluid(pos.y, self.sea_level) == FluidType::Lava {
            return (Some(FluidType::Lava), false);
        }

        let section: AquiferSectionPos = IVec3::new(pos.x - 5, pos.y + 1, pos.z - 5).into();

        let smallest: [(i32, IVec3); 4] = (0..=1)
            .rev()
            .cartesian_product((-1..=1).rev())
            .cartesian_product((0..=1).rev())
            .map(|((x, y), z)| section + IVec3::new(x, y, z).into())
            .map(|offset_section| {
                let mut random = self.random.with_pos(offset_section.pos);
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

        let nearest_status = self.compute_fluid(smallest[0].1, preliminary_surface, biome_noise);
        let block_state = nearest_status.at(pos.y);
        let similtarity = similarity(smallest[0].0, smallest[1].0);

        if similtarity <= 0.0 {
            // i believe this is the hot path
            return (
                Some(block_state),
                similtarity >= FLOWING_UPDATE_SIMILARITY
                    && !self
                        .compute_fluid(smallest[1].1, preliminary_surface, biome_noise)
                        .eq(&nearest_status),
            );
        }
        if block_state == FluidType::Water
            && simple_compute_fluid(pos.y, self.sea_level) == FluidType::Lava
        {
            return (Some(block_state), true);
        }
        let barrier = self
            .barrier_noise
            .at(pos.as_dvec3() * DVec3::new(1.0, 0.5, 1.0));
        let second_nearest_status =
            self.compute_fluid(smallest[1].1, preliminary_surface, biome_noise);
        if similtarity * Self::pressure(pos, barrier, nearest_status, second_nearest_status)
            > final_density
        {
            return (None, false);
        }
        let third_nearest_status =
            self.compute_fluid(smallest[2].1, preliminary_surface, biome_noise);
        let d2 = similarity(smallest[0].0, smallest[2].0);

        if d2 > 0.0
            && similtarity * d2 * Self::pressure(pos, barrier, nearest_status, third_nearest_status)
                > final_density
        {
            return (None, false);
        }

        let d3 = similarity(smallest[1].0, smallest[2].0);
        if d3 > 0.0
            && similtarity
                * d3
                * Self::pressure(pos, barrier, second_nearest_status, third_nearest_status)
                > final_density
        {
            return (None, false);
        }

        (
            Some(block_state),
            (nearest_status != second_nearest_status)
                || (d3 >= FLOWING_UPDATE_SIMILARITY
                    && second_nearest_status != third_nearest_status)
                || (d2 >= FLOWING_UPDATE_SIMILARITY && nearest_status != third_nearest_status)
                || (d2 >= FLOWING_UPDATE_SIMILARITY
                    && similarity(smallest[0].0, smallest[3].0) >= FLOWING_UPDATE_SIMILARITY
                    && nearest_status
                        != self.compute_fluid(smallest[3].1, preliminary_surface, biome_noise)),
        )
    }

    fn pressure(
        pos: IVec3,
        barrier: f64,
        first_fluid: FluidPicker,
        second_fluid: FluidPicker,
    ) -> f64 {
        let block_state = first_fluid.at(pos.y);
        let block_state1 = second_fluid.at(pos.y);
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

    //TODO: this is cached with just section poses... idk why
    fn compute_fluid(
        &self,
        pos: IVec3,
        preliminary_surface: &PreliminarySurface,
        biome_noise: &OverworldBiomeNoise,
    ) -> FluidPicker {
        const SURFACE_SAMPLING_OFFSETS_IN_CHUNKS: [IVec2; 13] = [
            IVec2::new(0, 0),
            IVec2::new(-32, -16),
            IVec2::new(-16, -16),
            IVec2::new(0, -16),
            IVec2::new(16, -16),
            IVec2::new(-3, 0),
            IVec2::new(-32, 0),
            IVec2::new(-16, 0),
            IVec2::new(16, 0),
            IVec2::new(-32, 16),
            IVec2::new(-16, 16),
            IVec2::new(0, 16),
            IVec2::new(16, 16),
        ];

        let fluid_status = if pos.y < self.sea_level.0.min(-54) {
            FluidPicker::new(-54, FluidType::Lava)
        } else {
            self.sea_level
        };

        let mut max_surface_level = i32::MAX;
        let mut fluid_present = false;

        for offset in SURFACE_SAMPLING_OFFSETS_IN_CHUNKS {
            let chunk_pos = ChunkPos::from(pos.xz() + offset);
            let new_y = preliminary_surface.at(chunk_pos) + 8;

            if offset == (0, 0).into() && pos.y - 12 > new_y {
                return fluid_status;
            }

            if (pos.y + 12 > new_y || offset == (0, 0).into())
                && simple_compute_fluid(new_y, self.sea_level) != FluidType::Air
            {
                if pos.y + 12 > new_y {
                    return if new_y < self.sea_level.0.min(-54) {
                        FluidPicker::new(-54, FluidType::Lava)
                    } else {
                        self.sea_level
                    };
                }
                fluid_present = true;
            }

            max_surface_level = max_surface_level.min(preliminary_surface.at(chunk_pos));
        }

        let serface_level = self.compute_surface_level(
            pos,
            fluid_status.0,
            max_surface_level,
            fluid_present,
            biome_noise,
        );
        let res = if serface_level.is_some_and(|surface| surface <= -10) && self.is_lava(pos) {
            FluidType::Lava
        } else {
            simple_compute_fluid(pos.y, self.sea_level)
        };
        FluidPicker::new(serface_level.unwrap_or(-64 * 1000), res) //TODO
    }

    fn is_lava(&self, pos: IVec3) -> bool {
        let pos = pos.div_euclid((64, 40, 64).into());
        self.lava_noise.at(pos.into()).abs() > 0.3
    }

    fn compute_surface_level(
        &self,
        pos: IVec3,
        default_level: i32,
        max_surface_level: i32,
        fluid_present: bool,
        biome_noise: &OverworldBiomeNoise,
    ) -> Option<i32> {
        if is_deep_dark_region(biome_noise, pos) {
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

        let floodedness = self
            .fluid_level_floodedness_noise
            .at(pos.as_dvec3() * DVec3::new(1.0, 0.67, 1.0))
            .clamp(-1.0, 1.0);
        let d4 = d2.remap(1.0, 0.0, -0.3, 0.8);
        let d5 = d2.remap(1.0, 0.0, -0.8, 0.4);

        if floodedness > d4 {
            Some(default_level)
        } else if floodedness > d5 {
            Some(self.calc_fluid_spread(pos).min(max_surface_level))
        } else {
            None
        }
    }

    fn calc_fluid_spread(&self, pos: IVec3) -> i32 {
        fn quantize(value: f64, factor: i32) -> i32 {
            (value / f64::from(factor)).floor() as i32 * factor
        }
        let pos = pos.div_euclid((16, 40, 16).into());
        let spread = quantize(
            self.fluid_level_spread_noise
                .at(pos.as_dvec3() * DVec3::new(1.0, 0.7142857142857143, 1.0))
                * 10.0,
            3,
        );
        pos.y * 40 + 20 + spread
    }
}
fn simple_compute_fluid(y: i32, sea_level: FluidPicker) -> FluidType {
    if y < sea_level.0.min(-54) {
        FluidType::Lava
    } else if y < sea_level.0 {
        sea_level.1
    } else {
        FluidType::Air
    }
}
const fn similarity(first_distance: i32, second_distance: i32) -> f64 {
    1.0 - ((second_distance - first_distance).abs() as f64) / (5.0 * 5.0)
}
//TODO: move
pub(crate) fn clamped_map(v: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    v.clamp(in_min, in_max)
        .remap(in_min, in_max, out_min, out_max)
}
