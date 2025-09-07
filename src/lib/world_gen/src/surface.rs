use ferrumc_world::vanilla_chunk_format::BlockData;

use crate::{
    NoiseGeneratorSettings,
    aquifier::{ChunkPos, ColumnPos, FluidType, compute_substance, preliminary_surface_level},
    biome::Biome,
    ore_veins::compute_vein_block,
    perlin_noise::{NormalNoise, lerp2},
    random::{RandomState, Rng, RngFactory},
};

struct BlockColumn {
    pos: ColumnPos,
    blocks: Vec<BlockData>,
}
impl BlockColumn {
    fn get_block(&self, y: i32) -> BlockData {
        todo!()
    }

    fn set_block(&self, y: i32, none: BlockData) -> BlockData {
        todo!()
    }
}

fn eroded_badlands_extend_height(pos: ColumnPos, surface_noises: &SurfaceNoises) -> Option<i32> {
    let pos = pos.block(0).as_dvec3();
    let surface = (surface_noises.badlands_surface_noise.get_value(pos) * 8.25)
        .abs()
        .min(surface_noises.badlands_pillar_noise.get_value(pos * 0.2) * 15.0);

    if surface > 0.0 {
        let pillar_roof = (surface_noises
            .badlands_pillar_roof_noise
            .get_value(pos * 0.75)
            * 1.5)
            .abs();
        Some((64.0 + (surface * surface * 2.5).min(pillar_roof * 50.0).ceil() + 24.0) as i32)
    } else {
        None
    }
}

fn frozen_ocean_extension(
    min_surface_level: i32,
    biome: Biome,
    block_column: &mut BlockColumn,
    height: i32,
    sea_level: i32,
    surface_noises: &SurfaceNoises,
    noise_random: &RandomState,
) {
    let min = (surface_noises
        .iceberg_surface_noise
        .get_value(block_column.pos.block(0).as_dvec3())
        * 8.25)
        .abs()
        .min(
            surface_noises
                .iceberg_pillar_noise
                .get_value(block_column.pos.block(0).as_dvec3() * 1.28)
                * 15.0,
        );

    if min > 1.8 {
        let abs = (surface_noises
            .iceberg_pillar_roof_noise
            .get_value(block_column.pos.block(0).as_dvec3() * 1.17)
            * 1.5)
            .abs();
        let mut iceburg_height = (min * min * 1.2).min(abs * 40.0).ceil() + 14.0;

        if biome.should_melt_frozen_ocean_iceberg_slightly(sea_level) {
            iceburg_height -= 2.0;
        }

        let (d3, d4) = if iceburg_height > 2.0 {
            (
                f64::from(sea_level) - iceburg_height - 7.0,
                f64::from(sea_level) + iceburg_height,
            )
        } else {
            (0.0, 0.0)
        };

        let mut rng = noise_random.random.with_pos(block_column.pos.block(0));
        let max_snow_blocks = 2 + rng.next_bounded(4);
        let min_snow_block_y = sea_level + 18 + rng.next_bounded(10) as i32;
        let mut snow_blocks = 0;

        for y in (min_surface_level..=height.max(iceburg_height as i32 + 1)).rev() {
            let block = block_column.get_block(y);

            let cond_air =
                block.name == "minecraft:air" && f64::from(y) < d4 && rng.next_f64() > 0.01;
            let cond_water = block.name == "minecraft:water"
                && f64::from(y) > d3
                && y < sea_level
                && d3 != 0.0
                && rng.next_f64() > 0.15;

            if cond_air || cond_water {
                if snow_blocks <= max_snow_blocks && y > min_snow_block_y {
                    block_column.set_block(
                        y,
                        BlockData {
                            name: "minecraft:snow".to_string(),
                            properties: None,
                        },
                    );
                    snow_blocks += 1;
                } else {
                    block_column.set_block(
                        y,
                        BlockData {
                            name: "minecraft:packed_ice".to_string(),
                            properties: None,
                        },
                    );
                }
            }
        }
    }
}

