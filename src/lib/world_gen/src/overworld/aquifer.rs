use crate::common::aquifer::{FluidPicker, FluidType};
use crate::common::math::clamped_map;
use crate::overworld::noise_depth::OverworldBiomeNoise;
use crate::perlin_noise::{
    AQUIFER_BARRIER, AQUIFER_FLUID_LEVEL_FLOODEDNESS, AQUIFER_FLUID_LEVEL_SPREAD, AQUIFER_LAVA,
    NormalNoise,
};
use crate::pos::{BlockPos, ChunkPos};
use core::f64;
use std::ops::Add;

use itertools::Itertools;

use bevy_math::{DVec3, IVec2, IVec3, Vec3Swizzles};

use crate::random::Xoroshiro128PlusPlus;

pub const SEA_LEVEL: i32 = 63;
pub const SEA_TYPE: FluidType = FluidType::Water;

pub struct Aquifer {
    factory: Xoroshiro128PlusPlus,
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

    //TODO: cache this if faster
    fn random_pos(self, factory: Xoroshiro128PlusPlus) -> BlockPos {
        let mut random = factory.at(self.pos);
        BlockPos::new(
            self.pos.x + random.next_bounded(10) as i32,
            self.pos.y + random.next_bounded(9) as i32,
            self.pos.z + random.next_bounded(10) as i32,
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
            value.x.div_euclid(16) * 16,
            value.x.div_euclid(12) * 12,
            value.z.div_euclid(16) * 16,
        )
    }
}

impl Aquifer {
    pub fn new(factory: Xoroshiro128PlusPlus) -> Self {
        let factory = factory.with_hash("minecraft:aquifer").fork();
        Self {
            factory,
            barrier_noise: AQUIFER_BARRIER.init(factory),
            fluid_level_floodedness_noise: AQUIFER_FLUID_LEVEL_FLOODEDNESS.init(factory),
            fluid_level_spread_noise: AQUIFER_FLUID_LEVEL_SPREAD.init(factory),
            lava_noise: AQUIFER_LAVA.init(factory),
        }
    }

    /// returns optional fluid type and if it should be updated
    pub(crate) fn at(
        &self,
        biome_noise: &OverworldBiomeNoise,
        pos: BlockPos,
        final_density: f64,
    ) -> (Option<FluidType>, bool) {
        const FLOWING_UPDATE_SIMILARITY: f64 = similarity(10 * 10, 12 * 12);
        if final_density > 0.0 {
            return (None, false);
        }
        let final_density = -final_density;

        if simple_compute_fluid(pos.y) == FluidType::Lava {
            return (Some(FluidType::Lava), false);
        }

        let smallest = self.find_nearest_section_randoms(pos);

        let nearest_status = self.compute_fluid(smallest[0].1, biome_noise);
        let block_state = nearest_status.at(pos.y);
        let s_0_1 = similarity(smallest[0].0, smallest[1].0);

        if s_0_1 <= 0.0 {
            return (
                Some(block_state),
                s_0_1 >= FLOWING_UPDATE_SIMILARITY
                    && nearest_status != self.compute_fluid(smallest[1].1, biome_noise),
            );
        }
        if block_state == FluidType::Water && simple_compute_fluid(pos.y - 1) == FluidType::Lava {
            return (Some(FluidType::Water), true);
        }
        let mut tmp = f64::NAN;
        let barrier = &mut tmp;
        let second_nearest_status = self.compute_fluid(smallest[1].1, biome_noise);
        if s_0_1 * self.pressure(pos, barrier, nearest_status, second_nearest_status)
            > final_density
        {
            return (None, false);
        }
        let third_nearest_status = self.compute_fluid(smallest[2].1, biome_noise);
        let s_1_3 = similarity(smallest[0].0, smallest[2].0);

        if s_0_1 * s_1_3 * self.pressure(pos, barrier, nearest_status, third_nearest_status)
            > final_density
        {
            return (None, false);
        }

        let s_2_3 = similarity(smallest[1].0, smallest[2].0);
        if s_0_1 * s_2_3 * self.pressure(pos, barrier, second_nearest_status, third_nearest_status)
            > final_density
        {
            return (None, false);
        }

        (
            Some(block_state),
            (nearest_status != second_nearest_status)
                || (s_2_3 >= FLOWING_UPDATE_SIMILARITY
                    && second_nearest_status != third_nearest_status)
                || (s_1_3 >= FLOWING_UPDATE_SIMILARITY && nearest_status != third_nearest_status)
                || (s_1_3 >= FLOWING_UPDATE_SIMILARITY
                    && similarity(smallest[0].0, smallest[3].0) >= FLOWING_UPDATE_SIMILARITY
                    && nearest_status != self.compute_fluid(smallest[3].1, biome_noise)),
        )
    }

    fn find_nearest_section_randoms(&self, pos: BlockPos) -> [(i32, BlockPos); 4] {
        let section: AquiferSectionPos = BlockPos::new(pos.x - 5, pos.y + 1, pos.z - 5).into();

        let smallest: [(i32, BlockPos); 4] = (0..=1)
            .rev()
            .cartesian_product((-1..=1).rev())
            .cartesian_product((0..=1).rev())
            .map(|((x, y), z)| section + IVec3::new(x, y, z).into())
            .map(|offset_section| {
                let random_pos = offset_section.random_pos(self.factory);
                (random_pos.distance_squared(pos), random_pos)
            })
            .k_smallest_by_key(4, |(dist, _)| *dist)
            .collect_array()
            .unwrap();
        smallest
    }

