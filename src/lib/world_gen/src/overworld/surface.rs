use crate::biome::Precipitation;
use crate::common::math::lerp2;
use std::range::Range;

use crate::common::surface::Surface;
use crate::overworld::aquifer::{Aquifer, SEA_LEVEL};
use crate::overworld::noise_depth::OverworldBiomeNoise;
use crate::overworld::ore_veins::Vein;
use crate::perlin_noise::{
    BADLANDS_PILLAR, BADLANDS_PILLAR_ROOF, BADLANDS_SURFACE, CALCITE, CLAY_BANDS_OFFSET, GRAVEL,
    ICE, ICEBERG_PILLAR, ICEBERG_PILLAR_ROOF, ICEBERG_SURFACE, PACKED_ICE, POWDER_SNOW, SURFACE,
    SURFACE_SECONDARY, SWAMP,
};
use crate::pos::{BlockPos, ChunkHeight};
use crate::random::Xoroshiro128PlusPlus;
use crate::{ChunkAccess, HeightmapType};
use crate::{biome_chunk::BiomeChunk, pos::ColumnPos};
use bevy_math::{DVec2, FloatExt, IVec2, IVec3};
use ferrumc_macros::{block, match_block};
use ferrumc_world::block_id::BlockId;

use crate::{biome::Biome, perlin_noise::NormalNoise, random::Rng};

pub struct SurfaceNoises {
    surface: NormalNoise<3>,
    surface_secondary: NormalNoise<4>,
    clay_bands_offset: NormalNoise<1>,
    swamp: NormalNoise<1>,
    packed_ice: NormalNoise<4>,
    ice: NormalNoise<4>,
    powder_snow: NormalNoise<4>,
    calcite: NormalNoise<4>,
    gravel: NormalNoise<4>,
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
    factory: Xoroshiro128PlusPlus,
    rules: SurfaceRules,
}

impl OverworldSurface {
    pub fn new(factory: Xoroshiro128PlusPlus, chunk_height: ChunkHeight) -> Self {
        Self {
            surface: Surface::new(block!("stone"), chunk_height),
            rules: SurfaceRules::new(factory),
            aquifer: Aquifer::new(factory),
            noises: SurfaceNoises {
                surface: SURFACE.init(factory),
                surface_secondary: SURFACE_SECONDARY.init(factory),
                clay_bands_offset: CLAY_BANDS_OFFSET.init(factory),
                swamp: SWAMP.init(factory),
                packed_ice: PACKED_ICE.init(factory),
                ice: ICE.init(factory),
                powder_snow: POWDER_SNOW.init(factory),
                calcite: CALCITE.init(factory),
                gravel: GRAVEL.init(factory),
                iceberg_surface_noise: ICEBERG_SURFACE.init(factory),
                iceberg_pillar_noise: ICEBERG_PILLAR.init(factory),
                iceberg_pillar_roof_noise: ICEBERG_PILLAR_ROOF.init(factory),
                badlands_surface_noise: BADLANDS_SURFACE.init(factory),
                badlands_pillar_noise: BADLANDS_PILLAR.init(factory),
                badlands_pillar_roof_noise: BADLANDS_PILLAR_ROOF.init(factory),
            },
            vein: Vein::new(factory),
            factory,
        }
    }

