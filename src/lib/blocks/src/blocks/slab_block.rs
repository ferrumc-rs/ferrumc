#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum SlabBlockType {
    AcaciaSlab,
    AndesiteSlab,
    BambooMosaicSlab,
    BambooSlab,
    BirchSlab,
    BlackstoneSlab,
    BrickSlab,
    CherrySlab,
    CobbledDeepslateSlab,
    CobblestoneSlab,
    CrimsonSlab,
    CutCopperSlab,
    CutRedSandstoneSlab,
    CutSandstoneSlab,
    DarkOakSlab,
    DarkPrismarineSlab,
    DeepslateBrickSlab,
    DeepslateTileSlab,
    DioriteSlab,
    EndStoneBrickSlab,
    ExposedCutCopperSlab,
    GraniteSlab,
    JungleSlab,
    MangroveSlab,
    MossyCobblestoneSlab,
    MossyStoneBrickSlab,
    MudBrickSlab,
    NetherBrickSlab,
    OakSlab,
    OxidizedCutCopperSlab,
    PaleOakSlab,
    PetrifiedOakSlab,
    PolishedAndesiteSlab,
    PolishedBlackstoneBrickSlab,
    PolishedBlackstoneSlab,
    PolishedDeepslateSlab,
    PolishedDioriteSlab,
    PolishedGraniteSlab,
    PolishedTuffSlab,
    PrismarineBrickSlab,
    PrismarineSlab,
    PurpurSlab,
    QuartzSlab,
    RedNetherBrickSlab,
    RedSandstoneSlab,
    ResinBrickSlab,
    SandstoneSlab,
    SmoothQuartzSlab,
    SmoothRedSandstoneSlab,
    SmoothSandstoneSlab,
    SmoothStoneSlab,
    SpruceSlab,
    StoneBrickSlab,
    StoneSlab,
    TuffBrickSlab,
    TuffSlab,
    WarpedSlab,
    WaxedCutCopperSlab,
    WaxedExposedCutCopperSlab,
    WaxedOxidizedCutCopperSlab,
    WaxedWeatheredCutCopperSlab,
    WeatheredCutCopperSlab,
}
#[allow(dead_code)]
pub struct SlabBlock {
    pub block_type: SlabBlockType,
    pub ty: SlabType,
    pub waterlogged: bool,
}
impl TryFrom<u32> for SlabBlock {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            12075u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12076u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12077u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            12078u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            12079u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12080u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            15159u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            15160u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            15161u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15162u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            15163u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15164u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12111u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            12112u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            12113u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            12114u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            12115u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12116u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12105u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12106u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            12107u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            12108u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12109u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12110u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12063u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            12064u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12065u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12066u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12067u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12068u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            20893u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            20894u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            20895u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            20896u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            20897u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            20898u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            12153u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12154u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            12155u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12156u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12157u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12158u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            12081u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CherrySlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12082u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CherrySlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12083u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CherrySlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12084u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CherrySlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12085u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CherrySlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12086u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CherrySlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            26048u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            26049u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            26050u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            26051u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            26052u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            26053u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12147u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12148u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12149u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            12150u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            12151u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12152u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            19691u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CrimsonSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            19692u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CrimsonSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            19693u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CrimsonSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            19694u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CrimsonSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            19695u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CrimsonSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            19696u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CrimsonSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            24322u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutCopperSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            24323u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            24324u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            24325u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            24326u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            24327u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12189u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12190u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12191u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12192u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12193u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12194u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            12135u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12136u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12137u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12138u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12139u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12140u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            12087u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            12088u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            12089u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12090u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12091u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12092u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            11607u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            11608u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            11609u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            11610u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            11611u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            11612u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            27281u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            27282u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            27283u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            27284u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            27285u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            27286u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            26870u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            26871u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            26872u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            26873u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            26874u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            26875u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            15177u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            15178u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15179u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15180u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            15181u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            15182u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15135u32 => Ok(SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            15136u32 => Ok(SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            15137u32 => Ok(SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            15138u32 => Ok(SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            15139u32 => Ok(SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            15140u32 => Ok(SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            24316u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            24317u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            24318u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            24319u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            24320u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            24321u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15153u32 => Ok(SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            15154u32 => Ok(SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            15155u32 => Ok(SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15156u32 => Ok(SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            15157u32 => Ok(SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15158u32 => Ok(SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            12069u32 => Ok(SlabBlock {
                block_type: SlabBlockType::JungleSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12070u32 => Ok(SlabBlock {
                block_type: SlabBlockType::JungleSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12071u32 => Ok(SlabBlock {
                block_type: SlabBlockType::JungleSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12072u32 => Ok(SlabBlock {
                block_type: SlabBlockType::JungleSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12073u32 => Ok(SlabBlock {
                block_type: SlabBlockType::JungleSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12074u32 => Ok(SlabBlock {
                block_type: SlabBlockType::JungleSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12099u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            12100u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12101u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            12102u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            12103u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12104u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            15129u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            15130u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15131u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15132u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            15133u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            15134u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15117u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            15118u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15119u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15120u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            15121u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15122u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12165u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MudBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12166u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MudBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12167u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MudBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12168u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MudBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12169u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MudBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12170u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MudBrickSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            12171u32 => Ok(SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            12172u32 => Ok(SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            12173u32 => Ok(SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12174u32 => Ok(SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            12175u32 => Ok(SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12176u32 => Ok(SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12051u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OakSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12052u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OakSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            12053u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OakSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12054u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OakSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            12055u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OakSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12056u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OakSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            24304u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            24305u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            24306u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            24307u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            24308u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            24309u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            12093u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12094u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12095u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12096u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            12097u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12098u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12141u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12142u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            12143u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            12144u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12145u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12146u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15171u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            15172u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            15173u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15174u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            15175u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15176u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            20903u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            20904u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            20905u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            20906u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            20907u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            20908u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            21394u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            21395u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            21396u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            21397u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            21398u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            21399u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            26459u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedDeepslateSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            26460u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedDeepslateSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            26461u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedDeepslateSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            26462u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedDeepslateSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            26463u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedDeepslateSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            26464u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedDeepslateSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15123u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedDioriteSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            15124u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedDioriteSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15125u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedDioriteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15126u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedDioriteSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            15127u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedDioriteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15128u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedDioriteSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15105u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            15106u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15107u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            15108u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            15109u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15110u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            22521u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            22522u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            22523u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            22524u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            22525u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            22526u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            11601u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            11602u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            11603u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            11604u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            11605u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            11606u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            11595u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            11596u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            11597u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            11598u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            11599u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            11600u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            12195u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PurpurSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12196u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PurpurSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12197u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PurpurSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            12198u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PurpurSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12199u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PurpurSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12200u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PurpurSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12177u32 => Ok(SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12178u32 => Ok(SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12179u32 => Ok(SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12180u32 => Ok(SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12181u32 => Ok(SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12182u32 => Ok(SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            15165u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            15166u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15167u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15168u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            15169u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            15170u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12183u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12184u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12185u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            12186u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            12187u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12188u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            7725u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            7726u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            7727u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            7728u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            7729u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            7730u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            12129u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12130u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            12131u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12132u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12133u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12134u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15147u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            15148u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15149u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            15150u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            15151u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15152u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            15111u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            15112u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15113u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15114u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            15115u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15116u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            15141u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            15142u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15143u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15144u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            15145u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            15146u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12123u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothStoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12124u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothStoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12125u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothStoneSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            12126u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothStoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12127u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothStoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12128u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothStoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12057u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            12058u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12059u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            12060u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12061u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12062u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12159u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12160u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12161u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            12162u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            12163u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            12164u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12117u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12118u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12119u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12120u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12121u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12122u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            22933u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            22934u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            22935u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            22936u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            22937u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            22938u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffBrickSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            22110u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            22111u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                waterlogged: false,
                ty: SlabType::Top,
            }),
            22112u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            22113u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            22114u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            22115u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            19697u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                waterlogged: true,
                ty: SlabType::Top,
            }),
            19698u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            19699u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            19700u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            19701u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            19702u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            24674u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            24675u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            24676u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            24677u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            24678u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            24679u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            24668u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedExposedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            24669u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedExposedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            24670u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedExposedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            24671u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedExposedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            24672u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedExposedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            24673u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedExposedCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            24656u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedOxidizedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            24657u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedOxidizedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            24658u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedOxidizedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            24659u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedOxidizedCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            24660u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedOxidizedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            24661u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedOxidizedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            24662u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            24663u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            24664u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            }),
            24665u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            24666u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            24667u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            24310u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            24311u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            24312u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            24313u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            }),
            24314u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Double,
            }),
            24315u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Double,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for SlabBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12075u32),
            SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12076u32),
            SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(12077u32),
            SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(12078u32),
            SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12079u32),
            SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(12080u32),
            SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(15159u32),
            SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(15160u32),
            SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15161u32),
            SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(15162u32),
            SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15163u32),
            SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15164u32),
            SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(12111u32),
            SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(12112u32),
            SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(12113u32),
            SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(12114u32),
            SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12115u32),
            SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12116u32),
            SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12105u32),
            SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(12106u32),
            SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(12107u32),
            SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12108u32),
            SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12109u32),
            SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12110u32),
            SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(12063u32),
            SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12064u32),
            SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12065u32),
            SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12066u32),
            SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12067u32),
            SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(12068u32),
            SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(20893u32),
            SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(20894u32),
            SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(20895u32),
            SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(20896u32),
            SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(20897u32),
            SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(20898u32),
            SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12153u32),
            SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(12154u32),
            SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12155u32),
            SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12156u32),
            SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12157u32),
            SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(12158u32),
            SlabBlock {
                block_type: SlabBlockType::CherrySlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12081u32),
            SlabBlock {
                block_type: SlabBlockType::CherrySlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12082u32),
            SlabBlock {
                block_type: SlabBlockType::CherrySlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12083u32),
            SlabBlock {
                block_type: SlabBlockType::CherrySlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12084u32),
            SlabBlock {
                block_type: SlabBlockType::CherrySlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12085u32),
            SlabBlock {
                block_type: SlabBlockType::CherrySlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12086u32),
            SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(26048u32),
            SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(26049u32),
            SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(26050u32),
            SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(26051u32),
            SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(26052u32),
            SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(26053u32),
            SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12147u32),
            SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12148u32),
            SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(12149u32),
            SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(12150u32),
            SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12151u32),
            SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(12152u32),
            SlabBlock {
                block_type: SlabBlockType::CrimsonSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(19691u32),
            SlabBlock {
                block_type: SlabBlockType::CrimsonSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(19692u32),
            SlabBlock {
                block_type: SlabBlockType::CrimsonSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(19693u32),
            SlabBlock {
                block_type: SlabBlockType::CrimsonSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(19694u32),
            SlabBlock {
                block_type: SlabBlockType::CrimsonSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(19695u32),
            SlabBlock {
                block_type: SlabBlockType::CrimsonSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(19696u32),
            SlabBlock {
                block_type: SlabBlockType::CutCopperSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(24322u32),
            SlabBlock {
                block_type: SlabBlockType::CutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(24323u32),
            SlabBlock {
                block_type: SlabBlockType::CutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(24324u32),
            SlabBlock {
                block_type: SlabBlockType::CutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(24325u32),
            SlabBlock {
                block_type: SlabBlockType::CutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(24326u32),
            SlabBlock {
                block_type: SlabBlockType::CutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(24327u32),
            SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12189u32),
            SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12190u32),
            SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12191u32),
            SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12192u32),
            SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12193u32),
            SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(12194u32),
            SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12135u32),
            SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12136u32),
            SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12137u32),
            SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12138u32),
            SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12139u32),
            SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(12140u32),
            SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(12087u32),
            SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(12088u32),
            SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12089u32),
            SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12090u32),
            SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12091u32),
            SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12092u32),
            SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(11607u32),
            SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(11608u32),
            SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(11609u32),
            SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(11610u32),
            SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(11611u32),
            SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(11612u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(27281u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(27282u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(27283u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(27284u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(27285u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(27286u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(26870u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(26871u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(26872u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(26873u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(26874u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(26875u32),
            SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(15177u32),
            SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15178u32),
            SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15179u32),
            SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(15180u32),
            SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(15181u32),
            SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15182u32),
            SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(15135u32),
            SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(15136u32),
            SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(15137u32),
            SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(15138u32),
            SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(15139u32),
            SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(15140u32),
            SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(24316u32),
            SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(24317u32),
            SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(24318u32),
            SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(24319u32),
            SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(24320u32),
            SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(24321u32),
            SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(15153u32),
            SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(15154u32),
            SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15155u32),
            SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(15156u32),
            SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15157u32),
            SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(15158u32),
            SlabBlock {
                block_type: SlabBlockType::JungleSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12069u32),
            SlabBlock {
                block_type: SlabBlockType::JungleSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12070u32),
            SlabBlock {
                block_type: SlabBlockType::JungleSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12071u32),
            SlabBlock {
                block_type: SlabBlockType::JungleSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12072u32),
            SlabBlock {
                block_type: SlabBlockType::JungleSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12073u32),
            SlabBlock {
                block_type: SlabBlockType::JungleSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12074u32),
            SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(12099u32),
            SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12100u32),
            SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(12101u32),
            SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(12102u32),
            SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12103u32),
            SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(12104u32),
            SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(15129u32),
            SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15130u32),
            SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15131u32),
            SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(15132u32),
            SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(15133u32),
            SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15134u32),
            SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(15117u32),
            SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15118u32),
            SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15119u32),
            SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(15120u32),
            SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15121u32),
            SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15122u32),
            SlabBlock {
                block_type: SlabBlockType::MudBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12165u32),
            SlabBlock {
                block_type: SlabBlockType::MudBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12166u32),
            SlabBlock {
                block_type: SlabBlockType::MudBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12167u32),
            SlabBlock {
                block_type: SlabBlockType::MudBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12168u32),
            SlabBlock {
                block_type: SlabBlockType::MudBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12169u32),
            SlabBlock {
                block_type: SlabBlockType::MudBrickSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(12170u32),
            SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(12171u32),
            SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(12172u32),
            SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12173u32),
            SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(12174u32),
            SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12175u32),
            SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12176u32),
            SlabBlock {
                block_type: SlabBlockType::OakSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12051u32),
            SlabBlock {
                block_type: SlabBlockType::OakSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(12052u32),
            SlabBlock {
                block_type: SlabBlockType::OakSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12053u32),
            SlabBlock {
                block_type: SlabBlockType::OakSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(12054u32),
            SlabBlock {
                block_type: SlabBlockType::OakSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12055u32),
            SlabBlock {
                block_type: SlabBlockType::OakSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12056u32),
            SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(24304u32),
            SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(24305u32),
            SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(24306u32),
            SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(24307u32),
            SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(24308u32),
            SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(24309u32),
            SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12093u32),
            SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12094u32),
            SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12095u32),
            SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(12096u32),
            SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12097u32),
            SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12098u32),
            SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12141u32),
            SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(12142u32),
            SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(12143u32),
            SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12144u32),
            SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12145u32),
            SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12146u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(15171u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(15172u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15173u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(15174u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15175u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15176u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(20903u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(20904u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(20905u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(20906u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(20907u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(20908u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(21394u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(21395u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(21396u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(21397u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(21398u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(21399u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedDeepslateSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(26459u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedDeepslateSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(26460u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedDeepslateSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(26461u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedDeepslateSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(26462u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedDeepslateSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(26463u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedDeepslateSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(26464u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedDioriteSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(15123u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedDioriteSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15124u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedDioriteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15125u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedDioriteSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(15126u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedDioriteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15127u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedDioriteSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15128u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(15105u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15106u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(15107u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(15108u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15109u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(15110u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(22521u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(22522u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(22523u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(22524u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(22525u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(22526u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(11601u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(11602u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(11603u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(11604u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(11605u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(11606u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(11595u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(11596u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(11597u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(11598u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(11599u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(11600u32),
            SlabBlock {
                block_type: SlabBlockType::PurpurSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12195u32),
            SlabBlock {
                block_type: SlabBlockType::PurpurSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12196u32),
            SlabBlock {
                block_type: SlabBlockType::PurpurSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(12197u32),
            SlabBlock {
                block_type: SlabBlockType::PurpurSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12198u32),
            SlabBlock {
                block_type: SlabBlockType::PurpurSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12199u32),
            SlabBlock {
                block_type: SlabBlockType::PurpurSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12200u32),
            SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12177u32),
            SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12178u32),
            SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12179u32),
            SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12180u32),
            SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12181u32),
            SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(12182u32),
            SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(15165u32),
            SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15166u32),
            SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15167u32),
            SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(15168u32),
            SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(15169u32),
            SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15170u32),
            SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12183u32),
            SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12184u32),
            SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(12185u32),
            SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(12186u32),
            SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12187u32),
            SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(12188u32),
            SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(7725u32),
            SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(7726u32),
            SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(7727u32),
            SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(7728u32),
            SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(7729u32),
            SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(7730u32),
            SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12129u32),
            SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(12130u32),
            SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12131u32),
            SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12132u32),
            SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12133u32),
            SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12134u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(15147u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15148u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(15149u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(15150u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15151u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(15152u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(15111u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15112u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15113u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(15114u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15115u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(15116u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(15141u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15142u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15143u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(15144u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(15145u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15146u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothStoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12123u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothStoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12124u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothStoneSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(12125u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothStoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12126u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothStoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12127u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothStoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12128u32),
            SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(12057u32),
            SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12058u32),
            SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(12059u32),
            SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12060u32),
            SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12061u32),
            SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12062u32),
            SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12159u32),
            SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12160u32),
            SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(12161u32),
            SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(12162u32),
            SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(12163u32),
            SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12164u32),
            SlabBlock {
                block_type: SlabBlockType::StoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12117u32),
            SlabBlock {
                block_type: SlabBlockType::StoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12118u32),
            SlabBlock {
                block_type: SlabBlockType::StoneSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12119u32),
            SlabBlock {
                block_type: SlabBlockType::StoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12120u32),
            SlabBlock {
                block_type: SlabBlockType::StoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12121u32),
            SlabBlock {
                block_type: SlabBlockType::StoneSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(12122u32),
            SlabBlock {
                block_type: SlabBlockType::TuffBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(22933u32),
            SlabBlock {
                block_type: SlabBlockType::TuffBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(22934u32),
            SlabBlock {
                block_type: SlabBlockType::TuffBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(22935u32),
            SlabBlock {
                block_type: SlabBlockType::TuffBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(22936u32),
            SlabBlock {
                block_type: SlabBlockType::TuffBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(22937u32),
            SlabBlock {
                block_type: SlabBlockType::TuffBrickSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(22938u32),
            SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(22110u32),
            SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                waterlogged: false,
                ty: SlabType::Top,
            } => Ok(22111u32),
            SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(22112u32),
            SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(22113u32),
            SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(22114u32),
            SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(22115u32),
            SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                waterlogged: true,
                ty: SlabType::Top,
            } => Ok(19697u32),
            SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(19698u32),
            SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(19699u32),
            SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(19700u32),
            SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(19701u32),
            SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(19702u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(24674u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(24675u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(24676u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(24677u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(24678u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(24679u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedExposedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(24668u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedExposedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(24669u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedExposedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(24670u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedExposedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(24671u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedExposedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(24672u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedExposedCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(24673u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedOxidizedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(24656u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedOxidizedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(24657u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedOxidizedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(24658u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedOxidizedCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(24659u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedOxidizedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(24660u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedOxidizedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(24661u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(24662u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(24663u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Bottom,
            } => Ok(24664u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(24665u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(24666u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(24667u32),
            SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(24310u32),
            SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(24311u32),
            SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(24312u32),
            SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Bottom,
            } => Ok(24313u32),
            SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                waterlogged: true,
                ty: SlabType::Double,
            } => Ok(24314u32),
            SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                waterlogged: false,
                ty: SlabType::Double,
            } => Ok(24315u32),
            _ => Err(()),
        }
    }
}