pub struct SurfaceNoises {
    //TODO
    surface_noise: NormalNoise<3>,
    iceberg_surface_noise: NormalNoise<3>,
    iceberg_pillar_noise: NormalNoise<4>,
    iceberg_pillar_roof_noise: NormalNoise<1>,
    badlands_surface_noise: NormalNoise<3>,
    badlands_pillar_noise: NormalNoise<4>,
    badlands_pillar_roof_noise: NormalNoise<1>,
}

pub struct BiomeManager {}
impl BiomeManager {
    fn get_biome(&self, stone_level: bevy_math::IVec3) -> Biome {
        todo!()
    }
} //TODO

pub struct ChunkAccess {}
impl ChunkAccess {
    fn column(&self, column: ColumnPos) -> &mut BlockColumn {
        todo!()
    }
} //TODO

#[allow(dead_code)]
pub fn build_surface(
    noises: &SurfaceNoises,
    settings: &NoiseGeneratorSettings,
    pos: ChunkPos,
    random: &RandomState,
    biome_manager: &BiomeManager,
    chunk: &mut ChunkAccess,
) {
    let sea_level = settings.sea_level.0;

    for column in pos.iter_columns() {
        let block_column = chunk.column(column);
        let mut stone_level = settings.noise_settings.min_y;
        let mut fluid_level = None;
        for y in settings.noise_settings.min_y..settings.noise_settings.height as i32 {
            let substance = compute_substance(random, settings, column.block(y)).0; //TODO:
            //update
            if substance.is_none() {
                stone_level = y;
                break;
            }
            if substance.is_some_and(|s| s != FluidType::Air) && fluid_level.is_none() {
                fluid_level = Some(y);
            }
        }
        let biome = biome_manager.get_biome(column.block(stone_level + 1));
        let extended_height = if matches!(biome, Biome::ErodedBadlands) && fluid_level.is_none() {
            eroded_badlands_extend_height(column, noises).unwrap_or(stone_level)
        } else {
            stone_level
        };

        let mut depth = 0;
        for y in (settings.noise_settings.min_y..=extended_height).rev() {
            if y < stone_level {
                let substance = compute_substance(random, settings, column.block(y)).0; //TODO:
                //update
                if let Some(sub) = substance {
                    if sub != FluidType::Air && fluid_level.is_none() {
                        fluid_level = Some(y);
                    }
                    continue;
                }
            }
            depth += 1;
            let depth_from_stone = y - extended_height + 1;

            if let Some(new_block) = settings
                .ore_veins_enabled
                .then_some(())
                .and_then(|()| compute_vein_block(random, &settings.noise_router, column.block(y)))
                .or_else(|| {
                    settings.rule_source.try_apply(
                        depth,
                        depth_from_stone,
                        fluid_level,
                        column.block(y),
                    )
                })
            {
                block_column.set_block(y, new_block);
            }
        }

        //TODO: post processing should maybe be moved
        if matches!(biome, Biome::FrozenOcean | Biome::DeepFrozenOcean) {
            frozen_ocean_extension(
                min_surface_level(column, settings, &noises.surface_noise, random),
                biome,
                block_column,
                extended_height + 1,
                sea_level,
                noises,
                random,
            );
        }
    }
}

fn min_surface_level(
    pos: ColumnPos,
    settings: &NoiseGeneratorSettings,
    surface_noise: &NormalNoise<3>,
    random: &RandomState,
) -> i32 {
    lerp2(
        (
            f64::from(pos.pos.x & 15) / 16.0,
            f64::from(pos.pos.y & 15) / 16.0,
        )
            .into(),
        f64::from(preliminary_surface_level(
            pos.chunk().column_pos(0, 0),
            settings,
        )),
        f64::from(preliminary_surface_level(
            pos.chunk().column_pos(16, 0),
            settings,
        )),
        f64::from(preliminary_surface_level(
            pos.chunk().column_pos(0, 16),
            settings,
        )),
        f64::from(preliminary_surface_level(
            pos.chunk().column_pos(16, 16),
            settings,
        )),
    ) as i32
        + get_surface_depth(pos, surface_noise, random)
        - 8
}
fn get_surface_depth(pos: ColumnPos, surface_noise: &NormalNoise<3>, random: &RandomState) -> i32 {
    (surface_noise.get_value(pos.block(0).as_dvec3()) * 2.75
        + 3.0
        + random.random.with_pos(pos.block(0)).next_f64() * 0.25) as i32
}
