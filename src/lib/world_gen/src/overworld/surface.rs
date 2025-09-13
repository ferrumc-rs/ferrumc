use crate::common::aquifer::FluidType;
use crate::common::surface::{PreliminarySurface, Surface, SurfaceRule};
use crate::overworld::aquifer::Aquifer;
use crate::overworld::noise_depth::OverworldBiomeNoise;
use crate::overworld::ore_veins::Vein;
use crate::perlin_noise::{
    BADLANDS_PILLAR, BADLANDS_PILLAR_ROOF, BADLANDS_SURFACE, ICEBERG_PILLAR, ICEBERG_PILLAR_ROOF,
    ICEBERG_SURFACE, SURFACE,
};
use crate::pos::ChunkHeight;
use crate::random::Xoroshiro128PlusPlusFactory;
use crate::{biome_chunk::BiomeChunk, common::aquifer::FluidPicker, pos::ColumnPos};
use bevy_math::{DVec2, IVec2, IVec3};
use ferrumc_world::vanilla_chunk_format::BlockData;

use crate::{
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

pub struct OverworldSurface {
    pub surface: Surface,
    pub aquifer: Aquifer,
    noises: SurfaceNoises,
    vein: Vein,
    random: Xoroshiro128PlusPlusFactory,
}

impl OverworldSurface {
    pub fn new(random: Xoroshiro128PlusPlusFactory, chunk_height: ChunkHeight) -> Self {
        Self {
            surface: Surface::new(
                PreliminarySurface::new(chunk_height, 2 << 2, |_pos| 0.0),
                BlockData {
                    name: "minecraft:stone".to_string(),
                    properties: None,
                }
                .to_block_id(),
                SurfaceRule {}, //TODO:
            ),
            aquifer: Aquifer::new(FluidPicker(63, FluidType::Water), random),
            noises: SurfaceNoises {
                surface_noise: SURFACE.init(random),
                iceberg_surface_noise: ICEBERG_SURFACE.init(random),
                iceberg_pillar_noise: ICEBERG_PILLAR.init(random),
                iceberg_pillar_roof_noise: ICEBERG_PILLAR_ROOF.init(random),
                badlands_surface_noise: BADLANDS_SURFACE.init(random),
                badlands_pillar_noise: BADLANDS_PILLAR.init(random),
                badlands_pillar_roof_noise: BADLANDS_PILLAR_ROOF.init(random),
            },
            vein: Vein::new(random),
            random,
        }
    }

    #[allow(dead_code)]
    pub fn build_surface(
        &self,
        biome_noise: &OverworldBiomeNoise,
        biome_manager: &BiomeChunk,
        pos: ColumnPos,
    ) -> Vec<BlockData> {
        let (stone_level, fluid_level) = self.surface.find_surface(pos, |pos, final_density| {
            self.aquifer
                .at(
                    &self.surface.preliminary_surface,
                    biome_noise,
                    pos,
                    final_density,
                )
                .0 //TODO
        });
        let biome = biome_manager.at(pos.block(stone_level + 1));
        let extended_height = if matches!(biome, Biome::ErodedBadlands) && fluid_level.is_none() {
            self.eroded_badlands_extend_height(pos)
                .unwrap_or(stone_level)
        } else {
            stone_level
        };

        let mut block_column = self.surface.make_column(
            extended_height,
            fluid_level,
            pos,
            biome,
            |pos, final_density| {
                (pos.y < stone_level).then_some(()).and_then(
                    |()| {
                        self.aquifer
                            .at(
                                &self.surface.preliminary_surface,
                                biome_noise,
                                pos,
                                final_density,
                            )
                            .0
                    }, //TODO
                )
            },
        ); //TODO: add Vein to rules

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
        let surface = (self.noises.badlands_surface_noise.at(pos) * 8.25)
            .abs()
            .min(self.noises.badlands_pillar_noise.at(pos * 0.2) * 15.0);

        if surface > 0.0 {
            let pillar_roof = (self.noises.badlands_pillar_roof_noise.at(pos * 0.75) * 1.5).abs();
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
        let min_y = self.surface.preliminary_surface.chunk_height.min_y;
        let min = (self
            .noises
            .iceberg_surface_noise
            .at(pos.block(0).as_dvec3())
            * 8.25)
            .abs()
            .min(
                self.noises
                    .iceberg_pillar_noise
                    .at(pos.block(0).as_dvec3() * 1.28)
                    * 15.0,
            );

        if min > 1.8 {
            let abs = (self
                .noises
                .iceberg_pillar_roof_noise
                .at(pos.block(0).as_dvec3() * 1.17)
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
            f64::from(self.surface.preliminary_surface.at(chunk)),
            f64::from(
                self.surface
                    .preliminary_surface
                    .at((chunk.pos + IVec2::new(16, 0)).into()),
            ),
            f64::from(
                self.surface
                    .preliminary_surface
                    .at((chunk.pos + IVec2::new(0, 16)).into()),
            ),
            f64::from(
                self.surface
                    .preliminary_surface
                    .at((chunk.pos + IVec2::new(16, 16)).into()),
            ),
        ) as i32
            + self.get_surface_depth(pos)
            - 8
    }

    fn get_surface_depth(&self, pos: ColumnPos) -> i32 {
        let pos = pos.block(0);
        (self.noises.surface_noise.at(pos.as_dvec3()) * 2.75
            + 3.0
            + self.random.with_pos(pos).next_f64() * 0.25) as i32
    }

    pub(crate) fn top_material(
        &self,
        biome: Biome,
        pos: IVec3,
        is_fluid: bool,
    ) -> Option<BlockData> {
        self.surface.rules.try_apply(
            biome,
            1,
            1,
            if is_fluid { Some(pos.y + 1) } else { None },
            pos,
        )
    }
}
