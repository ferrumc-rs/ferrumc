use crate::{
    DensityFunction,
    aquifer::{Aquifer, FluidPicker},
    biome_chunk::{BiomeChunk, BiomeNoise},
    overworld::ore_veins::Vein,
    pos::{ChunkHeight, ChunkPos, ColumnPos},
    random::RandomFactory,
};
use bevy_math::{DVec2, IVec2, IVec3};
use ferrumc_world::{block_id::BlockId, vanilla_chunk_format::BlockData};

use crate::{
    SurfaceRule,
    aquifer::FluidType,
    biome::Biome,
    perlin_noise::{NormalNoise, lerp2},
    random::{Rng, RngFactory},
};

pub struct SurfaceNoises {
    surface_noise: NormalNoise<3>,
    iceberg_surface_noise: NormalNoise<3>,
    iceberg_pillar_noise: NormalNoise<4>,
    iceberg_pillar_roof_noise: NormalNoise<1>,
    badlands_surface_noise: NormalNoise<3>,
    badlands_pillar_noise: NormalNoise<4>,
    badlands_pillar_roof_noise: NormalNoise<1>,
}

pub struct Surface {
    pub preliminary_surface: PreliminarySurface,
    pub aquifer: Aquifer,
    default_block: BlockId,
    final_density: DensityFunction,
    noises: SurfaceNoises,
    rules: SurfaceRule,
    vein: Vein,
    random: RandomFactory,
}

impl Surface {
    #[allow(dead_code)]
    pub fn build_surface(
        &self,
        biome_noise: &BiomeNoise,
        biome_manager: &BiomeChunk,
        pos: ColumnPos,
    ) -> Vec<BlockData> {
        let mut stone_level = self.preliminary_surface.chunk_height.min_y - 1;
        let mut fluid_level = None;
        for y in self.preliminary_surface.chunk_height.iter() {
            let substance = self
                .aquifer
                .compute_substance(
                    &self.preliminary_surface,
                    biome_noise,
                    pos.block(y),
                    self.final_density.compute(pos.block(y)),
                )
                .0; //TODO:
            //update
            if substance.is_none() {
                stone_level = y;
                break;
            }
            if substance.is_some_and(|s| s != FluidType::Air) && fluid_level.is_none() {
                fluid_level = Some(y);
            }
        }
        let biome = biome_manager.at(pos.block(stone_level + 1));
        let extended_height = if matches!(biome, Biome::ErodedBadlands) && fluid_level.is_none() {
            self.eroded_badlands_extend_height(pos)
                .unwrap_or(stone_level)
        } else {
            stone_level
        };

        let mut depth = 0;
        let mut block_column: Vec<BlockData> = (self.preliminary_surface.chunk_height.min_y
            ..=extended_height)
            .rev()
            .map(|y| {
                if y < stone_level {
                    let substance = self
                        .aquifer
                        .compute_substance(
                            &self.preliminary_surface,
                            biome_noise,
                            pos.block(y),
                            self.final_density.compute(pos.block(y)),
                        )
                        .0; //TODO:
                    //update
                    if let Some(sub) = substance {
                        if sub != FluidType::Air && fluid_level.is_none() {
                            fluid_level = Some(y);
                        }
                        return sub.into();
                    }
                }
                depth += 1;
                let depth_from_stone = y - extended_height + 1;

                self.vein
                    .at(pos.block(y))
                    .or_else(|| {
                        self.rules.try_apply(
                            biome,
                            depth,
                            depth_from_stone,
                            fluid_level,
                            pos.block(y),
                        )
                    })
                    .unwrap_or(self.default_block.to_block_data().unwrap())
            })
            .rev()
            .collect();

        //TODO: post processing should maybe be moved
        if matches!(biome, Biome::FrozenOcean | Biome::DeepFrozenOcean) {
            self.frozen_ocean_extension(
                self.aquifer.sea_level,
                pos,
                biome,
                &mut block_column,
                extended_height + 1,
            );
        }
        block_column
    }

    fn eroded_badlands_extend_height(&self, pos: ColumnPos) -> Option<i32> {
        let pos = pos.block(0).as_dvec3();
        let surface = (self.noises.badlands_surface_noise.get_value(pos) * 8.25)
            .abs()
            .min(self.noises.badlands_pillar_noise.get_value(pos * 0.2) * 15.0);

        if surface > 0.0 {
            let pillar_roof =
                (self.noises.badlands_pillar_roof_noise.get_value(pos * 0.75) * 1.5).abs();
            Some((64.0 + (surface * surface * 2.5).min(pillar_roof * 50.0).ceil() + 24.0) as i32)
        } else {
            None
        }
    }

