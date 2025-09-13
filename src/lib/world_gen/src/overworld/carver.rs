use crate::{
    common::carver::{Caver, can_reach},
    overworld::noise_depth::OverworldBiomeNoise,
};
use std::f32::consts::PI;

use bevy_math::{IVec3, Vec3Swizzles};
use ferrumc_world::block_id::BlockId;

use crate::{
    ChunkAccess,
    biome_chunk::BiomeChunk,
    common::{
        aquifer::FluidType,
        carver::{CarvingMask, carve_ellipsoid},
    },
    overworld::surface::OverworldSurface,
    pos::{BlockPos, ChunkHeight, ChunkPos},
    random::{LegacyRandom, Rng},
};

pub(super) struct OverworldCarver {
    // cave_carver: Caver,
    // extra_cave_carver: Caver,
    // canyon_carver: CanyonCarver,
} //TODO

impl OverworldCarver {
    pub fn new() -> Self {
        Self {}
    } //TODO
}

fn clear_overworld_cave_block(
    unreplaceable: &[BlockId],
    chunk: &mut ChunkAccess,
    surface: &OverworldSurface,
    biome_accessor: &BiomeChunk,
    biome_noise: &OverworldBiomeNoise,
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
            .at(&surface.surface.preliminary_surface, biome_noise, pos, 0.0)
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

impl Caver {
    #[allow(dead_code)]
    fn carve_overworld(
        &self,
        chunk: &mut ChunkAccess,
        biome_accessor: &BiomeChunk,
        seed: u64,
        chunk_pos: ChunkPos,
        carving_mask: &mut CarvingMask,
        surface: &OverworldSurface,
        biome_noise: &OverworldBiomeNoise,
    ) {
        self.carve(
            &mut |pos, surface_reached| {
                clear_overworld_cave_block(
                    &self.unreplaceable,
                    chunk,
                    surface,
                    biome_accessor,
                    biome_noise,
                    surface_reached,
                    pos,
                )
            },
            |random: &mut LegacyRandom| {
                random.next_f32() * 2.0//TODO: different in the nether 
                    + random.next_f32()
                        * if random.next_bounded(10) == 0 {
                            random.next_f32() * random.next_f32() * 3.0 + 1.0
                        } else {
                            1.0
                        }
            },
            seed,
            chunk_pos,
            carving_mask,
        );
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
        surface: &OverworldSurface,
        biome_noise: &OverworldBiomeNoise,
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
