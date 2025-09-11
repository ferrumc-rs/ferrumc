use std::{
    f32::consts::{FRAC_PI_2, PI},
    iter::empty,
    ops::Range,
};

use bevy_math::{DVec2, DVec3, IVec3, Vec2Swizzles, Vec3Swizzles};
use ferrumc_world::{block_id::BlockId, vanilla_chunk_format::BlockData};
use itertools::{Either, Itertools};

use crate::{
    aquifer::FluidType,
    biome_chunk::{BiomeChunk, BiomeNoise},
    pos::{BlockPos, ChunkHeight, ChunkPos},
    random::{LegacyRandom, Rng},
    surface::Surface,
};

pub struct ChunkAccess {}

impl ChunkAccess {
    pub fn get_block_state(&self, pos: IVec3) -> BlockData {
        todo!()
    }

    pub fn set_block_state(&mut self, pos: IVec3, data: BlockData) {
        todo!()
    }
}

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
    fn carve(&mut self, pos: IVec3) -> bool {
        let i = pos.x & 15 | (pos.z & 15) << 4 | (pos.y - self.min_y) << 8;
        let res = self.carved[i as usize];
        self.carved[i as usize] = true;
        res
    }
}

fn carve_ellipsoid(
    chunk_pos: ChunkPos,
    pos: DVec3,
    radii: DVec2,
    chunk_height: ChunkHeight,
) -> impl Iterator<Item = (DVec3, IVec3)> {
    if (chunk_pos.column_pos(8, 8).pos.as_dvec2() - pos.xz())
        .abs()
        .max_element()
        > 16.0 + radii.x * 2.0
    {
        return Either::Left(empty());
    }

    let radii = radii.xyx();
    let min = ((pos - radii).floor().as_ivec3() - IVec3::splat(1))
        .max((0, chunk_height.min_y + 1, 0).into());
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

fn can_reach(
    chunk_pos: ChunkPos,
    pos: DVec3,
    branch_index: u32,
    branch_count: u32,
    width: f32,
) -> bool {
    chunk_pos
        .column_pos(8, 8)
        .pos
        .as_dvec2()
        .distance_squared(pos.xz())
        - f64::from((branch_count - branch_index).pow(2))
        <= f64::from(width) + 2.0 + 16.0
}

fn clear_overworld_cave_block(
    unreplaceable: &[BlockId],
    chunk: &mut ChunkAccess,
    surface: &Surface,
    biome_accessor: &BiomeChunk,
    biome_noise: &BiomeNoise,
    surface_reached: &mut bool,
    pos: BlockPos,
) {
    let block = chunk.get_block_state(pos);

    if unreplaceable.contains(&block.to_block_id()) {
        return;
    }

    if block.name == "minecraft:grass_block" || block.name == "minecraft:mycelium" {
        *surface_reached = true;
    }

    if let (Some(carve_state), fluid_update /* TODO */) =
        surface
            .aquifer
            .compute_substance(&surface.preliminary_surface, biome_noise, pos, 0.0)
    {
        chunk.set_block_state(pos, carve_state.into());
        if *surface_reached {
            let check_pos = pos - IVec3::new(0, 1, 0);
            if chunk.get_block_state(check_pos).name == "minecraft:dirt"
                && let Some(block_state1) = surface.top_material(
                    biome_accessor.at(check_pos),
                    check_pos,
                    carve_state != FluidType::Air,
                )
            {
                chunk.set_block_state(check_pos, block_state1);
                // if block_state1.name == "minecraft:water" || block_state1.name == "minecraft:lava" {
                //     //TODO
                // }
            }
        }
    };
}
pub struct Caver {
    chunk_height: ChunkHeight,
    unreplaceable: Vec<BlockId>,
    probability: f32,
    y: Range<i32>,
    horizontal_radius_mul: Range<f32>,
    vertical_radius_mul: Range<f32>,
    floor_level: Range<f32>,
    y_scale: Range<f32>,
}
impl Caver {
    #[allow(dead_code)]
    pub(crate) fn carve(
        &self,
        chunk: &mut ChunkAccess,
        biome_accessor: &BiomeChunk,
        seed: u64,
        chunk_pos: ChunkPos,
        carving_mask: &mut CarvingMask,
        surface: &Surface,
        biome_noise: &BiomeNoise,
    ) {
        let mut random = LegacyRandom::large_features(seed, chunk_pos);
        if random.next_f32() > self.probability {
            return;
        }

        let block_pos_coord = (4 * 2 - 1) << 4;
        let bound = random.next_bounded(15 /*TODO: nether*/) + 1;
        let bound1 = random.next_bounded(bound) + 1;
        for _ in 0..random.next_bounded(bound1) {
            let random_pos = chunk_pos.block(
                random.next_bounded(16),
                random.next_i32_range(self.y.clone()),
                random.next_bounded(16),
            );
            let horizontal_radius_mul = random.next_f32_range(self.horizontal_radius_mul.clone());
            let vertical_radius_mul = random.next_f32_range(self.vertical_radius_mul.clone());
            let floor_level = random.next_f32_range(self.floor_level.clone()).into();

            let tunnels = if random.next_bounded(4) == 0 {
                let y_scale = f64::from(random.next_f32_range(self.y_scale.clone()));
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
                    if relative.y <= floor_level
                        || relative.length_squared() >= 1.0
                        || carving_mask.carve(pos)
                    {
                        return;
                    }
                    clear_overworld_cave_block(
                        &self.unreplaceable,
                        chunk,
                        surface,
                        biome_accessor,
                        biome_noise,
                        &mut surface_reached,
                        pos,
                    )
                }
                random.next_bounded(4) + 1
            } else {
                1
            };

            for _ in 0..tunnels {
                let f1 = random.next_f32() * (PI * 2.0);
                let f = (random.next_f32() - 0.5) / 4.0;
                let thickness = random.next_f32() * 2.0//TODO: different in the nether 
                    + random.next_f32()
                        * if random.next_bounded(10) == 0 {
                            random.next_f32() * random.next_f32() * 3.0 + 1.0
                        } else {
                            1.0
                        };
                let branch_count = block_pos_coord - random.next_bounded(block_pos_coord / 4);

                let mut tunnler = Tunnler {
                    caver: self,
                    surface,
                    biome_noise,
                    floor_level,
                    chunk,
                    biome_accessor,
                    chunk_pos,
                    branch_count,
                    carving_mask,
                    horizontal_radius_multiplier: horizontal_radius_mul.into(),
                    vertical_radius_multiplier: vertical_radius_mul.into(),
                };

                tunnler.create_tunnel(
                    random.next_u64(),
                    random_pos.into(),
                    thickness,
                    f1,
                    f,
                    0,
                    1.0, //TODO: 5 in the nether
                );
            }
        }
    }
}

