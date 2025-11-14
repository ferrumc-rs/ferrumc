use std::{
    f32::consts::{FRAC_PI_2, PI},
    iter::empty,
    range::Range,
};

use bevy_math::{DVec2, DVec3, IVec3, Vec2Swizzles, Vec3Swizzles};
use itertools::{Either, Itertools};

use crate::{
    pos::{BlockPos, ChunkHeight, ChunkPos},
    random::{LegacyRandom, Rng},
};

pub(crate) struct CarvingMask {
    carved: Vec<bool>,
    min_y: i32,
}
impl CarvingMask {
    pub(crate) fn new(chunk_height: ChunkHeight) -> Self {
        Self {
            min_y: chunk_height.min_y,
            carved: vec![false; (chunk_height.height * 16 * 16) as usize],
        }
    }
    pub(crate) fn carve(&mut self, pos: IVec3) -> bool {
        let i = pos.x & 15 | (pos.z & 15) << 4 | (pos.y - self.min_y) << 8;
        let res = self.carved[i as usize];
        self.carved[i as usize] = true;
        res
    }
}

pub(crate) fn carve_ellipsoid(
    chunk_pos: ChunkPos,
    pos: DVec3,
    radii: DVec2,
    chunk_height: ChunkHeight,
) -> impl Iterator<Item = (DVec3, IVec3)> {
    if (chunk_pos.center().pos.as_dvec2() - pos.xz())
        .abs()
        .max_element()
        > 16.0 + radii.x * 2.0
    {
        return Either::Left(empty());
    }

    let radii = radii.xyx();
    let min = ((pos - radii).floor().as_ivec3() - 1).max((0, chunk_height.min_y + 1, 0).into());
    let max = (pos + radii)
        .floor()
        .as_ivec3()
        .min((15, chunk_height.max_y() - 1 - 7, 15).into());
    let x = move |x| ((f64::from(x) + 0.5 - pos.x) / radii.x, x);
    let z = (min.z..=max.z).map(move |z| ((f64::from(z) + 0.5 - pos.z) / radii.z, z));
    let y = (min.y..=max.y)
        .rev()
        .map(move |y| ((f64::from(y) - 0.5 - pos.y) / radii.y, y));
    Either::Right(
        (min.x..=max.x)
            .map(x)
            .cartesian_product(z)
            .filter(|((dx, _), (dz, _))| dx * dx + dz * dz < 1.0)
            .cartesian_product(y)
            .map(|(((dx, x), (dz, z)), (dy, y))| (DVec3::new(dx, dy, dz), IVec3::new(x, y, z))),
    )
}

pub struct Caver {
    bound: u32,
    ratio: f64,
    chunk_height: ChunkHeight,
    probability: f32,
    y: Range<i32>,
    horizontal_radius_mul: Range<f32>,
    vertical_radius_mul: Range<f32>,
    floor_level: Range<f32>,
    y_scale: Range<f32>,
}
impl Caver {
    pub const fn new(
        bound: u32,
        ratio: f64,
        chunk_height: ChunkHeight,
        probability: f32,
        y: Range<i32>,
        horizontal_radius_mul: Range<f32>,
        vertical_radius_mul: Range<f32>,
        floor_level: Range<f32>,
        y_scale: Range<f32>,
    ) -> Self {
        Self {
            bound,
            ratio,
            chunk_height,
            probability,
            y,
            horizontal_radius_mul,
            vertical_radius_mul,
            floor_level,
            y_scale,
        }
    }

    pub(crate) fn carve(
        &self,
        clearer: &mut impl FnMut(BlockPos, &mut bool),
        mut thickness: impl FnMut(&mut LegacyRandom) -> f32,
        seed: u64,
        chunk_pos: ChunkPos,
        carving_mask: &mut CarvingMask,
    ) {
        let mut random = LegacyRandom::large_features(seed, chunk_pos);
        if random.next_f32() > self.probability {
            return;
        }

        let block_pos_coord = (4 * 2 - 1) << 4;
        let bound = random.next_bounded(self.bound) + 1;
        let bound1 = random.next_bounded(bound) + 1;
        for _ in 0..random.next_bounded(bound1) {
            let random_pos = chunk_pos.chunk_block(
                random.next_bounded(16) as u8,
                random.next_i32_range(self.y),
                random.next_bounded(16) as u8,
            );
            let horizontal_radius_mul = random.next_f32_range(self.horizontal_radius_mul);
            let vertical_radius_mul = random.next_f32_range(self.vertical_radius_mul);
            let floor_level = random.next_f32_range(self.floor_level).into();

            let tunnels = if random.next_bounded(4) == 0 {
                let y_scale = f64::from(random.next_f32_range(self.y_scale));
                let radius = f64::from(1.0 + random.next_f32() * 6.0);
                let mut surface_reached = false;
                for (relative, pos) in carve_ellipsoid(
                    chunk_pos,
                    random_pos.as_dvec3() + DVec3::from((1.0, 0.0, 0.0)),
                    (
                        1.5 + f64::from((FRAC_PI_2).sin()) * radius,
                        (1.5 + f64::from((FRAC_PI_2).sin()) * radius) * y_scale,
                    )
                        .into(),
                    self.chunk_height,
                ) {
                    if !(relative.y <= floor_level
                        || relative.length_squared() >= 1.0
                        || carving_mask.carve(pos))
                    {
                        clearer(pos, &mut surface_reached);
                    }
                }
                random.next_bounded(4) + 1
            } else {
                1
            };

            for _ in 0..tunnels {
                let f1 = random.next_f32() * (PI * 2.0);
                let f = (random.next_f32() - 0.5) / 4.0;
                let thickness = thickness(&mut random);
                let branch_count = block_pos_coord - random.next_bounded(block_pos_coord / 4);

                let mut tunnler = Tunnler {
                    caver: self,
                    floor_level,
                    chunk_pos,
                    branch_count,
                    carving_mask,
                    horizontal_radius_multiplier: horizontal_radius_mul.into(),
                    vertical_radius_multiplier: vertical_radius_mul.into(),
                };

                tunnler.create_tunnel(
                    clearer,
                    random.next_random(),
                    random_pos.into(),
                    thickness,
                    f1,
                    f,
                    0,
                    self.ratio,
                );
            }
        }
    }
}