    fn frozen_ocean_extension(
        &self,
        sea_level: FluidPicker,
        pos: ColumnPos,
        biome: Biome,
        block_column: &mut [BlockData],
        height: i32,
    ) {
        let min_surface_level = self.min_surface_level(pos);
        let sea_level = sea_level.0;
        let min_y = self.preliminary_surface.chunk_height.min_y;
        let min = (self
            .noises
            .iceberg_surface_noise
            .get_value(pos.block(0).as_dvec3())
            * 8.25)
            .abs()
            .min(
                self.noises
                    .iceberg_pillar_noise
                    .get_value(pos.block(0).as_dvec3() * 1.28)
                    * 15.0,
            );

        if min > 1.8 {
            let abs = (self
                .noises
                .iceberg_pillar_roof_noise
                .get_value(pos.block(0).as_dvec3() * 1.17)
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

            let mut rng = self.random.with_pos(pos.block(0));
            let max_snow_blocks = 2 + rng.next_bounded(4);
            let min_snow_block_y = sea_level + 18 + rng.next_bounded(10) as i32;
            let mut snow_blocks = 0;

            for y in (min_surface_level..=height.max(iceburg_height as i32 + 1)).rev() {
                let block = &block_column[(y + min_y) as usize];

                let cond_air =
                    block.name == "minecraft:air" && f64::from(y) < d4 && rng.next_f64() > 0.01;
                let cond_water = block.name == "minecraft:water"
                    && f64::from(y) > d3
                    && y < sea_level
                    && d3 != 0.0
                    && rng.next_f64() > 0.15;

                if cond_air || cond_water {
                    if snow_blocks <= max_snow_blocks && y > min_snow_block_y {
                        block_column[(y + min_y) as usize] = BlockData {
                            name: "minecraft:snow".to_string(),
                            properties: None,
                        };
                        snow_blocks += 1;
                    } else {
                        block_column[(y + min_y) as usize] = BlockData {
                            name: "minecraft:packed_ice".to_string(),
                            properties: None,
                        };
                    }
                }
            }
        }
    }

    fn min_surface_level(&self, pos: ColumnPos) -> i32 {
        let chunk = pos.chunk();
        lerp2(
            DVec2::from(pos.pos & 15) / 16.0,
            f64::from(self.preliminary_surface.at(chunk)),
            f64::from(
                self.preliminary_surface
                    .at((chunk.pos + IVec2::new(16, 0)).into()),
            ),
            f64::from(
                self.preliminary_surface
                    .at((chunk.pos + IVec2::new(0, 16)).into()),
            ),
            f64::from(
                self.preliminary_surface
                    .at((chunk.pos + IVec2::new(16, 16)).into()),
            ),
        ) as i32
            + self.get_surface_depth(pos)
            - 8
    }

    fn get_surface_depth(&self, pos: ColumnPos) -> i32 {
        let pos = pos.block(0);
        (self.noises.surface_noise.get_value(pos.as_dvec3()) * 2.75
            + 3.0
            + self.random.with_pos(pos).next_f64() * 0.25) as i32
    }

    pub(crate) fn top_material(
        &self,
        biome: Biome,
        pos: IVec3,
        is_fluid: bool,
    ) -> Option<BlockData> {
        self.rules.try_apply(
            biome,
            1,
            1,
            if is_fluid { Some(pos.y + 1) } else { None },
            pos,
        )
    }
}

pub struct PreliminarySurface {
    pub chunk_height: ChunkHeight,
    noise_size_vertical: usize,
    initial_density_without_jaggedness: DensityFunction,
}

impl PreliminarySurface {
    pub(crate) fn at(&self, chunk: ChunkPos) -> i32 {
        let column = chunk.column_pos(0, 0);
        self.chunk_height
            .iter()
            .rev()
            .step_by(self.noise_size_vertical)
            .find(|y| {
                self.initial_density_without_jaggedness
                    .compute(column.block(*y))
                    > 0.390625
            })
            .unwrap_or(i32::MAX) //TODO: should this panic?
    }
}
