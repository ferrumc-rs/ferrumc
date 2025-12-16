use crate::common::aquifer::{FluidPicker, FluidType};
use crate::common::math::clamped_map;
use crate::overworld::noise_depth::OverworldBiomeNoise;
use crate::perlin_noise::{
    AQUIFER_BARRIER, AQUIFER_FLUID_LEVEL_FLOODEDNESS, AQUIFER_FLUID_LEVEL_SPREAD, AQUIFER_LAVA,
    NormalNoise,
};
use core::f64;
use ferrumc_world::pos::BlockPos;
use std::collections::HashMap;
use std::ops::Add;

use itertools::Itertools;

use bevy_math::DVec3;

use crate::random::Xoroshiro128PlusPlus;

pub const SEA_LEVEL: i32 = 63;
pub const SEA_TYPE: FluidType = FluidType::Water;
pub const LAVA_LEVEL: i32 = -54;
pub const LAVA_TYPE: FluidType = FluidType::Lava;

pub struct Aquifer {
    factory: Xoroshiro128PlusPlus,
    barrier_noise: NormalNoise<1>,
    fluid_level_floodedness_noise: NormalNoise<1>,
    fluid_level_spread_noise: NormalNoise<1>,
    lava_noise: NormalNoise<1>,
}

pub struct ChunkAquifer<'a> {
    aquifer: &'a Aquifer,
    biome_noise: &'a OverworldBiomeNoise,
    surrounding_fluids: HashMap<u64, FluidPicker>,
    to_update: Vec<u64>,
}

/// a 16 by 16 by 12 Region
#[derive(Clone, Copy)]
struct AquiferSectionPos {
    pos: BlockPos,
}

impl AquiferSectionPos {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            pos: BlockPos::of(x, y, z),
        }
    }

    //TODO: cache this if faster
    fn random_pos(self, factory: Xoroshiro128PlusPlus) -> BlockPos {
        let mut random = factory.at(self.pos); //TODO: wrong; should not be
        //readjusted for block pos
        self.pos
            + (
                random.next_bounded(10) as i32,
                random.next_bounded(9) as i32,
                random.next_bounded(10) as i32,
            )
    }
}

impl Add for AquiferSectionPos {
    type Output = AquiferSectionPos;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            pos: self.pos + rhs.pos.pos.into(),
        }
    }
}