    pub fn build_surface(
        &self,
        biome_noise: &OverworldBiomeNoise,
        chunk: &ChunkAccess,
        biome_manager: &BiomeChunk,
        pos: ColumnPos,
    ) -> Vec<BlockId> {
        let (stone_level, fluid_level) = self.surface.find_surface(pos, |pos, final_density| {
            self.aquifer.at(biome_noise, pos, final_density).0 //TODO
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
            |biome: Biome,
             depth_above: i32,
             depth_below: i32,
             fluid_level: Option<i32>,
             pos: BlockPos| {
                self.rules
                    .try_apply(
                        chunk,
                        self,
                        biome_noise,
                        biome,
                        depth_above,
                        depth_below,
                        fluid_level,
                        pos,
                    )
                    .to_block()
            },
            |pos, final_density| {
                (pos.y < stone_level).then_some(()).and_then(
                    |()| self.aquifer.at(biome_noise, pos, final_density).0, //TODO
                )
            },
        ); //TODO: add Vein to rules

        if matches!(biome, Biome::FrozenOcean | Biome::DeepFrozenOcean) {
            self.frozen_ocean_extension(
                biome_noise,
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
        noise: &OverworldBiomeNoise,
        pos: ColumnPos,
        biome: Biome,
        block_column: &mut [BlockId],
        height: i32,
    ) {
        fn should_melt_frozen_ocean_iceberg_slightly(
            biome: Biome,
            pos: BlockPos,
            sea_level: i32,
        ) -> bool {
            biome.block_temperature(pos, sea_level) > 0.1
        }
        let min_surface_level = self.min_surface_level(noise, pos);
        let min_y = self.surface.chunk_height.min_y;
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

            if should_melt_frozen_ocean_iceberg_slightly(biome, pos.block(SEA_LEVEL), SEA_LEVEL) {
                iceburg_height -= 2.0;
            }

            let (d3, d4) = if iceburg_height > 2.0 {
                (
                    f64::from(SEA_LEVEL) - iceburg_height - 7.0,
                    f64::from(SEA_LEVEL) + iceburg_height,
                )
            } else {
                (0.0, 0.0)
            };

            let mut rng = self.factory.at(pos.block(0));
            let max_snow_blocks = 2 + rng.next_bounded(4);
            let min_snow_block_y = SEA_LEVEL + 18 + rng.next_bounded(10) as i32;
            let mut snow_blocks = 0;

            for y in (min_surface_level..=height.max(iceburg_height as i32 + 1)).rev() {
                let block = block_column[(y + min_y) as usize];

                let cond_air =
                    match_block!("air", block) && f64::from(y) < d4 && rng.next_f64() > 0.01;
                let cond_water = match_block!("water", block)
                    && f64::from(y) > d3
                    && y < SEA_LEVEL
                    && d3 != 0.0
                    && rng.next_f64() > 0.15;

                if cond_air || cond_water {
                    if snow_blocks <= max_snow_blocks && y > min_snow_block_y {
                        block_column[(y + min_y) as usize] = block!("snow_block");
                        snow_blocks += 1;
                    } else {
                        block_column[(y + min_y) as usize] = block!("packed_ice");
                    }
                }
            }
        }
    }

    pub fn min_surface_level(&self, noise: &OverworldBiomeNoise, pos: ColumnPos) -> i32 {
        let chunk = pos.chunk();
        lerp2(
            DVec2::from(pos.pos & 15) / 16.0,
            f64::from(noise.preliminary_surface(chunk)),
            f64::from(noise.preliminary_surface((chunk.pos + IVec2::new(16, 0)).into())),
            f64::from(noise.preliminary_surface((chunk.pos + IVec2::new(0, 16)).into())),
            f64::from(noise.preliminary_surface((chunk.pos + IVec2::new(16, 16)).into())),
        ) as i32
            + self.get_surface_depth(pos)
            - 8
    }

    fn get_surface_depth(&self, pos: ColumnPos) -> i32 {
        let pos = pos.block(0);
        (self.noises.surface.at(pos.as_dvec3()) * 2.75
            + 3.0
            + self.factory.at(pos).next_f64() * 0.25) as i32
    }

    pub(crate) fn top_material(
        &self,
        chunk: &ChunkAccess,
        biome_noise: &OverworldBiomeNoise,
        biome: Biome,
        pos: IVec3,
        is_fluid: bool,
    ) -> Option<BlockId> {
        self.rules
            .try_apply(
                chunk,
                self,
                biome_noise,
                biome,
                1,
                1,
                if is_fluid { Some(pos.y + 1) } else { None },
                pos,
            )
            .to_block()
    }
}

struct SurfaceRules {
    factory: Xoroshiro128PlusPlus,
    bedrock: Xoroshiro128PlusPlus,
    deepslate: Xoroshiro128PlusPlus,
    clay_bands: [SurfaceBlock; 192],
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum SurfaceBlock {
    Air,
    Bedrock,
    WhiteTerracotta,
    OrangeTerracotta,
    Terracotta,
    YellowTerracotta,
    BrownTerracotta,
    RedTerracotta,
    LightGrayTerracotta,
    RedSand,
    RedSandstone,
    Stone,
    Deepslate,
    Dirt,
    Podzol,
    CoarseDirt,
    Mycelium,
    GrassBlock,
    Calcite,
    Gravel,
    Sand,
    Sandstone,
    PackedIce,
    SnowBlock,
    Mud,
    PowderSnow,
    Ice,
    Water,
    Lava,
    Netherrack,
    SoulSand,
    SoulSoil,
    Basalt,
    Blackstone,
    WarpedWartBlock,
    WarpedNylium,
    NetherWartBlock,
    CrimsonNylium,
    Endstone,
}
impl SurfaceBlock {
    fn to_block(self) -> Option<BlockId> {
        if self == SurfaceBlock::Stone {
            return None;
        }
        Some(match self {
            SurfaceBlock::Air => todo!(),
            SurfaceBlock::Bedrock => todo!(),
            SurfaceBlock::WhiteTerracotta => todo!(),
            SurfaceBlock::OrangeTerracotta => todo!(),
            SurfaceBlock::Terracotta => todo!(),
            SurfaceBlock::YellowTerracotta => todo!(),
            SurfaceBlock::BrownTerracotta => todo!(),
            SurfaceBlock::RedTerracotta => todo!(),
            SurfaceBlock::LightGrayTerracotta => todo!(),
            SurfaceBlock::RedSand => todo!(),
            SurfaceBlock::RedSandstone => todo!(),
            SurfaceBlock::Stone => todo!(),
            SurfaceBlock::Deepslate => todo!(),
            SurfaceBlock::Dirt => todo!(),
            SurfaceBlock::Podzol => todo!(),
            SurfaceBlock::CoarseDirt => todo!(),
            SurfaceBlock::Mycelium => todo!(),
            SurfaceBlock::GrassBlock => todo!(),
            SurfaceBlock::Calcite => todo!(),
            SurfaceBlock::Gravel => todo!(),
            SurfaceBlock::Sand => todo!(),
            SurfaceBlock::Sandstone => todo!(),
            SurfaceBlock::PackedIce => todo!(),
            SurfaceBlock::SnowBlock => todo!(),
            SurfaceBlock::Mud => todo!(),
            SurfaceBlock::PowderSnow => todo!(),
            SurfaceBlock::Ice => todo!(),
            SurfaceBlock::Water => todo!(),
            SurfaceBlock::Lava => todo!(),
            SurfaceBlock::Netherrack => todo!(),
            SurfaceBlock::SoulSand => todo!(),
            SurfaceBlock::SoulSoil => todo!(),
            SurfaceBlock::Basalt => todo!(),
            SurfaceBlock::Blackstone => todo!(),
            SurfaceBlock::WarpedWartBlock => todo!(),
            SurfaceBlock::WarpedNylium => todo!(),
            SurfaceBlock::NetherWartBlock => todo!(),
            SurfaceBlock::CrimsonNylium => todo!(),
            SurfaceBlock::Endstone => todo!(),
        })
    }
}
impl SurfaceRules {
    fn new(factory: Xoroshiro128PlusPlus) -> Self {
        Self {
            factory,
            bedrock: factory.with_hash("minecraft:bedrock_floor").fork(),
            deepslate: factory.with_hash("minecraft:deepslate").fork(),
            clay_bands: badlands_clay(factory),
        }
    }

    pub fn try_apply(
        &self,
        chunk: &ChunkAccess,
        surface: &OverworldSurface,
        biome_noise: &OverworldBiomeNoise,
        biome: Biome,
        depth_above: i32,
        depth_below: i32,
        fluid_level: Option<i32>,
        pos: BlockPos,
    ) -> SurfaceBlock {
        use SurfaceBlock::*;
        //bedrock
        if pos.y == -64
            || pos.y < -64 + 5
                && self.bedrock.at(pos).next_f32()
                    < f64::from(pos.y).remap(-64.0, -64.0 + 5.0, 1.0, 0.0) as f32
        {
            return Bedrock;
        }

        if pos.y >= surface.min_surface_level(biome_noise, pos.into()) {
            let surface_noise = surface.noises.surface.at(pos.with_y(0).into());
            let surface_depth =
                surface_noise * 2.75 + 3.0 + self.factory.at(pos.with_y(0)).next_f64() * 0.25;

            if depth_above <= 1 {
                if biome == Biome::WoodedBadlands && f64::from(pos.y) >= 97.0 + surface_depth * 2.0
                {
                    if badlands_noise_condition(surface_noise) {
                        return CoarseDirt;
                    }
                    return if fluid_level.is_none() {
                        GrassBlock
                    } else {
                        Dirt
                    };
                }
                if biome == Biome::Swamp
                    && pos.y == 62
                    && surface.noises.swamp.at(pos.into()) >= 0.0
                {
                    return Water;
                }
                if biome == Biome::MangroveSwamp
                    && pos.y >= 60
                    && pos.y < 63
                    && surface.noises.swamp.at(pos.into()) >= 0.0
                {
                    return Water;
                }
            }
            if matches!(
                biome,
                Biome::Badlands | Biome::ErodedBadlands | Biome::WoodedBadlands
            ) {
                if depth_above <= 1 {
                    if pos.y >= 256 {
                        return OrangeTerracotta;
                    }
                    if f64::from(pos.y) >= 72.0 + f64::from(depth_above) + surface_depth {
                        return if badlands_noise_condition(surface_noise) {
                            Terracotta
                        } else {
                            self.get_badlands_clay(surface, pos)
                        };
                    }
                    if fluid_level.is_none() {
                        return if depth_below <= 1 {
                            RedSandstone
                        } else {
                            RedSand
                        };
                    }
                    if surface_depth > 0.0 {
                        return OrangeTerracotta;
                    }
                    if fluid_level.is_none_or(|f| {
                        f64::from(pos.y) + f64::from(depth_below)
                            >= f64::from(f) - 6.0 - surface_depth
                    }) {
                        return WhiteTerracotta;
                    }
                    return if depth_below <= 1 { Stone } else { Gravel };
                }
                if f64::from(pos.y) + f64::from(depth_above) >= 63.0 - surface_depth {
                    return if pos.y >= 63
                        && f64::from(pos.y) + f64::from(depth_above) < 74.0 - surface_depth
                    {
                        OrangeTerracotta
                    } else {
                        self.get_badlands_clay(surface, pos)
                    };
                }

                if f64::from(depth_above) <= 1.0 + surface_depth
                    && fluid_level.is_none_or(|f| {
                        f64::from(pos.y) + f64::from(depth_below)
                            >= f64::from(f) - 6.0 - surface_depth
                    })
                {
                    return WhiteTerracotta;
                }
            }
            if depth_above <= 1 && fluid_level.is_none_or(|f| pos.y >= f - 1) {
                return if matches!(biome, Biome::DeepFrozenOcean | Biome::FrozenOcean)
                    && surface_depth <= 0.0
                {
                    if fluid_level.is_none() {
                        Air
                    } else if biome.precipitation(pos) == Precipitation::Snow {
                        Ice
                    } else {
                        Water
                    }
                } else {
                    if biome == Biome::FrozenPeaks {
                        if is_steep(pos, chunk) {
                            return PackedIce;
                        }
                        if (0.0..=0.2).contains(&surface.noises.packed_ice.at(pos.with_y(0).into()))
                        {
                            return PackedIce;
                        }
                        if (0.0..=0.025).contains(&surface.noises.ice.at(pos.with_y(0).into())) {
                            return PackedIce;
                        }
                        if fluid_level.is_none() {
                            return SnowBlock;
                        }
                    }
                    if biome == Biome::SnowySlopes {
                        if is_steep(pos, chunk) {
                            return Stone;
                        }
                        if fluid_level.is_none()
                            && (0.45..=0.58)
                                .contains(&surface.noises.powder_snow.at(pos.with_y(0).into()))
                        {
                            return PowderSnow;
                        }
                        if fluid_level.is_none() {
                            return SnowBlock;
                        }
                    }
                    if biome == Biome::JaggedPeaks {
                        if is_steep(pos, chunk) {
                            return Stone;
                        }
                        if fluid_level.is_none() {
                            return SnowBlock;
                        }
                    }
                    if biome == Biome::Grove {
                        if fluid_level.is_none()
                            && (0.45..=0.58)
                                .contains(&surface.noises.powder_snow.at(pos.with_y(0).into()))
                        {
                            return PowderSnow;
                        }
                        if fluid_level.is_none() {
                            return SnowBlock;
                        }
                    }
                    if biome == Biome::WindsweptSavanna {
                        if surface_noise >= 1.75 / 8.25 {
                            return Stone;
                        }
                        if surface_noise >= -0.5 / 8.25 {
                            return CoarseDirt;
                        }
                    }
                    if biome == Biome::WindsweptGravellyHills {
                        return if surface_noise >= 2.0 / 8.25 {
                            if depth_below <= 1 { Stone } else { Gravel }
                        } else if surface_noise >= 1.0 / 8.25 {
                            Stone
                        } else if surface_noise >= -1.0 / 8.25 {
                            if fluid_level.is_none() {
                                GrassBlock
                            } else {
                                Dirt
                            }
                        } else if depth_below <= 1 {
                            Stone
                        } else {
                            Gravel
                        };
                    }
                    if matches!(
                        biome,
                        Biome::OldGrowthPineTaiga | Biome::OldGrowthSpruceTaiga
                    ) {
                        if surface_noise >= 1.75 / 8.25 {
                            return CoarseDirt;
                        }
                        if surface_noise >= -0.95 / 8.25 {
                            return Podzol;
                        }
                    }
                    if biome == Biome::IceSpikes && fluid_level.is_none() {
                        return SnowBlock;
                    }
                    if biome == Biome::MangroveSwamp {
                        return Mud;
                    }
                    if biome == Biome::MushroomFields {
                        return Mycelium;
                    }
                    return if fluid_level.is_none() {
                        GrassBlock
                    } else {
                        Dirt
                    };
                };
            }
            if fluid_level.is_none_or(|f| {
                f64::from(pos.y) + f64::from(depth_below) >= f64::from(f) - 6.0 - surface_depth
            }) {
                if depth_above <= 1
                    && matches!(biome, Biome::DeepFrozenOcean | Biome::FrozenOcean)
                    && surface_depth <= 0.0
                {
                    return Water;
                }
                if f64::from(depth_above) <= 1.0 + surface_depth {
                    if biome == Biome::FrozenPeaks {
                        if is_steep(pos, chunk) {
                            return PackedIce;
                        }
                        if (-0.5..=0.2)
                            .contains(&surface.noises.packed_ice.at(pos.with_y(0).into()))
                        {
                            return PackedIce;
                        }
                        if (-0.0625..=0.025).contains(&surface.noises.ice.at(pos.with_y(0).into()))
                        {
                            return PackedIce;
                        }
                        if fluid_level.is_none() {
                            return SnowBlock;
                        }
                    }
                    if biome == Biome::SnowySlopes {
                        if is_steep(pos, chunk) {
                            return Stone;
                        }
                        if fluid_level.is_none()
                            && (0.45..=0.58)
                                .contains(&surface.noises.powder_snow.at(pos.with_y(0).into()))
                        {
                            return PowderSnow;
                        }
                        if fluid_level.is_none() {
                            return SnowBlock;
                        }
                    }
                    if biome == Biome::JaggedPeaks {
                        return Stone;
                    }
                    if biome == Biome::Grove {
                        return if (0.45..=0.58)
                            .contains(&surface.noises.powder_snow.at(pos.with_y(0).into()))
                        {
                            PowderSnow
                        } else {
                            Dirt
                        };
                    }
                    if biome == Biome::StonyPeaks {
                        return if (-0.0125..=0.0125)
                            .contains(&surface.noises.calcite.at(pos.with_y(0).into()))
                        {
                            Calcite
                        } else {
                            Stone
                        };
                    }
                    if biome == Biome::StonyShore {
                        return if depth_below <= 1
                            && (-0.05..=0.05)
                                .contains(&surface.noises.gravel.at(pos.with_y(0).into()))
                        {
                            Gravel
                        } else {
                            Stone
                        };
                    }
                    if biome == Biome::WindsweptHills && surface_noise >= 1.0 / 8.25 {
                        return Stone;
                    }
                    if matches!(
                        biome,
                        Biome::SnowyBeach | Biome::Beach | Biome::WarmOcean | Biome::Desert
                    ) {
                        return if depth_below <= 1 { Sandstone } else { Sand };
                    }
                    if biome == Biome::DripstoneCaves {
                        return Stone;
                    }
                    if biome == Biome::WindsweptSavanna && surface_noise >= 1.75 / 8.25 {
                        return Stone;
                    }
                    if biome == Biome::WindsweptHills {
                        return if surface_noise >= 2.0 / 8.25 {
                            if depth_below <= 1 { Stone } else { Gravel }
                        } else if surface_noise >= 1.0 / 8.25 {
                            Stone
                        } else if surface_noise >= -1.0 / 8.25 {
                            Dirt
                        } else if depth_below <= 1 {
                            Stone
                        } else {
                            Gravel
                        };
                    }
                    if biome == Biome::MangroveSwamp {
                        return Mud;
                    }
                    return Dirt;
                }
                if matches!(biome, Biome::SnowyBeach | Biome::Beach | Biome::WarmOcean)
                    && (f64::from(depth_above)
                        <= 1.0
                            + surface_depth
                            + surface
                                .noises
                                .surface_secondary
                                .at(pos.with_y(0).into())
                                .remap(-1.0, 1.0, 0.0, 6.0)
                        || f64::from(depth_above)
                            <= 1.0
                                + surface_depth
                                + surface
                                    .noises
                                    .surface_secondary
                                    .at(pos.with_y(0).into())
                                    .remap(-1.0, 1.0, 0.0, 30.0))
                {
                    return Sandstone;
                }
            }
            if depth_above <= 1 {
                return if matches!(biome, Biome::FrozenPeaks | Biome::JaggedPeaks) {
                    Stone
                } else if matches!(
                    biome,
                    Biome::WarmOcean | Biome::DeepLukewarmOcean | Biome::LukewarmOcean
                ) {
                    if depth_below <= 1 { Sandstone } else { Sand }
                } else if depth_below <= 1 {
                    Stone
                } else {
                    Gravel
                };
            }
        }

        if pos.y <= 0
            || pos.y < 8
                && self.deepslate.at(pos).next_f32()
                    < f64::from(pos.y).remap(0.0, 8.0, 1.0, 0.0) as f32
        {
            return Deepslate;
        }

        Stone
    }

    fn get_badlands_clay(&self, surface: &OverworldSurface, pos: BlockPos) -> SurfaceBlock {
        let i = (surface.noises.clay_bands_offset.at(pos.with_y(0).into()) * 4.0).round(); //TODO:
        //this rounding is not the same as in java.
        self.clay_bands
            [(pos.y as usize + i as usize + self.clay_bands.len()) % self.clay_bands.len()]
    }
}
fn badlands_clay(factory: Xoroshiro128PlusPlus) -> [SurfaceBlock; 192] {
    fn make_bands(
        rng: &mut Xoroshiro128PlusPlus,
        output: &mut [SurfaceBlock],
        min_size: usize,
        state: SurfaceBlock,
    ) {
        // Java: nextIntBetweenInclusive(6, 15)
        let band_count = rng.next_i32_range(Range::from(6..16));

        for _ in 0..band_count {
            // Java: minSize + nextInt(3)
            let band_len = min_size + rng.next_bounded(3) as usize;
            // Java: nextInt(output.length)
            let start = rng.next_bounded(output.len() as u32) as usize;

            for offset in 0..band_len {
                let idx = start + offset;
                if idx >= output.len() {
                    break;
                }
                output[idx] = state;
            }
        }
    }
    let mut random = factory.with_hash("minecraft:clay_bands");
    let mut block_states = [SurfaceBlock::Terracotta; 192];

    let mut i = 1 + random.next_bounded(5) as usize;
    while i < block_states.len() {
        block_states[i] = SurfaceBlock::OrangeTerracotta;
        i += 2 + random.next_bounded(5) as usize;
    }

    make_bands(
        &mut random,
        &mut block_states,
        1,
        SurfaceBlock::YellowTerracotta,
    );
    make_bands(
        &mut random,
        &mut block_states,
        2,
        SurfaceBlock::BrownTerracotta,
    );
    make_bands(
        &mut random,
        &mut block_states,
        1,
        SurfaceBlock::RedTerracotta,
    );

    let ix = random.next_i32_range(Range::from(9..16));
    let mut painted = 0;
    let mut i = 0;

    while painted < ix && i < block_states.len() {
        block_states[i] = SurfaceBlock::WhiteTerracotta;

        if i > 0 && random.next_bool() {
            block_states[i - 1] = SurfaceBlock::LightGrayTerracotta;
        }
        if i + 1 < block_states.len() && random.next_bool() {
            block_states[i + 1] = SurfaceBlock::LightGrayTerracotta;
        }

        painted += 1;
        i += random.next_bounded(16) as usize + 4;
    }

    block_states
}

fn badlands_noise_condition(surface_noise: f64) -> bool {
    (-0.909..=-0.5454).contains(&surface_noise)
        || (-0.1818..=0.1818).contains(&surface_noise)
        || (0.5454..=0.909).contains(&surface_noise)
}

fn is_steep(pos: BlockPos, chunk: &ChunkAccess) -> bool {
    let x = pos.x & 15;
    let z = pos.z & 15;

    let max_z = (z - 1).max(0);
    let min_z = (z + 1).min(15);

    let height = chunk.get_height(HeightmapType::WorldSurfaceWg, x, max_z);
    let height1 = chunk.get_height(HeightmapType::WorldSurfaceWg, x, min_z);

    if height1 >= height + 4 {
        true
    } else {
        let max_x = (x - 1).max(0);
        let min_x = (x + 1).min(15);

        let height2 = chunk.get_height(HeightmapType::WorldSurfaceWg, max_x, z);
        let height3 = chunk.get_height(HeightmapType::WorldSurfaceWg, min_x, z);

        height2 >= height3 + 4
    }
}
