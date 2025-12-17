use crate::biome::Precipitation;
use crate::common::math::lerp2;
use crate::overworld::overworld_generator::CHUNK_HEIGHT;
use crate::perlin_noise::ConstNormalNoise;
use std::range::Range;

use crate::biome_chunk::BiomeChunk;
use crate::common::surface::Surface;
use crate::overworld::aquifer::SEA_LEVEL;
use crate::overworld::noise_depth::OverworldBiomeNoise;
use crate::overworld::ore_veins::Vein;

use crate::random::Xoroshiro128PlusPlus;
use bevy_math::FloatExt;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::pos::{BlockPos, ColumnPos};

use crate::{biome::Biome, perlin_noise::NormalNoise, random::Rng};

const SURFACE: ConstNormalNoise<3> = ConstNormalNoise::new("minecraft:surface", -6, [1.0, 1.0, 1.0]);
const SURFACE_SECONDARY: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:surface_secondary", -6, [1.0, 1.0, 0.0, 1.0]);
const CLAY_BANDS_OFFSET: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:clay_bands_offset", -8, [1.0]);
const BADLANDS_PILLAR: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:badlands_pillar", -2, [1.0, 1.0, 1.0, 1.0]);
const BADLANDS_PILLAR_ROOF: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:badlands_pillar_roof", -8, [1.0]);
const BADLANDS_SURFACE: ConstNormalNoise<3> =
    ConstNormalNoise::new("minecraft:badlands_surface", -6, [1.0, 1.0, 1.0]);
const ICEBERG_PILLAR: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:iceberg_pillar", -6, [1.0, 1.0, 1.0, 1.0]);
const ICEBERG_PILLAR_ROOF: ConstNormalNoise<1> =
    ConstNormalNoise::new("minecraft:iceberg_pillar_roof", -3, [1.0]);
const ICEBERG_SURFACE: ConstNormalNoise<3> =
    ConstNormalNoise::new("minecraft:iceberg_surface", -6, [1.0, 1.0, 1.0]);
const SWAMP: ConstNormalNoise<1> = ConstNormalNoise::new("minecraft:surface_swamp", -2, [1.0]);
const CALCITE: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:calcite", -9, [1.0, 1.0, 1.0, 1.0]);
const GRAVEL: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:gravel", -8, [1.0, 1.0, 1.0, 1.0]);
const POWDER_SNOW: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:powder_snow", -6, [1.0, 1.0, 1.0, 1.0]);
const PACKED_ICE: ConstNormalNoise<4> =
    ConstNormalNoise::new("minecraft:packed_ice", -7, [1.0, 1.0, 1.0, 1.0]);
const ICE: ConstNormalNoise<4> = ConstNormalNoise::new("minecraft:ice", -4, [1.0, 1.0, 1.0, 1.0]);

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
    noises: SurfaceNoises,
    vein: Vein,
    factory: Xoroshiro128PlusPlus,
    rules: SurfaceRules,
}

