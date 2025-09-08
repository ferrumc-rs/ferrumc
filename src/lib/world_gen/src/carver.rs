use std::{
    f32::consts::{FRAC_PI_2, PI},
    ops::Range,
};

use bevy_math::{DVec3, IVec3, Vec3Swizzles};
use ferrumc_world::{block_id::BlockId, vanilla_chunk_format::BlockData};

use crate::{
    NoiseGeneratorSettings, NoiseSettings,
    aquifier::{ChunkPos, FluidType, compute_substance},
    random::{LegacyRandom, RandomState, Rng},
    surface::{BiomeManager, top_material},
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
    pub(crate) fn new(settengs: &NoiseSettings) -> Self {
        Self {
            min_y: settengs.min_y,
            carved: vec![false; ((settengs.height as i32 - settengs.min_y) * 16 * 16) as usize],
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
    random: &RandomState,
    settings: &NoiseGeneratorSettings,
    chunk_pos: ChunkPos,
    chunk: &mut ChunkAccess,
    biome_accessor: &BiomeManager,
    pos: DVec3,
    horizontal_radius: f64,
    vertical_radius: f64,
    carving_mask: &mut CarvingMask,
    skip_checker: &impl Fn(f64, f64, f64, i32) -> bool,
    unreplaceable: &[BlockId],
) {
    if (chunk_pos.column_pos(8, 8).pos.as_dvec2() - pos.xz())
        .abs()
        .max_element()
        > 16.0 + horizontal_radius * 2.0
    {
        return;
    }

    let min_block = chunk_pos.column_pos(0, 0);

    let max = ((pos.x - horizontal_radius).floor() as i32 - min_block.pos.x - 1).max(0);
    let min = ((pos.x + horizontal_radius).floor() as i32 - min_block.pos.x).min(15);
    let max1 =
        ((pos.y - vertical_radius).floor() as i32 - 1).max(settings.noise_settings.min_y + 1);
    let min1 = ((pos.y + vertical_radius).floor() as i32 + 1)
        .min(settings.noise_settings.min_y + settings.noise_settings.height as i32 - 1 - 7);
    let max2 = ((pos.z - horizontal_radius).floor() as i32 - min_block.pos.y - 1).max(0);
    let min2 = ((pos.z + horizontal_radius).floor() as i32 - min_block.pos.y).min(15);

    for i1 in max..=min {
        let block_x = chunk_pos.pos.x + i1;
        let d3 = (f64::from(block_x) + 0.5 - pos.x) / horizontal_radius;

        for i2 in max2..=min2 {
            let block_z = chunk_pos.pos.y + i2;
            let d4 = (f64::from(block_z) + 0.5 - pos.z) / horizontal_radius;

            if d3 * d3 + d4 * d4 >= 1.0 {
                continue;
            }
            let mut surface_reached = false;

            for i3 in (max1..=min1).rev() {
                let d5 = (f64::from(i3) - 0.5 - pos.y) / vertical_radius;

                if skip_checker(d3, d5, d4, i3) || carving_mask.carve((i1, i3, i2).into()) {
                    continue;
                }
                let pos = (block_x, i3, block_z).into();
                let block = chunk.get_block_state(pos);

                if unreplaceable.contains(&block.to_block_id()) {
                    continue;
                }

                if block.name == "minecraft:grass_block" || block.name == "minecraft:mycelium" {
                    surface_reached = true;
                }

                if let (Some(carve_state), fluid_update /* TODO */) =
                    compute_substance(random, settings, pos, 0.0)
                {
                    chunk.set_block_state(pos, carve_state.into());
                    if surface_reached {
                        let check_pos = pos - IVec3::new(0, 1, 0);
                        if chunk.get_block_state(check_pos).name == "minecraft:dirt"
                            && let Some(block_state1) = top_material(
                                &settings.rule_source,
                                biome_accessor.get_biome(check_pos),
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
        }
    }
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

pub(crate) struct CarveSettings {
    y: Range<i32>,
    horizontal_radius_mul: Range<f32>,
    vertical_radius_mul: Range<f32>,
    floor_level: Range<f32>,
    y_scale: Range<f32>,
}

#[allow(dead_code)]
pub(crate) fn carve(
    random_state: &RandomState,
    settings: &NoiseGeneratorSettings,
    unreplaceable: &[BlockId],
    chunk: &mut ChunkAccess,
    biome_accessor: &BiomeManager,
    seed: u64,
    chunk_pos: ChunkPos,
    carving_mask: &mut CarvingMask,
    carve_settings: CarveSettings,
) {
    let mut random = LegacyRandom::large_features(seed, chunk_pos);
    let block_pos_coord = (4 * 2 - 1) << 4;
    let bound = random.next_bounded(16) + 1;
    let bound1 = random.next_bounded(bound);
    for _ in 0..random.next_bounded(bound1) {
        let random_pos = chunk_pos.block(
            random.next_bounded(16),
            random.next_i32_range(carve_settings.y.clone()),
            random.next_bounded(16),
        );
        let horizontal_radius_mul =
            random.next_f32_range(carve_settings.horizontal_radius_mul.clone());
        let vertical_radius_mul = random.next_f32_range(carve_settings.vertical_radius_mul.clone());
        let floor_level = random
            .next_f32_range(carve_settings.floor_level.clone())
            .into();

        let tunnels = if random.next_bounded(4) == 0 {
            let y_scale = f64::from(random.next_f32_range(carve_settings.y_scale.clone()));
            let radius = f64::from(1.0 + random.next_f32() * 6.0);
            carve_ellipsoid(
                random_state,
                settings,
                chunk_pos,
                chunk,
                biome_accessor,
                random_pos.as_dvec3() + DVec3::from((1.0, 0.0, 0.0)),
                1.5 + f64::from((FRAC_PI_2).sin()) * radius,
                (1.5 + f64::from((FRAC_PI_2).sin()) * radius) * y_scale,
                carving_mask,
                &|relative_x, relative_y, relative_z, _skip_y| {
                    should_skip(relative_x, relative_y, relative_z, floor_level)
                },
                unreplaceable,
            );
            random.next_bounded(4) + 1
        } else {
            1
        };

        for _ in 0..tunnels {
            let f1 = random.next_f32() * (PI * 2.0);
            let f = (random.next_f32() - 0.5) / 4.0;
            let thickness = random.next_f32() * 2.0
                + random.next_f32()
                    * if random.next_bounded(10) == 0 {
                        random.next_f32() * random.next_f32() * 3.0 + 1.0
                    } else {
                        1.0
                    };
            let branch_count = block_pos_coord - random.next_bounded(block_pos_coord / 4);

            create_tunnel(
                random_state,
                settings,
                chunk,
                biome_accessor,
                random.next_u64(),
                chunk_pos,
                random_pos.into(),
                horizontal_radius_mul.into(),
                vertical_radius_mul.into(),
                thickness,
                f1,
                f,
                0,
                branch_count,
                1.0,
                carving_mask,
                &|relative_x, relative_y, relative_z, _skip_y| {
                    should_skip(relative_x, relative_y, relative_z, floor_level)
                },
                unreplaceable,
            );
        }
    }
}

fn create_tunnel(
    random_state: &RandomState,
    settings: &NoiseGeneratorSettings,
    chunk: &mut ChunkAccess,
    biome_accessor: &BiomeManager,
    seed: u64,
    chunk_pos: ChunkPos,
    mut pos: DVec3,
    horizontal_radius_multiplier: f64,
    vertical_radius_multiplier: f64,
    thickness: f32,
    mut yaw: f32,
    mut pitch: f32,
    branch_index: u32,
    branch_count: u32,
    horizontal_vertical_ratio: f64,
    carving_mask: &mut CarvingMask,
    skip_checker: &impl Fn(f64, f64, f64, i32) -> bool,
    unreplaceable: &[BlockId],
) {
    let mut random_source = LegacyRandom::new(seed);
    let i = random_source.next_bounded(branch_count / 2) + branch_count / 4;
    let flag = random_source.next_bounded(6) == 0;
    let mut f = 0.0f32;
    let mut f1 = 0.0f32;

    for i1 in branch_index..branch_count {
        let d =
            1.5 + f64::from((PI * i1 as f32 / branch_count as f32).sin()) * f64::from(thickness);
        let d1 = d * horizontal_vertical_ratio;

        let cos = pitch.cos();
        pos.x += f64::from(yaw.cos() * cos);
        pos.y += f64::from(pitch.sin());
        pos.z += f64::from(yaw.sin() * cos);

        pitch *= if flag { 0.92 } else { 0.7 };
        pitch += f1 * 0.1;
        yaw += f * 0.1;

        f1 *= 0.9;
        f *= 0.75;

        f1 +=
            (random_source.next_f32() - random_source.next_f32()) * random_source.next_f32() * 2.0;
        f += (random_source.next_f32() - random_source.next_f32()) * random_source.next_f32() * 4.0;

        if i1 == i && thickness > 1.0 {
            create_tunnel(
                random_state,
                settings,
                chunk,
                biome_accessor,
                random_source.next_u64(),
                chunk_pos,
                pos,
                horizontal_radius_multiplier,
                vertical_radius_multiplier,
                random_source.next_f32() * 0.5 + 0.5,
                yaw - std::f32::consts::FRAC_PI_2,
                pitch / 3.0,
                i1,
                branch_count,
                1.0,
                carving_mask,
                skip_checker,
                unreplaceable,
            );
            create_tunnel(
                random_state,
                settings,
                chunk,
                biome_accessor,
                random_source.next_u64(),
                chunk_pos,
                pos,
                horizontal_radius_multiplier,
                vertical_radius_multiplier,
                random_source.next_f32() * 0.5 + 0.5,
                yaw + std::f32::consts::FRAC_PI_2,
                pitch / 3.0,
                i1,
                branch_count,
                1.0,
                carving_mask,
                skip_checker,
                unreplaceable,
            );
            return;
        }

        if random_source.next_bounded(4) != 0 {
            if !can_reach(chunk_pos, pos, i1, branch_count, thickness) {
                return;
            }

            carve_ellipsoid(
                random_state,
                settings,
                chunk_pos,
                chunk,
                biome_accessor,
                pos,
                d * horizontal_radius_multiplier,
                d1 * vertical_radius_multiplier,
                carving_mask,
                skip_checker,
                unreplaceable,
            );
        }
    }
}

fn should_skip(relative: f64, relative_y: f64, relative_z: f64, min_relative_y: f64) -> bool {
    relative_y <= min_relative_y
        || (relative * relative + relative_y * relative_y + relative_z * relative_z) >= 1.0
}

#[allow(dead_code)]
pub(crate) fn carve_canyon(
    random_state: &RandomState,
    settings: &NoiseGeneratorSettings,
    chunk: &mut ChunkAccess,
    biome_accessor: &BiomeManager,
    seed: u64,
    chunk_pos: ChunkPos,
    carving_mask: &mut CarvingMask,
    unreplaceable: &[BlockId],
) {
    const WIDTH_SMOOTHNESS: u32 = 3;
    const VERTICAL_RADIUS_DEFAULT_FACTOR: f64 = 1.0;
    const VERTICAL_RADIUS_CENTER_FACTOR: f64 = 0.0;
    const Y_SCALE: f64 = 3.0;
    let mut random = LegacyRandom::large_features(seed, chunk_pos);
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
    let gen_depth = settings.noise_settings.height;
    let mut width_factors = vec![0.0; gen_depth as usize];
    let mut f = 1.0f32;

    for i in 0..gen_depth {
        if i == 0 || random.next_bounded(WIDTH_SMOOTHNESS) == 0 {
            f = 1.0 + random.next_f32() * random.next_f32();
        }
        width_factors[i as usize] = f * f;
    }

    let mut f = 0.0f32;
    let mut f1 = 0.0f32;

    for i in 0..branch_count {
        let mut horizontal_radius = 1.5
            + f64::from((i as f32 * std::f32::consts::PI / branch_count as f32).sin())
                * f64::from(thickness);

        horizontal_radius *= f64::from(random.next_f32_range(0.75..1.0));
        let vertical_radius = (VERTICAL_RADIUS_DEFAULT_FACTOR
            + VERTICAL_RADIUS_CENTER_FACTOR
                * (1.0 - ((0.5 - f64::from(i) / f64::from(branch_count)).abs()) * 2.0))
            * horizontal_radius
            * Y_SCALE
            * f64::from(random.next_f32() * (1.0 - 0.75) + 0.75);

        let cos = pitch.cos();
        let sin = pitch.sin();

        random_pos.x += f64::from(yaw.cos() * cos);
        random_pos.y += f64::from(sin);
        random_pos.z += f64::from(yaw.sin() * cos);

        pitch *= 0.7;
        pitch += f1 * 0.05;
        yaw += f * 0.05;

        f1 *= 0.8;
        f *= 0.5;

        f1 += (random.next_f32() - random.next_f32()) * random.next_f32() * 2.0;
        f += (random.next_f32() - random.next_f32()) * random.next_f32() * 4.0;

        if random.next_bounded(4) != 0 {
            if !can_reach(chunk_pos, random_pos, i, branch_count, thickness) {
                return;
            }

            carve_ellipsoid(
                random_state,
                settings,
                chunk_pos,
                chunk,
                biome_accessor,
                random_pos,
                horizontal_radius,
                vertical_radius,
                carving_mask,
                &|relative_x, relative_y, relative_z, skip_y| {
                    (relative_x * relative_x + relative_z * relative_z)
                        * f64::from(
                            width_factors[(skip_y - settings.noise_settings.min_y) as usize - 1],
                        )
                        + relative_y * relative_y / 6.0
                        >= 1.0
                },
                unreplaceable,
            );
        }
    }
}
