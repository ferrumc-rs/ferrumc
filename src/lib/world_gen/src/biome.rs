use bevy_math::Vec3Swizzles;

use crate::{
    perlin_noise::{BIOME_INFO_NOISE, FROZEN_TEMPERATURE_NOISE, TEMPERATURE_NOISE},
    pos::BlockPos,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Biome {
    TheVoid,
    Plains,
    SunflowerPlains,
    SnowyPlains,
    IceSpikes,
    Desert,
    Swamp,
    MangroveSwamp,
    Forest,
    FlowerForest,
    BirchForest,
    DarkForest,
    PaleGarden,
    OldGrowthBirchForest,
    OldGrowthPineTaiga,
    OldGrowthSpruceTaiga,
    Taiga,
    SnowyTaiga,
    Savanna,
    SavannaPlateau,
    WindsweptHills,
    WindsweptGravellyHills,
    WindsweptForest,
    WindsweptSavanna,
    Jungle,
    SparseJungle,
    BambooJungle,
    Badlands,
    ErodedBadlands,
    WoodedBadlands,
    Meadow,
    CherryGrove,
    Grove,
    SnowySlopes,
    FrozenPeaks,
    JaggedPeaks,
    StonyPeaks,
    River,
    FrozenRiver,
    Beach,
    SnowyBeach,
    StonyShore,
    WarmOcean,
    LukewarmOcean,
    DeepLukewarmOcean,
    Ocean,
    DeepOcean,
    ColdOcean,
    DeepColdOcean,
    FrozenOcean,
    DeepFrozenOcean,
    MushroomFields,
    DripstoneCaves,
    LushCaves,
    DeepDark,
    NetherWastes,
    WarpedForest,
    CrimsonForest,
    SoulSandValley,
    BasaltDeltas,
    TheEnd,
    EndHighlands,
    EndMidlands,
    SmallEndIslands,
    EndBarrens,
}

impl Biome {
    pub(crate) fn should_melt_frozen_ocean_iceberg_slightly(
        &self,
        pos: BlockPos,
        sea_level: i32,
    ) -> bool {
        self.block_temperature(pos, sea_level) > 0.1
    }

    pub(crate) fn cold_enough_to_snow(&self, pos: BlockPos) -> bool {
        self.block_temperature(pos, 64) < 0.15
    }

    fn temperature(&self) -> f32 {
        todo!("we probably want to parse the biome json data for this")
    }

    pub fn block_temperature(&self, pos: BlockPos, sea_level: i32) -> f32 {
        let y = pos.y;
        let pos = pos.xz().as_dvec2();
        let temp = if *self == Biome::FrozenOcean
            && FROZEN_TEMPERATURE_NOISE.legacy_simplex_at(pos * 0.05) * 7.
                + BIOME_INFO_NOISE.legacy_simplex_at(pos * 0.2)
                < 0.3
            && BIOME_INFO_NOISE.legacy_simplex_at(pos * 0.09) < 0.8
        {
            0.2
        } else {
            self.temperature()
        };
        if y > sea_level + 17 {
            temp - ((TEMPERATURE_NOISE.legacy_simplex_at(pos / 8.) * 8.) as f32 + y as f32
                - (sea_level + 17) as f32)
                * 0.05
                / 40.
        } else {
            temp
        }
    }
}
