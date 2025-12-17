use crate::perlin_noise::ConstPerlinNoise;
use crate::random::LegacyRandom;
use std::sync::LazyLock;

use crate::perlin_noise::PerlinNoise;

use ferrumc_world::pos::BlockPos;

pub static BIOME_INFO_NOISE: LazyLock<PerlinNoise<1>> =
    LazyLock::new(|| ConstPerlinNoise::new(0, [1.0]).legacy_init(&mut LegacyRandom::new(2345)));
pub static TEMPERATURE_NOISE: LazyLock<PerlinNoise<1>> =
    LazyLock::new(|| ConstPerlinNoise::new(0, [1.0]).legacy_init(&mut LegacyRandom::new(1234)));
pub static FROZEN_TEMPERATURE_NOISE: LazyLock<PerlinNoise<3>> = LazyLock::new(|| {
    ConstPerlinNoise::new(-2, [1.0, 1.0, 1.0]).legacy_init(&mut LegacyRandom::new(3456))
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
pub enum Precipitation {
    None,
    Snow,
    Rain,
}

impl Biome {
    pub fn precipitation(&self, pos: BlockPos) -> Precipitation {
        use Biome::*;
        if self.temperature() == 2.0
            || matches!(
                self,
                TheEnd | EndHighlands | EndMidlands | SmallEndIslands | EndBarrens | TheVoid
            )
        {
            Precipitation::None
        } else if self.block_temperature(pos, 64) < 0.15 {
            Precipitation::Snow
        } else {
            Precipitation::Rain
        }
    }

    fn temperature(&self) -> f32 {
        use Biome::*;
        match self {
            Desert | Savanna | SavannaPlateau | WindsweptSavanna | Badlands | ErodedBadlands
            | WoodedBadlands => 2.0,
            StonyPeaks => 1.0,
            Jungle | SparseJungle | BambooJungle => 0.95,
            MushroomFields => 0.9,
            DripstoneCaves | DeepDark | Plains | SunflowerPlains | Swamp | MangroveSwamp
            | Beach => 0.8,
            Forest | FlowerForest | PaleGarden | DarkForest => 0.7,
            BirchForest | OldGrowthBirchForest => 0.6,
            Meadow | CherryGrove | LushCaves | ColdOcean | DeepColdOcean | Ocean | DeepOcean
            | LukewarmOcean | DeepLukewarmOcean | WarmOcean | DeepFrozenOcean | River => 0.5,
            OldGrowthPineTaiga => 0.3,
            Taiga | OldGrowthSpruceTaiga => 0.25,
            WindsweptHills | WindsweptGravellyHills | WindsweptForest | StonyShore => 0.2,
            SnowyBeach => 0.05,
            SnowyPlains | IceSpikes | FrozenOcean | FrozenRiver => 0.0,
            Grove => -0.2,
            SnowySlopes => -0.3,
            SnowyTaiga => -0.5,
            FrozenPeaks | JaggedPeaks => -0.7,
            NetherWastes | WarpedForest | CrimsonForest | SoulSandValley | BasaltDeltas => 2.0,
            TheEnd | EndHighlands | EndMidlands | SmallEndIslands | EndBarrens | TheVoid => 0.5,
        }
    }

    pub fn block_temperature(&self, pos: BlockPos, sea_level: i32) -> f32 {
        let y = pos.y();
        let pos = pos.column().pos.as_dvec2();
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