impl OverworldSurface {
    pub fn new(factory: Xoroshiro128PlusPlus) -> Self {
        Self {
            surface: Surface::new(CHUNK_HEIGHT),
            rules: SurfaceRules::new(factory),
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
        chunk: &mut Chunk,
        biome_manager: &BiomeChunk,
        pos: ColumnPos,
    ) {
        let (stone_level, fluid_level) = self.surface.find_surface(chunk, pos);
        let biome = biome_manager.at(pos.block((stone_level + 1).into()));
        let extended_height = if matches!(biome, Biome::ErodedBadlands) && fluid_level.is_none() {
            self.eroded_badlands_extend_height(pos)
                .unwrap_or(stone_level)
        } else {
            stone_level
        };

        self.surface.make_column(
            chunk,
            extended_height,
            fluid_level,
            pos,
            biome,
            |chunk, biome, depth_above, depth_below, fluid_level, pos| {
                self.rules
                    .try_apply(
                        chunk,
                        self,
                        biome_noise,
                        biome,
                        depth_above.into(),
                        depth_below.into(),
                        fluid_level.map(i32::from),
                        pos,
                    )
                    .to_block()
            },
        ); //TODO: add Vein to rules

        if matches!(biome, Biome::FrozenOcean | Biome::DeepFrozenOcean) {
            self.frozen_ocean_extension(pos, biome, chunk);
        }
    }

    fn eroded_badlands_extend_height(&self, pos: ColumnPos) -> Option<i16> {
        let pos = pos.block(0).pos.as_dvec3();
        let surface = (self.noises.badlands_surface_noise.at(pos) * 8.25)
            .abs()
            .min(self.noises.badlands_pillar_noise.at(pos * 0.2) * 15.0);

        if surface > 0.0 {
            let pillar_roof = (self.noises.badlands_pillar_roof_noise.at(pos * 0.75) * 1.5).abs();
            Some((64.0 + (surface * surface * 2.5).min(pillar_roof * 50.0).ceil() + 24.0) as i16)
        } else {
            None
        }
    }

    fn frozen_ocean_extend_height(&self, pos: ColumnPos, biome: Biome) -> Option<i32> {
        let noise_pos = pos.block(0).pos.as_dvec3();
        let surface = (self.noises.iceberg_surface_noise.at(noise_pos) * 8.25)
            .abs()
            .min(self.noises.iceberg_pillar_noise.at(noise_pos * 1.28) * 15.0);
        if surface <= 1.8 {
            return None;
        }
        let abs = (self.noises.iceberg_pillar_roof_noise.at(noise_pos * 1.17) * 1.5).abs();
        let iceburg_height = (surface * surface * 1.2).min(abs * 40.0).ceil() as i32 + 14;

        if biome.block_temperature(pos.block(SEA_LEVEL), SEA_LEVEL) > 0.1 {
            Some(iceburg_height - 2)
        } else {
            Some(iceburg_height)
        }
    }

    fn frozen_ocean_extension(&self, pos: ColumnPos, biome: Biome, block_column: &mut Chunk) {
        if let Some(iceburg_height) = self.frozen_ocean_extend_height(pos, biome) {
            let mut rng = self.factory.at(pos.block(0));
            let max_snow_blocks = 2 + rng.next_bounded(4);
            let min_snow_block_y = SEA_LEVEL + 18 + rng.next_bounded(10) as i32;
            let mut snow_blocks = 0;
            for y in (SEA_LEVEL - iceburg_height - 7..=SEA_LEVEL + iceburg_height).rev() {
                let pos = pos
                    .block((y as i16 - CHUNK_HEIGHT.min_y).into())
                    .chunk_block_pos();
                let block = block_column.get_block(pos).unwrap();
                if block == block!("air") && rng.next_f64() > 0.01
                    || block == block!("water", {level: 0}) && rng.next_f64() > 0.15
                {
                    block_column
                        .set_block(
                            pos,
                            if snow_blocks <= max_snow_blocks && y > min_snow_block_y {
                                snow_blocks += 1;
                                block!("snow_block")
                            } else {
                                block!("packed_ice")
                            },
                        )
                        .unwrap();
                }
            }
        }
    }

    pub fn min_surface_level(&self, noise: &OverworldBiomeNoise, pos: ColumnPos) -> i32 {
        let chunk = pos.chunk();
        lerp2(
            pos.chunk_column_pos().pos.as_dvec2() / 16.0,
            f64::from(noise.preliminary_surface(chunk)),
            f64::from(noise.preliminary_surface(chunk + (1, 0))),
            f64::from(noise.preliminary_surface(chunk + (0, 1))),
            f64::from(noise.preliminary_surface(chunk + (16, 16))),
        ) as i32
            + self.get_surface_depth(pos)
            - 8
    }

    fn get_surface_depth(&self, pos: ColumnPos) -> i32 {
        let pos = pos.block(0);
        (self.noises.surface.at(pos) * 2.75 + 3.0 + self.factory.at(pos).next_f64() * 0.25) as i32
    }

    pub(crate) fn top_material(
        &self,
        chunk: &Chunk,
        biome_noise: &OverworldBiomeNoise,
        biome: Biome,
        pos: BlockPos,
        is_fluid: bool,
    ) -> Option<BlockStateId> {
        self.rules
            .try_apply(
                chunk,
                self,
                biome_noise,
                biome,
                1,
                1,
                if is_fluid { Some(pos.y() + 1) } else { None },
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
    fn to_block(self) -> Option<BlockStateId> {
        if self == SurfaceBlock::Stone {
            return None;
        }
        Some(match self {
            SurfaceBlock::Air => block!("air"),
            SurfaceBlock::Bedrock => block!("bedrock"),
            SurfaceBlock::WhiteTerracotta => block!("white_terracotta"),
            SurfaceBlock::OrangeTerracotta => block!("orange_terracotta"),
            SurfaceBlock::Terracotta => block!("terracotta"),
            SurfaceBlock::YellowTerracotta => block!("yellow_terracotta"),
            SurfaceBlock::BrownTerracotta => block!("brown_terracotta"),
            SurfaceBlock::RedTerracotta => block!("red_terracotta"),
            SurfaceBlock::LightGrayTerracotta => block!("light_gray_terracotta"),
            SurfaceBlock::RedSand => block!("red_sand"),
            SurfaceBlock::RedSandstone => block!("red_sandstone"),
            SurfaceBlock::Stone => block!("stone"),
            SurfaceBlock::Deepslate => block!("deepslate", {axis: "y"}),
            SurfaceBlock::Dirt => block!("dirt"),
            SurfaceBlock::Podzol => block!("podzol", {snowy: false}),
            SurfaceBlock::CoarseDirt => block!("coarse_dirt"),
            SurfaceBlock::Mycelium => block!("mycelium", {snowy: false}),
            SurfaceBlock::GrassBlock => block!("grass_block", {snowy: false}),
            SurfaceBlock::Calcite => block!("calcite"),
            SurfaceBlock::Gravel => block!("gravel"),
            SurfaceBlock::Sand => block!("sand"),
            SurfaceBlock::Sandstone => block!("sandstone"),
            SurfaceBlock::PackedIce => block!("packed_ice"),
            SurfaceBlock::SnowBlock => block!("snow_block"),
            SurfaceBlock::Mud => block!("mud"),
            SurfaceBlock::PowderSnow => block!("powder_snow"),
            SurfaceBlock::Ice => block!("ice"),
            SurfaceBlock::Water => block!("water", {level: 0}),
            SurfaceBlock::Lava => block!("lava", {level: 0}),
            SurfaceBlock::Netherrack => block!("netherrack"),
            SurfaceBlock::SoulSand => block!("soul_sand"),
            SurfaceBlock::SoulSoil => block!("soul_soil"),
            SurfaceBlock::Basalt => block!("basalt", {axis: "y"}),
            SurfaceBlock::Blackstone => block!("blackstone"),
            SurfaceBlock::WarpedWartBlock => block!("warped_wart_block"),
            SurfaceBlock::WarpedNylium => block!("warped_nylium"),
            SurfaceBlock::NetherWartBlock => block!("nether_wart_block"),
            SurfaceBlock::CrimsonNylium => block!("crimson_nylium"),
            SurfaceBlock::Endstone => block!("end_stone"),
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
        chunk: &Chunk,
        surface: &OverworldSurface,
        biome_noise: &OverworldBiomeNoise,
        biome: Biome,
        depth_above: i32,
        depth_below: i32,
        fluid_level: Option<i32>,
        pos: BlockPos,
    ) -> SurfaceBlock {
        use Biome::*;
        use SurfaceBlock::*;
        //bedrock
        if pos.y() == -64
            || pos.y() < -64 + 5
                && self.bedrock.at(pos).next_f32()
                    < f64::from(pos.y()).remap(-64.0, -64.0 + 5.0, 1.0, 0.0) as f32
        {
            return Bedrock;
        }

        if pos.y() >= surface.min_surface_level(biome_noise, pos.column()) {
            let is_on_surface = depth_above <= 1;
            let surface_noise = || surface.noises.surface.at(pos.column().block(0));
            let surface_depth = || {
                surface_noise() * 2.75
                    + 3.0
                    + self.factory.at(pos.column().block(0)).next_f64() * 0.25
            };
            let some_fluid_condition = || {
                fluid_level.is_none_or(|f| {
                    f64::from(pos.y()) + f64::from(depth_below)
                        >= f64::from(f) - 6.0 - surface_depth()
                })
            };
            let fluid_depth = fluid_level.unwrap_or(pos.y()) - pos.y();
            let some_surface_depth_cond = || f64::from(depth_above) <= 1.0 + surface_depth();
            match biome {
                WoodedBadlands => {
                    let surface_noise = surface_noise();
                    if is_on_surface && f64::from(pos.y()) >= 97.0 + surface_depth() * 2.0 {
                        if badlands_noise_condition(surface_noise) {
                            return CoarseDirt;
                        }
                        return if fluid_level.is_none() {
                            GrassBlock
                        } else {
                            Dirt
                        };
                    }
                    if let Some(value) = self.badlands(
                        surface,
                        depth_above,
                        fluid_level,
                        pos,
                        depth_below,
                        is_on_surface,
                        surface_noise,
                        surface_depth(),
                    ) {
                        return value;
                    }
                }
                Swamp => {
                    if is_on_surface && pos.y() == 62 && surface.noises.swamp.at(pos) >= 0.0 {
                        return Water;
                    }
                }
                MangroveSwamp => {
                    if is_on_surface
                        && (60..63).contains(&pos.y())
                        && surface.noises.swamp.at(pos) >= 0.0
                    {
                        return Water;
                    }
                    if is_on_surface && fluid_depth <= 1 {
                        return Mud;
                    }
                }
                Badlands | ErodedBadlands => {
                    if let Some(value) = self.badlands(
                        surface,
                        depth_above,
                        fluid_level,
                        pos,
                        depth_below,
                        is_on_surface,
                        surface_noise(),
                        surface_depth(),
                    ) {
                        return value;
                    }
                }
                DeepFrozenOcean | FrozenOcean => {
                    if is_on_surface && fluid_depth <= 1 && surface_depth() <= 0.0 {
                        return if fluid_level.is_none() {
                            Air
                        } else if biome.precipitation(pos) == Precipitation::Snow {
                            Ice
                        } else {
                            Water
                        };
                    }
                    if some_fluid_condition() && is_on_surface && surface_depth() <= 0.0 {
                        return Water;
                    }
                }
                FrozenPeaks => {
                    if is_on_surface && fluid_depth <= 1 {
                        if is_steep(pos, chunk) {
                            return PackedIce;
                        }
                        if (0.0..=0.2)
                            .contains(&surface.noises.packed_ice.at(pos.column().block(0)))
                        {
                            return PackedIce;
                        }
                        if (0.0..=0.025).contains(&surface.noises.ice.at(pos.column().block(0))) {
                            return PackedIce;
                        }
                        if fluid_depth == 0 {
                            return SnowBlock;
                        }
                    }
                    if is_on_surface {
                        return Stone;
                    }
                    if some_fluid_condition() && some_surface_depth_cond() {
                        if is_steep(pos, chunk) {
                            return PackedIce;
                        }
                        if (-0.5..=0.2)
                            .contains(&surface.noises.packed_ice.at(pos.column().block(0)))
                        {
                            return PackedIce;
                        }
                        if (-0.0625..=0.025).contains(&surface.noises.ice.at(pos.column().block(0)))
                        {
                            return PackedIce;
                        }
                        if fluid_level.is_none() {
                            return SnowBlock;
                        }
                    }
                }
                SnowySlopes => {
                    if is_on_surface && fluid_depth <= 1 {
                        if is_steep(pos, chunk) {
                            return Stone;
                        }
                        if fluid_level.is_none()
                            && (0.45..=0.58)
                                .contains(&surface.noises.powder_snow.at(pos.column().block(0)))
                        {
                            return PowderSnow;
                        }
                        if fluid_level.is_none() {
                            return SnowBlock;
                        }
                    }
                    if some_fluid_condition() && some_surface_depth_cond() {
                        if is_steep(pos, chunk) {
                            return Stone;
                        }
                        if fluid_level.is_none()
                            && (0.45..=0.58)
                                .contains(&surface.noises.powder_snow.at(pos.column().block(0)))
                        {
                            return PowderSnow;
                        }
                        if fluid_level.is_none() {
                            return SnowBlock;
                        }
                    }
                }
                JaggedPeaks => {
                    if is_on_surface && fluid_depth <= 1 {
                        if is_steep(pos, chunk) {
                            return Stone;
                        }
                        if fluid_level.is_none() {
                            return SnowBlock;
                        }
                    }
                    if is_on_surface {
                        return Stone;
                    }
                    if some_fluid_condition() && some_surface_depth_cond() {
                        return Stone;
                    }
                }
                Grove => {
                    if is_on_surface && fluid_depth <= 1 {
                        if fluid_level.is_none()
                            && (0.45..=0.58)
                                .contains(&surface.noises.powder_snow.at(pos.column().block(0)))
                        {
                            return PowderSnow;
                        }
                        if fluid_level.is_none() {
                            return SnowBlock;
                        }
                    }
                    if some_fluid_condition() && some_surface_depth_cond() {
                        return if (0.45..=0.58)
                            .contains(&surface.noises.powder_snow.at(pos.column().block(0)))
                        {
                            PowderSnow
                        } else {
                            Dirt
                        };
                    }
                }
                WindsweptSavanna => {
                    if is_on_surface && fluid_depth <= 1 {
                        if surface_noise() >= 1.75 / 8.25 {
                            return Stone;
                        }
                        if surface_noise() >= -0.5 / 8.25 {
                            return CoarseDirt;
                        }
                    }
                    if some_fluid_condition()
                        && some_surface_depth_cond()
                        && surface_noise() >= 1.75 / 8.25
                    {
                        return Stone;
                    }
                }
                WindsweptGravellyHills => {
                    if is_on_surface && fluid_depth <= 1 {
                        return if surface_noise() >= 2.0 / 8.25 {
                            if depth_below <= 1 { Stone } else { Gravel }
                        } else if surface_noise() >= 1.0 / 8.25 {
                            Stone
                        } else if surface_noise() >= -1.0 / 8.25 {
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
                }
                OldGrowthPineTaiga | OldGrowthSpruceTaiga => {
                    if is_on_surface && fluid_depth <= 1 {
                        if surface_noise() >= 1.75 / 8.25 {
                            return CoarseDirt;
                        }
                        if surface_noise() >= -0.95 / 8.25 {
                            return Podzol;
                        }
                    }
                }
                IceSpikes => {
                    if is_on_surface && fluid_depth == 0 {
                        return SnowBlock;
                    }
                }
                MushroomFields => {
                    if is_on_surface && fluid_depth <= 1 {
                        return Mycelium;
                    }
                }
                WarmOcean => {
                    if is_on_surface {
                        return if depth_below <= 1 { Sandstone } else { Sand };
                    }
                    if some_fluid_condition() && some_surface_depth_cond() {
                        return if depth_below <= 1 { Sandstone } else { Sand };
                    }
                    if some_fluid_condition()
                        && (f64::from(depth_above)
                            <= 1.0
                                + surface_depth()
                                + surface
                                    .noises
                                    .surface_secondary
                                    .at(pos.column().block(0))
                                    .remap(-1.0, 1.0, 0.0, 6.0)
                            || f64::from(depth_above)
                                <= 1.0
                                    + surface_depth()
                                    + surface
                                        .noises
                                        .surface_secondary
                                        .at(pos.column().block(0))
                                        .remap(-1.0, 1.0, 0.0, 30.0))
                    {
                        return Sandstone;
                    }
                }
                DeepLukewarmOcean | LukewarmOcean => {
                    if is_on_surface {
                        return if depth_below <= 1 { Sandstone } else { Sand };
                    }
                }
                StonyPeaks => {
                    if some_fluid_condition() && some_surface_depth_cond() {
                        return if (-0.0125..=0.0125)
                            .contains(&surface.noises.calcite.at(pos.column().block(0)))
                        {
                            Calcite
                        } else {
                            Stone
                        };
                    }
                }
                StonyShore => {
                    if some_fluid_condition() && some_surface_depth_cond() {
                        return if depth_below <= 1
                            && (-0.05..=0.05)
                                .contains(&surface.noises.gravel.at(pos.column().block(0)))
                        {
                            Gravel
                        } else {
                            Stone
                        };
                    }
                }
                WindsweptHills => {
                    if some_fluid_condition()
                        && some_surface_depth_cond()
                        && surface_noise() >= 1.0 / 8.25
                    {
                        return Stone;
                    }
                    if some_fluid_condition() && some_surface_depth_cond() {
                        let surface_noise = surface_noise();
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
                }
                Desert => {
                    if some_fluid_condition() && some_surface_depth_cond() {
                        return if depth_below <= 1 { Sandstone } else { Sand };
                    }
                }
                SnowyBeach | Beach => {
                    if some_fluid_condition() && some_surface_depth_cond() {
                        return if depth_below <= 1 { Sandstone } else { Sand };
                    }
                    if some_fluid_condition()
                        && (f64::from(depth_above)
                            <= 1.0
                                + surface_depth()
                                + surface
                                    .noises
                                    .surface_secondary
                                    .at(pos.column().block(0))
                                    .remap(-1.0, 1.0, 0.0, 6.0)
                            || f64::from(depth_above)
                                <= 1.0
                                    + surface_depth()
                                    + surface
                                        .noises
                                        .surface_secondary
                                        .at(pos.column().block(0))
                                        .remap(-1.0, 1.0, 0.0, 30.0))
                    {
                        return Sandstone;
                    }
                }
                DripstoneCaves => {
                    if some_fluid_condition() && some_surface_depth_cond() {
                        return Stone;
                    }
                }

                _ => {
                    if is_on_surface && fluid_level.is_none_or(|f| pos.y() >= f - 1) {
                        return if fluid_level.is_none() {
                            GrassBlock
                        } else {
                            Dirt
                        };
                    }
                    if some_fluid_condition() {
                        return Dirt;
                    }
                }
            }
        }

        if pos.y() <= 0
            || pos.y() < 8
                && self.deepslate.at(pos).next_f32()
                    < f64::from(pos.y()).remap(0.0, 8.0, 1.0, 0.0) as f32
        {
            return Deepslate;
        }

        Stone
    }

    fn badlands(
        &self,
        surface: &OverworldSurface,
        depth_above: i32,
        fluid_level: Option<i32>,
        pos: BlockPos,
        depth_below: i32,
        is_on_surface: bool,
        surface_noise: f64,
        surface_depth: f64,
    ) -> Option<SurfaceBlock> {
        use SurfaceBlock::*;
        if is_on_surface {
            if pos.y() >= 256 {
                return Some(OrangeTerracotta);
            }
            if f64::from(pos.y()) >= 72.0 + f64::from(depth_above) + surface_depth {
                return Some(if badlands_noise_condition(surface_noise) {
                    Terracotta
                } else {
                    self.get_badlands_clay(surface, pos)
                });
            }
            if fluid_level.is_none() {
                return Some(if depth_below <= 1 {
                    RedSandstone
                } else {
                    RedSand
                });
            }
            if surface_depth > 0.0 {
                return Some(OrangeTerracotta);
            }
            if fluid_level.is_none_or(|f| {
                f64::from(pos.y()) + f64::from(depth_below) >= f64::from(f) - 6.0 - surface_depth
            }) {
                return Some(WhiteTerracotta);
            }
            return Some(if depth_below <= 1 { Stone } else { Gravel });
        }
        if f64::from(pos.y()) + f64::from(depth_above) >= 63.0 - surface_depth {
            return Some(
                if pos.y() >= 63
                    && f64::from(pos.y()) + f64::from(depth_above) < 74.0 - surface_depth
                {
                    OrangeTerracotta
                } else {
                    self.get_badlands_clay(surface, pos)
                },
            );
        }

        if f64::from(depth_above) <= 1.0 + surface_depth
            && fluid_level.is_none_or(|f| {
                f64::from(pos.y()) + f64::from(depth_below) >= f64::from(f) - 6.0 - surface_depth
            })
        {
            return Some(WhiteTerracotta);
        }
        None
    }

    fn get_badlands_clay(&self, surface: &OverworldSurface, pos: BlockPos) -> SurfaceBlock {
        let i = (surface.noises.clay_bands_offset.at(pos.column().block(0)) * 4.0).round(); //TODO:
        //this rounding is not the same as in java.
        self.clay_bands
            [(pos.y() as usize + i as usize + self.clay_bands.len()) % self.clay_bands.len()]
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

fn is_steep(pos: BlockPos, chunk: &Chunk) -> bool {
    return false;
    // let x = pos.x & 15;
    // let z = pos.z & 15;
    //
    // let max_z = (z - 1).max(0);
    // let min_z = (z + 1).min(15);
    //
    // let height = chunk.get_height(HeightmapType::WorldSurfaceWg, x, max_z);
    // let height1 = chunk.get_height(HeightmapType::WorldSurfaceWg, x, min_z);
    //
    // if height1 >= height + 4 {
    //     true
    // } else {
    //     let max_x = (x - 1).max(0);
    //     let min_x = (x + 1).min(15);
    //
    //     let height2 = chunk.get_height(HeightmapType::WorldSurfaceWg, max_x, z);
    //     let height3 = chunk.get_height(HeightmapType::WorldSurfaceWg, min_x, z);
    //
    //     height2 >= height3 + 4
    // }
}