impl From<BlockPos> for AquiferSectionPos {
    fn from(value: BlockPos) -> Self {
        Self::new(
            value.x().div_euclid(16) * 16,
            value.y().div_euclid(12) * 12,
            value.z().div_euclid(16) * 16,
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
    pub fn chunk<'a>(&'a self, biome_noise: &'a OverworldBiomeNoise) -> ChunkAquifer<'a> {
        ChunkAquifer {
            aquifer: self,
            biome_noise,
            surrounding_fluids: HashMap::new(),
            to_update: Vec::new(),
        }
    }
    /// uses (-1..=1) by (0..=1) surrounding chunks ( 3 * 2 = 6 chunks), offset by -5, -5 so at most (-2..=1) by (-1..=1)
    fn find_nearest_section_randoms(&self, pos: BlockPos) -> [(i32, BlockPos); 4] {
        let section: AquiferSectionPos = (pos + (-5, 1, -5)).into();
        (0..=1)
            .rev()
            .cartesian_product((-1..=1).rev())
            .cartesian_product((0..=1).rev())
            .map(|((x, y), z)| section + AquiferSectionPos::new(x, y, z))
            .map(|offset_section| {
                let random_pos = offset_section.random_pos(self.factory);
                (random_pos.distance_squared(pos), random_pos)
            })
            .k_smallest_by_key(4, |(dist, _)| *dist)
            .collect_array()
            .unwrap()
    }
    fn pressure(
        &self,
        pos: BlockPos,
        barrier: &mut f64,
        first_fluid: FluidPicker,
        second_fluid: FluidPicker,
    ) -> f64 {
        let block_state = first_fluid.at(pos.y());
        let block_state1 = second_fluid.at(pos.y());
        // Check lava/water mix edge case
        if block_state == FluidType::Lava && block_state1 == FluidType::Water
            || block_state1 == FluidType::Lava && block_state == FluidType::Water
        {
            return 2.0;
        }

        if first_fluid.0 == second_fluid.0 {
            return 0.0;
        }

        let offset = 2 * pos.y() + 1 - first_fluid.0 - second_fluid.0;
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
    fn spread_fluid(&self, pos: BlockPos, min_prelim_surface: i16) -> FluidPicker {
        let surface_level = self.fluid_spread(pos).min(min_prelim_surface);
        let res = simple_compute_fluid(pos.y());
        let res = if res != FluidType::Lava && surface_level <= -10 && self.is_lava(pos) {
            FluidType::Lava
        } else {
            res
        };
        FluidPicker::new(surface_level.into(), res)
    }
    fn is_lava(&self, pos: BlockPos) -> bool {
        let pos = pos.pos.div_euclid((64, 40, 64).into());
        self.lava_noise.at(pos).abs() > 0.3
    }
    fn barrier(&self, pos: BlockPos) -> f64 {
        let pos = DVec3::from(pos) * DVec3::new(1.0, 0.5, 1.0);
        self.barrier_noise.at(pos)
    }
    fn floodedness(&self, pos: BlockPos) -> f64 {
        let pos = DVec3::from(pos) * DVec3::new(1.0, 0.67, 1.0);
        self.fluid_level_floodedness_noise.at(pos)
    }
    fn fluid_spread(&self, pos: BlockPos) -> i16 {
        fn quantize(value: f64, factor: i16) -> i16 {
            (value / f64::from(factor)).floor() as i16 * factor
        }
        let pos = pos.pos.div_euclid((16, 40, 16).into());
        let noise_pos = pos.as_dvec3() * DVec3::new(1.0, 0.7142857142857143, 1.0);
        let spread = quantize(self.fluid_level_spread_noise.at(noise_pos) * 10.0, 3);
        pos.y as i16 * 40 + 20 + spread
    }
}
impl ChunkAquifer<'_> {
    pub fn at(&mut self, pos: BlockPos, final_density: f64) -> Option<FluidType> {
        const FLOWING_UPDATE_SIMILARITY: f64 = similarity(10 * 10, 12 * 12);
        if final_density > 0.0 {
            return None;
        }
        let final_density = -final_density;

        if simple_compute_fluid(pos.y()) == LAVA_TYPE {
            return Some(LAVA_TYPE);
        }

        let smallest = self.aquifer.find_nearest_section_randoms(pos);

        let nearest_status = self.compute_fluid(smallest[0].1);
        let block_state = nearest_status.at(pos.y());
        let s_0_1 = similarity(smallest[0].0, smallest[1].0);

        if block_state == FluidType::Water && simple_compute_fluid(pos.y() - 1) == FluidType::Lava {
            self.to_update.push(pos.pack());
            return Some(block_state);
        }
        let second_nearest_status = self.compute_fluid(smallest[1].1);
        if (FLOWING_UPDATE_SIMILARITY..=0.).contains(&s_0_1) {
            if nearest_status != second_nearest_status {
                self.to_update.push(pos.pack());
            }
            return Some(block_state);
        }
        let mut tmp = f64::NAN;
        let barrier = &mut tmp;
        if s_0_1
            * self
                .aquifer
                .pressure(pos, barrier, nearest_status, second_nearest_status)
            > final_density
        {
            return None;
        }
        let third_nearest_status = self.compute_fluid(smallest[2].1);
        let s_1_3 = similarity(smallest[0].0, smallest[2].0);
        let s_2_3 = similarity(smallest[1].0, smallest[2].0);

        if s_0_1
            * s_1_3
            * self
                .aquifer
                .pressure(pos, barrier, nearest_status, third_nearest_status)
            > final_density
            || s_0_1
                * s_2_3
                * self
                    .aquifer
                    .pressure(pos, barrier, second_nearest_status, third_nearest_status)
                > final_density
        {
            return None;
        }

        if (nearest_status != second_nearest_status)
            || (s_2_3 >= FLOWING_UPDATE_SIMILARITY && second_nearest_status != third_nearest_status)
            || (s_1_3 >= FLOWING_UPDATE_SIMILARITY && nearest_status != third_nearest_status)
            || (s_1_3 >= FLOWING_UPDATE_SIMILARITY
                && similarity(smallest[0].0, smallest[3].0) >= FLOWING_UPDATE_SIMILARITY
                && nearest_status != self.compute_fluid(smallest[3].1))
        {
            self.to_update.push(pos.pack());
        }

        Some(block_state)
    }

    ///this is only ever called at one position in each `AquiferSectionPos` so we may cache this
    ///per section.
    /// uses -1..=1 by -2..=1 chunk prelim surface and (-3, 0)
    /// so at most (-4..=2) by (-3..=2) is accessed
    ///TODO: cache this if faster
    fn compute_fluid(&mut self, pos: BlockPos) -> FluidPicker {
        if let Some(res) = self.surrounding_fluids.get(&pos.pack()) {
            *res
        } else {
            let res = self.compute_fluid_inner(pos);
            self.surrounding_fluids.insert(pos.pack(), res);
            res
        }
    }

    fn compute_fluid_inner(&self, pos: BlockPos) -> FluidPicker {
        const SURFACE_OFFESET: i16 = 8;
        let chunk = pos.chunk();
        let mut min_prelim_surface = self.biome_noise.preliminary_surface(chunk) + SURFACE_OFFESET;
        if pos.y() as i16 - 12 > min_prelim_surface {
            return simple_fluid_picker(pos.y());
        }
        for z in -1..=1 {
            for x in -2..=1 {
                let preliminary_surface = self
                    .biome_noise
                    .preliminary_surface(chunk + (x - if z == 0 && x < 1 { 1 } else { 0 }, z))
                    + SURFACE_OFFESET;
                if (pos.y() as i16 + 12 > preliminary_surface)
                    && preliminary_surface < SEA_LEVEL as i16
                {
                    return simple_fluid_picker(preliminary_surface.into());
                }
                min_prelim_surface = min_prelim_surface.min(preliminary_surface);
            }
        }

        if self.biome_noise.is_deep_dark_region(pos) {
            return FluidPicker::EMPTY;
        }
        let diff_to_surface_min = if min_prelim_surface < SEA_LEVEL as i16 {
            (min_prelim_surface - pos.y() as i16).into()
        } else {
            f64::MAX
        };
        let floodedness = self.aquifer.floodedness(pos);
        if floodedness > clamped_map(diff_to_surface_min, 0.0, 64.0, -0.3, 0.8) {
            simple_fluid_picker(pos.y())
        } else if floodedness <= clamped_map(diff_to_surface_min, 0.0, 64.0, -0.8, 0.4) {
            FluidPicker::EMPTY
        } else {
            self.aquifer
                .spread_fluid(pos, min_prelim_surface - SURFACE_OFFESET)
        }
    }
}

fn simple_compute_fluid(y: i32) -> FluidType {
    if y < SEA_LEVEL.min(LAVA_LEVEL) {
        LAVA_TYPE
    } else if y < SEA_LEVEL {
        SEA_TYPE
    } else {
        FluidType::Air
    }
}

fn simple_fluid_picker(y: i32) -> FluidPicker {
    if y < SEA_LEVEL.min(LAVA_LEVEL) {
        FluidPicker::new(LAVA_LEVEL, LAVA_TYPE)
    } else {
        FluidPicker::new(SEA_LEVEL, SEA_TYPE)
    }
}

const fn similarity(first_distance: i32, second_distance: i32) -> f64 {
    1.0 - ((second_distance - first_distance).abs() as f64) / (5.0 * 5.0)
}