struct Tunnler<'a> {
    caver: &'a Caver,
    surface: &'a Surface,
    biome_noise: &'a BiomeNoise,
    floor_level: f64,
    chunk: &'a mut ChunkAccess,
    biome_accessor: &'a BiomeChunk,
    chunk_pos: ChunkPos,
    branch_count: u32,
    carving_mask: &'a mut CarvingMask,
    horizontal_radius_multiplier: f64,
    vertical_radius_multiplier: f64,
}
impl Tunnler<'_> {
    fn create_tunnel(
        &mut self,
        seed: u64,
        mut pos: DVec3,
        thickness: f32,
        mut yaw: f32,
        mut pitch: f32,
        branch_index: u32,
        horizontal_vertical_ratio: f64,
    ) {
        let mut random = LegacyRandom::new(seed);
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
                    random.next_u64(),
                    pos,
                    random.next_f32() * 0.5 + 0.5,
                    yaw - FRAC_PI_2,
                    pitch / 3.0,
                    curr_branch_idx,
                    1.0,
                );
                self.create_tunnel(
                    random.next_u64(),
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
                    if relative.y <= self.floor_level
                        || relative.length_squared() >= 1.0
                        || self.carving_mask.carve(pos)
                    {
                        return;
                    }
                    clear_overworld_cave_block(
                        &self.caver.unreplaceable,
                        self.chunk,
                        self.surface,
                        self.biome_accessor,
                        self.biome_noise,
                        &mut surface_reached,
                        pos,
                    )
                }
            }
        }
    }
}

