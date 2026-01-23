#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum WaterloggableBlockType {
    Barrier,
    BrainCoral,
    BrainCoralFan,
    BubbleCoral,
    BubbleCoralFan,
    Conduit,
    CopperGrate,
    DeadBrainCoral,
    DeadBrainCoralFan,
    DeadBubbleCoral,
    DeadBubbleCoralFan,
    DeadFireCoral,
    DeadFireCoralFan,
    DeadHornCoral,
    DeadHornCoralFan,
    DeadTubeCoral,
    DeadTubeCoralFan,
    ExposedCopperGrate,
    FireCoral,
    FireCoralFan,
    HangingRoots,
    HeavyCore,
    HornCoral,
    HornCoralFan,
    MangroveRoots,
    OxidizedCopperGrate,
    TubeCoral,
    TubeCoralFan,
    WaxedCopperGrate,
    WaxedExposedCopperGrate,
    WaxedOxidizedCopperGrate,
    WaxedWeatheredCopperGrate,
    WeatheredCopperGrate,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WaterloggableBlock {
    pub block_type: WaterloggableBlockType,
    pub waterlogged: bool,
}
impl WaterloggableBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<WaterloggableBlock>();
}
impl TryFrom<u32> for WaterloggableBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            11254u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::Barrier,
                waterlogged: true,
            }),
            11255u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::Barrier,
                waterlogged: false,
            }),
            13848u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::BrainCoral,
                waterlogged: true,
            }),
            13849u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::BrainCoral,
                waterlogged: false,
            }),
            13868u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::BrainCoralFan,
                waterlogged: true,
            }),
            13869u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::BrainCoralFan,
                waterlogged: false,
            }),
            13850u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::BubbleCoral,
                waterlogged: true,
            }),
            13851u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::BubbleCoral,
                waterlogged: false,
            }),
            13870u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::BubbleCoralFan,
                waterlogged: true,
            }),
            13871u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::BubbleCoralFan,
                waterlogged: false,
            }),
            13965u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::Conduit,
                waterlogged: true,
            }),
            13966u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::Conduit,
                waterlogged: false,
            }),
            25704u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::CopperGrate,
                waterlogged: true,
            }),
            25705u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::CopperGrate,
                waterlogged: false,
            }),
            13838u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBrainCoral,
                waterlogged: true,
            }),
            13839u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBrainCoral,
                waterlogged: false,
            }),
            13858u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBrainCoralFan,
                waterlogged: true,
            }),
            13859u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBrainCoralFan,
                waterlogged: false,
            }),
            13840u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBubbleCoral,
                waterlogged: true,
            }),
            13841u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBubbleCoral,
                waterlogged: false,
            }),
            13860u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBubbleCoralFan,
                waterlogged: true,
            }),
            13861u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBubbleCoralFan,
                waterlogged: false,
            }),
            13842u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadFireCoral,
                waterlogged: true,
            }),
            13843u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadFireCoral,
                waterlogged: false,
            }),
            13862u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadFireCoralFan,
                waterlogged: true,
            }),
            13863u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadFireCoralFan,
                waterlogged: false,
            }),
            13844u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadHornCoral,
                waterlogged: true,
            }),
            13845u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadHornCoral,
                waterlogged: false,
            }),
            13864u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadHornCoralFan,
                waterlogged: true,
            }),
            13865u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadHornCoralFan,
                waterlogged: false,
            }),
            13836u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadTubeCoral,
                waterlogged: true,
            }),
            13837u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadTubeCoral,
                waterlogged: false,
            }),
            13856u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadTubeCoralFan,
                waterlogged: true,
            }),
            13857u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadTubeCoralFan,
                waterlogged: false,
            }),
            25706u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::ExposedCopperGrate,
                waterlogged: true,
            }),
            25707u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::ExposedCopperGrate,
                waterlogged: false,
            }),
            13852u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::FireCoral,
                waterlogged: true,
            }),
            13853u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::FireCoral,
                waterlogged: false,
            }),
            13872u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::FireCoralFan,
                waterlogged: true,
            }),
            13873u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::FireCoralFan,
                waterlogged: false,
            }),
            25960u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::HangingRoots,
                waterlogged: true,
            }),
            25961u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::HangingRoots,
                waterlogged: false,
            }),
            27742u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::HeavyCore,
                waterlogged: true,
            }),
            27743u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::HeavyCore,
                waterlogged: false,
            }),
            13854u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::HornCoral,
                waterlogged: true,
            }),
            13855u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::HornCoral,
                waterlogged: false,
            }),
            13874u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::HornCoralFan,
                waterlogged: true,
            }),
            13875u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::HornCoralFan,
                waterlogged: false,
            }),
            163u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::MangroveRoots,
                waterlogged: true,
            }),
            164u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::MangroveRoots,
                waterlogged: false,
            }),
            25710u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::OxidizedCopperGrate,
                waterlogged: true,
            }),
            25711u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::OxidizedCopperGrate,
                waterlogged: false,
            }),
            13846u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::TubeCoral,
                waterlogged: true,
            }),
            13847u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::TubeCoral,
                waterlogged: false,
            }),
            13866u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::TubeCoralFan,
                waterlogged: true,
            }),
            13867u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::TubeCoralFan,
                waterlogged: false,
            }),
            25712u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedCopperGrate,
                waterlogged: true,
            }),
            25713u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedCopperGrate,
                waterlogged: false,
            }),
            25714u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedExposedCopperGrate,
                waterlogged: true,
            }),
            25715u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedExposedCopperGrate,
                waterlogged: false,
            }),
            25718u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedOxidizedCopperGrate,
                waterlogged: true,
            }),
            25719u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedOxidizedCopperGrate,
                waterlogged: false,
            }),
            25716u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedWeatheredCopperGrate,
                waterlogged: true,
            }),
            25717u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedWeatheredCopperGrate,
                waterlogged: false,
            }),
            25708u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::WeatheredCopperGrate,
                waterlogged: true,
            }),
            25709u32 => Ok(WaterloggableBlock {
                block_type: WaterloggableBlockType::WeatheredCopperGrate,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for WaterloggableBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            WaterloggableBlock {
                block_type: WaterloggableBlockType::Barrier,
                waterlogged: true,
            } => Ok(11254u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::Barrier,
                waterlogged: false,
            } => Ok(11255u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::BrainCoral,
                waterlogged: true,
            } => Ok(13848u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::BrainCoral,
                waterlogged: false,
            } => Ok(13849u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::BrainCoralFan,
                waterlogged: true,
            } => Ok(13868u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::BrainCoralFan,
                waterlogged: false,
            } => Ok(13869u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::BubbleCoral,
                waterlogged: true,
            } => Ok(13850u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::BubbleCoral,
                waterlogged: false,
            } => Ok(13851u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::BubbleCoralFan,
                waterlogged: true,
            } => Ok(13870u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::BubbleCoralFan,
                waterlogged: false,
            } => Ok(13871u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::Conduit,
                waterlogged: true,
            } => Ok(13965u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::Conduit,
                waterlogged: false,
            } => Ok(13966u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::CopperGrate,
                waterlogged: true,
            } => Ok(25704u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::CopperGrate,
                waterlogged: false,
            } => Ok(25705u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBrainCoral,
                waterlogged: true,
            } => Ok(13838u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBrainCoral,
                waterlogged: false,
            } => Ok(13839u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBrainCoralFan,
                waterlogged: true,
            } => Ok(13858u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBrainCoralFan,
                waterlogged: false,
            } => Ok(13859u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBubbleCoral,
                waterlogged: true,
            } => Ok(13840u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBubbleCoral,
                waterlogged: false,
            } => Ok(13841u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBubbleCoralFan,
                waterlogged: true,
            } => Ok(13860u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadBubbleCoralFan,
                waterlogged: false,
            } => Ok(13861u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadFireCoral,
                waterlogged: true,
            } => Ok(13842u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadFireCoral,
                waterlogged: false,
            } => Ok(13843u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadFireCoralFan,
                waterlogged: true,
            } => Ok(13862u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadFireCoralFan,
                waterlogged: false,
            } => Ok(13863u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadHornCoral,
                waterlogged: true,
            } => Ok(13844u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadHornCoral,
                waterlogged: false,
            } => Ok(13845u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadHornCoralFan,
                waterlogged: true,
            } => Ok(13864u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadHornCoralFan,
                waterlogged: false,
            } => Ok(13865u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadTubeCoral,
                waterlogged: true,
            } => Ok(13836u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadTubeCoral,
                waterlogged: false,
            } => Ok(13837u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadTubeCoralFan,
                waterlogged: true,
            } => Ok(13856u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::DeadTubeCoralFan,
                waterlogged: false,
            } => Ok(13857u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::ExposedCopperGrate,
                waterlogged: true,
            } => Ok(25706u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::ExposedCopperGrate,
                waterlogged: false,
            } => Ok(25707u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::FireCoral,
                waterlogged: true,
            } => Ok(13852u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::FireCoral,
                waterlogged: false,
            } => Ok(13853u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::FireCoralFan,
                waterlogged: true,
            } => Ok(13872u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::FireCoralFan,
                waterlogged: false,
            } => Ok(13873u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::HangingRoots,
                waterlogged: true,
            } => Ok(25960u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::HangingRoots,
                waterlogged: false,
            } => Ok(25961u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::HeavyCore,
                waterlogged: true,
            } => Ok(27742u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::HeavyCore,
                waterlogged: false,
            } => Ok(27743u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::HornCoral,
                waterlogged: true,
            } => Ok(13854u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::HornCoral,
                waterlogged: false,
            } => Ok(13855u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::HornCoralFan,
                waterlogged: true,
            } => Ok(13874u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::HornCoralFan,
                waterlogged: false,
            } => Ok(13875u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::MangroveRoots,
                waterlogged: true,
            } => Ok(163u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::MangroveRoots,
                waterlogged: false,
            } => Ok(164u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::OxidizedCopperGrate,
                waterlogged: true,
            } => Ok(25710u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::OxidizedCopperGrate,
                waterlogged: false,
            } => Ok(25711u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::TubeCoral,
                waterlogged: true,
            } => Ok(13846u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::TubeCoral,
                waterlogged: false,
            } => Ok(13847u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::TubeCoralFan,
                waterlogged: true,
            } => Ok(13866u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::TubeCoralFan,
                waterlogged: false,
            } => Ok(13867u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedCopperGrate,
                waterlogged: true,
            } => Ok(25712u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedCopperGrate,
                waterlogged: false,
            } => Ok(25713u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedExposedCopperGrate,
                waterlogged: true,
            } => Ok(25714u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedExposedCopperGrate,
                waterlogged: false,
            } => Ok(25715u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedOxidizedCopperGrate,
                waterlogged: true,
            } => Ok(25718u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedOxidizedCopperGrate,
                waterlogged: false,
            } => Ok(25719u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedWeatheredCopperGrate,
                waterlogged: true,
            } => Ok(25716u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::WaxedWeatheredCopperGrate,
                waterlogged: false,
            } => Ok(25717u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::WeatheredCopperGrate,
                waterlogged: true,
            } => Ok(25708u32),
            WaterloggableBlock {
                block_type: WaterloggableBlockType::WeatheredCopperGrate,
                waterlogged: false,
            } => Ok(25709u32),
            _ => Err(()),
        }
    }
}
