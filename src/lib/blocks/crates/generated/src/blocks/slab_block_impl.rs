use crate::SlabBlock;
use crate::SlabBlockType;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for SlabBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
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
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12078u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12079u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12080u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15159u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            15160u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15161u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15162u32 => Ok(SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
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
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12112u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12113u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12114u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
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
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12107u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12108u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12109u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12110u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12063u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            20896u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            20897u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            20898u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12153u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12154u32 => Ok(SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: false,
            }),
            26050u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            26051u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12150u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12151u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12152u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12194u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12140u32 => Ok(SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12087u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12088u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                ty: SlabType::Top,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12092u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            11607u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: true,
            }),
            11612u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            27281u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            27282u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            27283u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            27284u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            27285u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            27286u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            26870u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            26871u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            26872u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15177u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15182u32 => Ok(SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15135u32 => Ok(SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            15136u32 => Ok(SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15137u32 => Ok(SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15138u32 => Ok(SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            15139u32 => Ok(SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15140u32 => Ok(SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            24316u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            24317u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            24318u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15155u32 => Ok(SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15156u32 => Ok(SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            15157u32 => Ok(SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15158u32 => Ok(SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12100u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12101u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12102u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12103u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12104u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15129u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            15133u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15134u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15117u32 => Ok(SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12171u32 => Ok(SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12172u32 => Ok(SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12173u32 => Ok(SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12174u32 => Ok(SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12175u32 => Ok(SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12053u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OakSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12054u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OakSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12055u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OakSlab,
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: false,
            }),
            24306u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            24307u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            24308u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            24309u32 => Ok(SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12097u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12143u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12144u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12145u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12146u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15171u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            15172u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                ty: SlabType::Top,
                waterlogged: false,
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
                ty: SlabType::Top,
                waterlogged: false,
            }),
            20905u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            20906u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
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
                ty: SlabType::Top,
                waterlogged: true,
            }),
            21395u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Top,
                waterlogged: true,
            }),
            15106u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            15107u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15108u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            15109u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15110u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            22521u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            22522u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            22523u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            22524u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            22525u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            22526u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            11601u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            11598u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            11599u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            11600u32 => Ok(SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12182u32 => Ok(SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15165u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12186u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12187u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            12188u32 => Ok(SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            7725u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            7726u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            7727u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            7728u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            7729u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            7730u32 => Ok(SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            12129u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12130u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            15150u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            15151u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            15152u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            15111u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            15145u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: true,
            }),
            12058u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            12059u32 => Ok(SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            12162u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            12163u32 => Ok(SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: false,
            }),
            22110u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            22111u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            22112u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            22113u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            22114u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            22115u32 => Ok(SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Double,
                waterlogged: false,
            }),
            19697u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                ty: SlabType::Top,
                waterlogged: true,
            }),
            19698u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                ty: SlabType::Top,
                waterlogged: false,
            }),
            19699u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            24677u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            24678u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            }),
            24665u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            24666u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            24667u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: false,
            }),
            24314u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            }),
            24315u32 => Ok(SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12077u32),
            SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12078u32),
            SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12079u32),
            SlabBlock {
                block_type: SlabBlockType::AcaciaSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12080u32),
            SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(15159u32),
            SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15160u32),
            SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15161u32),
            SlabBlock {
                block_type: SlabBlockType::AndesiteSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
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
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12111u32),
            SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12112u32),
            SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12113u32),
            SlabBlock {
                block_type: SlabBlockType::BambooMosaicSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
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
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12106u32),
            SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12107u32),
            SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12108u32),
            SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12109u32),
            SlabBlock {
                block_type: SlabBlockType::BambooSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12110u32),
            SlabBlock {
                block_type: SlabBlockType::BirchSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(20895u32),
            SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(20896u32),
            SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(20897u32),
            SlabBlock {
                block_type: SlabBlockType::BlackstoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(20898u32),
            SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12153u32),
            SlabBlock {
                block_type: SlabBlockType::BrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(26049u32),
            SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(26050u32),
            SlabBlock {
                block_type: SlabBlockType::CobbledDeepslateSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12149u32),
            SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12150u32),
            SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12151u32),
            SlabBlock {
                block_type: SlabBlockType::CobblestoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12193u32),
            SlabBlock {
                block_type: SlabBlockType::CutRedSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12139u32),
            SlabBlock {
                block_type: SlabBlockType::CutSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12140u32),
            SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12087u32),
            SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                ty: SlabType::Top,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12091u32),
            SlabBlock {
                block_type: SlabBlockType::DarkOakSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12092u32),
            SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(11611u32),
            SlabBlock {
                block_type: SlabBlockType::DarkPrismarineSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(11612u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(27281u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(27282u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(27283u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(27284u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(27285u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(27286u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(26870u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(26871u32),
            SlabBlock {
                block_type: SlabBlockType::DeepslateTileSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(26875u32),
            SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15181u32),
            SlabBlock {
                block_type: SlabBlockType::DioriteSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15182u32),
            SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(15135u32),
            SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15136u32),
            SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15137u32),
            SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(15138u32),
            SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15139u32),
            SlabBlock {
                block_type: SlabBlockType::EndStoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15140u32),
            SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(24316u32),
            SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(24317u32),
            SlabBlock {
                block_type: SlabBlockType::ExposedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15154u32),
            SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15155u32),
            SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(15156u32),
            SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15157u32),
            SlabBlock {
                block_type: SlabBlockType::GraniteSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12099u32),
            SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12100u32),
            SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12101u32),
            SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12102u32),
            SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12103u32),
            SlabBlock {
                block_type: SlabBlockType::MangroveSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12104u32),
            SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(15132u32),
            SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15133u32),
            SlabBlock {
                block_type: SlabBlockType::MossyCobblestoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15134u32),
            SlabBlock {
                block_type: SlabBlockType::MossyStoneBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12170u32),
            SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12171u32),
            SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12172u32),
            SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12173u32),
            SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12174u32),
            SlabBlock {
                block_type: SlabBlockType::NetherBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12052u32),
            SlabBlock {
                block_type: SlabBlockType::OakSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12053u32),
            SlabBlock {
                block_type: SlabBlockType::OakSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12054u32),
            SlabBlock {
                block_type: SlabBlockType::OakSlab,
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(24305u32),
            SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(24306u32),
            SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(24307u32),
            SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(24308u32),
            SlabBlock {
                block_type: SlabBlockType::OxidizedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12096u32),
            SlabBlock {
                block_type: SlabBlockType::PaleOakSlab,
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12142u32),
            SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12143u32),
            SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12144u32),
            SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12145u32),
            SlabBlock {
                block_type: SlabBlockType::PetrifiedOakSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12146u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(15171u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedAndesiteSlab,
                ty: SlabType::Top,
                waterlogged: false,
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
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(20904u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(20905u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
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
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(21394u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedBlackstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(15105u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(15106u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15107u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(15108u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15109u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedGraniteSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15110u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(22521u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(22522u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(22523u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(22524u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(22525u32),
            SlabBlock {
                block_type: SlabBlockType::PolishedTuffSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(22526u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(11597u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(11598u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(11599u32),
            SlabBlock {
                block_type: SlabBlockType::PrismarineSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12181u32),
            SlabBlock {
                block_type: SlabBlockType::QuartzSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12182u32),
            SlabBlock {
                block_type: SlabBlockType::RedNetherBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12185u32),
            SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12186u32),
            SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(12187u32),
            SlabBlock {
                block_type: SlabBlockType::RedSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(12188u32),
            SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(7725u32),
            SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(7726u32),
            SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(7727u32),
            SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(7728u32),
            SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(7729u32),
            SlabBlock {
                block_type: SlabBlockType::ResinBrickSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(7730u32),
            SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12129u32),
            SlabBlock {
                block_type: SlabBlockType::SandstoneSlab,
                ty: SlabType::Top,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(15149u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(15150u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(15151u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothQuartzSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(15152u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothRedSandstoneSlab,
                ty: SlabType::Top,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(15144u32),
            SlabBlock {
                block_type: SlabBlockType::SmoothSandstoneSlab,
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(12057u32),
            SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(12058u32),
            SlabBlock {
                block_type: SlabBlockType::SpruceSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(12161u32),
            SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(12162u32),
            SlabBlock {
                block_type: SlabBlockType::StoneBrickSlab,
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(22938u32),
            SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(22110u32),
            SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(22111u32),
            SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(22112u32),
            SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(22113u32),
            SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(22114u32),
            SlabBlock {
                block_type: SlabBlockType::TuffSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(22115u32),
            SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                ty: SlabType::Top,
                waterlogged: true,
            } => Ok(19697u32),
            SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                ty: SlabType::Top,
                waterlogged: false,
            } => Ok(19698u32),
            SlabBlock {
                block_type: SlabBlockType::WarpedSlab,
                ty: SlabType::Bottom,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(24676u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(24677u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
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
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: true,
            } => Ok(24664u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(24665u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(24666u32),
            SlabBlock {
                block_type: SlabBlockType::WaxedWeatheredCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
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
                ty: SlabType::Bottom,
                waterlogged: false,
            } => Ok(24313u32),
            SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: true,
            } => Ok(24314u32),
            SlabBlock {
                block_type: SlabBlockType::WeatheredCutCopperSlab,
                ty: SlabType::Double,
                waterlogged: false,
            } => Ok(24315u32),
            _ => Err(()),
        }
    }
}