pub struct CanyonCarver {
    unreplaceable: Vec<BlockId>,
    chunk_height: ChunkHeight,
}

impl CanyonCarver {
    #[allow(dead_code)]
    pub(crate) fn carve_canyon(
        &self,
        chunk: &mut ChunkAccess,
        biome_accessor: &BiomeChunk,
        seed: u64,
        chunk_pos: ChunkPos,
        carving_mask: &mut CarvingMask,
        surface: &Surface,
        biome_noise: &BiomeNoise,
    ) {
        const PROBABILITY: f32 = 0.01;
        const WIDTH_SMOOTHNESS: u32 = 3;
        const VERTICAL_RADIUS_DEFAULT_FACTOR: f64 = 1.0;
        const VERTICAL_RADIUS_CENTER_FACTOR: f64 = 0.0;
        const Y_SCALE: f64 = 3.0;
        let mut random = LegacyRandom::large_features(seed, chunk_pos);
        if random.next_f32() > PROBABILITY {
            return;
        }
        let mut random_pos = chunk_pos
            .block(
                random.next_bounded(16),
                random.next_i32_range(10..68),
                random.next_bounded(16),
            )
            .as_dvec3();
        let mut yaw = random.next_f32() * (PI * 2.0);
        let mut pitch = random.next_f32_range(-0.125..0.125);
        let thickness = random.next_trapezoid(0.0, 6.0, 2.0);
        let branch_count =
            (f64::from((4 * 2 - 1) * 16) * f64::from(random.next_f32_range(0.75..1.0))) as u32;

        let mut random = LegacyRandom::new(random.next_u64());
        let mut f = 0.0;
        let width_factors: Vec<f32> = (0..self.chunk_height.height)
            .map(|i| {
                if i == 0 || random.next_bounded(WIDTH_SMOOTHNESS) == 0 {
                    f = 1.0 + random.next_f32() * random.next_f32();
                }
                f
            })
            .collect();
        let mut yaw_factor = 0.0f32;
        let mut pitch_factor = 0.0f32;

        for i in 0..branch_count {
            let mut horizontal_radius =
                1.5 + f64::from((i as f32 * PI / branch_count as f32).sin()) * f64::from(thickness);

            horizontal_radius *= f64::from(random.next_f32_range(0.75..1.0));
            let vertical_radius = (VERTICAL_RADIUS_DEFAULT_FACTOR
                + VERTICAL_RADIUS_CENTER_FACTOR
                    * (1.0 - ((0.5 - f64::from(i) / f64::from(branch_count)).abs()) * 2.0))
                * horizontal_radius
                * Y_SCALE
                * f64::from(random.next_f32() * (1.0 - 0.75) + 0.75);

            random_pos.x += f64::from(yaw.cos() * pitch.cos());
            random_pos.y += f64::from(pitch.sin());
            random_pos.z += f64::from(yaw.sin() * pitch.cos());

            pitch *= 0.7;
            pitch += pitch_factor * 0.05;
            yaw += yaw_factor * 0.05;

            pitch_factor *= 0.8;
            yaw_factor *= 0.5;

            pitch_factor += (random.next_f32() - random.next_f32()) * random.next_f32() * 2.0;
            yaw_factor += (random.next_f32() - random.next_f32()) * random.next_f32() * 4.0;

            if random.next_bounded(4) != 0 {
                if !can_reach(chunk_pos, random_pos, i, branch_count, thickness) {
                    return;
                }

                let mut surface_reached = false;
                let radii = (horizontal_radius, vertical_radius).into();
                for (relative, pos) in
                    carve_ellipsoid(chunk_pos, random_pos, radii, self.chunk_height)
                {
                    if (relative.xz().length_squared())
                        * f64::from(width_factors[(pos.y - self.chunk_height.min_y) as usize - 1])
                        + relative.y.powi(2) / 6.0
                        >= 1.0
                        || carving_mask.carve(pos)
                    {
                        continue;
                    }
                    clear_overworld_cave_block(
                        &self.unreplaceable,
                        chunk,
                        surface,
                        biome_accessor,
                        biome_noise,
                        &mut surface_reached,
                        pos,
                    )
                }
            }
        }
    }
}