    fn pressure(
        &self,
        pos: IVec3,
        barrier: &mut f64,
        first_fluid: FluidPicker,
        second_fluid: FluidPicker,
    ) -> f64 {
        let block_state = first_fluid.at(pos.y);
        let block_state1 = second_fluid.at(pos.y);
        // Check lava/water mix edge case
        if block_state == FluidType::Lava && block_state1 == FluidType::Water
            || block_state1 == FluidType::Lava && block_state == FluidType::Water
        {
            return 2.0;
        }

        if first_fluid.0 == second_fluid.0 {
            return 0.0;
        }

        let offset = 2 * pos.y + 1 - first_fluid.0 - second_fluid.0;
        let diff = (first_fluid.0 - second_fluid.0).abs();
        let influance = f64::from(diff - offset.abs());

        let pessure = if offset > 0 {
            if influance > 0.0 {
                influance / 3.0
            } else {
                influance / 5.0
            }
        } else {
            let influance = 6.0 + influance;
            if influance > 0.0 {
                influance / 6.0
            } else {
                influance / 20.0
            }
        };

        let barrier = if (-2.0..=2.0).contains(&pessure) {
            if barrier.is_nan() {
                *barrier = self.barrier(pos);
            }
            *barrier
        } else {
            0.0
        };

        2.0 * (barrier + pessure)
    }

    ///this is only ever called at one position in each `AquiferSectionPos` so we may cache this
    ///per section.
    ///TODO: cache this if faster
    fn compute_fluid(&self, pos: BlockPos, biome_noise: &OverworldBiomeNoise) -> FluidPicker {
        let chunk = ChunkPos::of(pos);
        let mut min_prelim_surface = biome_noise.preliminary_surface(chunk);
        if pos.y - 12 > min_prelim_surface + 8 {
            return simple_fluid_picker(pos.y);
        }
        for y in -1..=1 {
            for x in -2..=1 {
                let preliminary_surface = biome_noise
                    .preliminary_surface(chunk + (x - if y == 0 && x < 1 { 1 } else { 0 }, y));
                if (pos.y + 12 > preliminary_surface + 8) && preliminary_surface + 8 < SEA_LEVEL {
                    return simple_fluid_picker(preliminary_surface + 8);
                }
                min_prelim_surface = min_prelim_surface.min(preliminary_surface);
            }
        }

        if biome_noise.is_deep_dark_region(pos) {
            return FluidPicker::EMPTY;
        }
        let tmp = if min_prelim_surface + 8 < SEA_LEVEL {
            (min_prelim_surface + 8 - pos.y).into()
        } else {
            f64::MAX
        };
        let floodedness = self.floodedness(pos);
        if floodedness > clamped_map(tmp, 0.0, 64.0, -0.3, 0.8) {
            simple_fluid_picker(pos.y)
        } else if floodedness <= clamped_map(tmp, 0.0, 64.0, -0.8, 0.4) {
            FluidPicker::EMPTY
        } else {
            self.spread_fluid(pos, min_prelim_surface)
        }
    }

    fn spread_fluid(&self, pos: IVec3, min_prelim_surface: i32) -> FluidPicker {
        let surface_level = self.fluid_spread(pos).min(min_prelim_surface);
        let res = simple_compute_fluid(pos.y);
        let res = if res != FluidType::Lava && surface_level <= -10 && self.is_lava(pos) {
            FluidType::Lava
        } else {
            res
        };
        FluidPicker::new(surface_level, res)
    }

    fn is_lava(&self, pos: IVec3) -> bool {
        let pos = pos.div_euclid((64, 40, 64).into());
        self.lava_noise.at(pos.into()).abs() > 0.3
    }
    fn barrier(&self, pos: IVec3) -> f64 {
        let pos = pos.as_dvec3() * DVec3::new(1.0, 0.5, 1.0);
        self.barrier_noise.at(pos)
    }
    fn floodedness(&self, pos: IVec3) -> f64 {
        let pos = pos.as_dvec3() * DVec3::new(1.0, 0.67, 1.0);
        self.fluid_level_floodedness_noise.at(pos)
    }
    fn fluid_spread(&self, pos: IVec3) -> i32 {
        fn quantize(value: f64, factor: i32) -> i32 {
            (value / f64::from(factor)).floor() as i32 * factor
        }
        let pos = pos.div_euclid((16, 40, 16).into());
        let noise_pos = pos.as_dvec3() * DVec3::new(1.0, 0.7142857142857143, 1.0);
        let spread = quantize(self.fluid_level_spread_noise.at(noise_pos) * 10.0, 3);
        pos.y * 40 + 20 + spread
    }
}

fn simple_compute_fluid(y: i32) -> FluidType {
    if y < SEA_LEVEL.min(-54) {
        FluidType::Lava
    } else if y < SEA_LEVEL {
        SEA_TYPE
    } else {
        FluidType::Air
    }
}

fn simple_fluid_picker(y: i32) -> FluidPicker {
    if y < SEA_LEVEL.min(-54) {
        FluidPicker::new(-54, FluidType::Lava)
    } else {
        FluidPicker::new(SEA_LEVEL, SEA_TYPE)
    }
}

const fn similarity(first_distance: i32, second_distance: i32) -> f64 {
    1.0 - ((second_distance - first_distance).abs() as f64) / (5.0 * 5.0)
}