struct Tunnler<'a> {
    caver: &'a Caver,
    floor_level: f64,
    chunk_pos: ChunkPos,
    branch_count: u32,
    carving_mask: &'a mut CarvingMask,
    horizontal_radius_multiplier: f64,
    vertical_radius_multiplier: f64,
}
impl Tunnler<'_> {
    fn create_tunnel(
        &mut self,
        clearer: &mut impl FnMut(BlockPos, &mut bool),
        mut random: LegacyRandom,
        mut pos: DVec3,
        thickness: f32,
        mut yaw: f32,
        mut pitch: f32,
        branch_index: u32,
        horizontal_vertical_ratio: f64,
    ) {
        let i = random.next_bounded(self.branch_count / 2) + self.branch_count / 4;
        let more_pitch = random.next_bounded(6) == 0;
        let mut yaw_mul = 0.0f32;
        let mut pitch_mul = 0.0f32;

        for curr_branch_idx in branch_index..self.branch_count {
            let d = 1.5
                + f64::from((PI * curr_branch_idx as f32 / self.branch_count as f32).sin())
                    * f64::from(thickness);
            let cos = pitch.cos();
            pos.x += f64::from(yaw.cos() * cos);
            pos.y += f64::from(pitch.sin());
            pos.z += f64::from(yaw.sin() * cos);

            pitch *= if more_pitch { 0.92 } else { 0.7 };
            pitch += pitch_mul * 0.1;
            yaw += yaw_mul * 0.1;

            pitch_mul *= 0.9;
            yaw_mul *= 0.75;

            pitch_mul += (random.next_f32() - random.next_f32()) * random.next_f32() * 2.0;
            yaw_mul += (random.next_f32() - random.next_f32()) * random.next_f32() * 4.0;

            if curr_branch_idx == i && thickness > 1.0 {
                self.create_tunnel(
                    clearer,
                    random.next_random(),
                    pos,
                    random.next_f32() * 0.5 + 0.5,
                    yaw - FRAC_PI_2,
                    pitch / 3.0,
                    curr_branch_idx,
                    1.0,
                );
                self.create_tunnel(
                    clearer,
                    random.next_random(),
                    pos,
                    random.next_f32() * 0.5 + 0.5,
                    yaw + FRAC_PI_2,
                    pitch / 3.0,
                    curr_branch_idx,
                    1.0,
                );
                return;
            }

            if random.next_bounded(4) != 0 {
                if !can_reach(
                    self.chunk_pos,
                    pos,
                    curr_branch_idx,
                    self.branch_count,
                    thickness,
                ) {
                    return;
                }

                let mut surface_reached = false;
                for (relative, pos) in carve_ellipsoid(
                    self.chunk_pos,
                    pos,
                    (
                        d * self.horizontal_radius_multiplier,
                        d * horizontal_vertical_ratio * self.vertical_radius_multiplier,
                    )
                        .into(),
                    self.caver.chunk_height,
                ) {
                    if !(relative.y <= self.floor_level
                        || relative.length_squared() >= 1.0
                        || self.carving_mask.carve(pos))
                    {
                        clearer(pos, &mut surface_reached);
                    }
                }
            }
        }
    }
}
pub(crate) fn can_reach(
    chunk_pos: ChunkPos,
    pos: DVec3,
    branch_index: u32,
    branch_count: u32,
    width: f32,
) -> bool {
    chunk_pos.center().pos.as_dvec2().distance_squared(pos.xz())
        - f64::from((branch_count - branch_index).pow(2))
        <= f64::from(width) + 2.0 + 16.0
}
