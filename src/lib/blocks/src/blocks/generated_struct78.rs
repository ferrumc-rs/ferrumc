#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum GeneratedStruct78Type {
    CopperBulb,
    ExposedCopperBulb,
    OxidizedCopperBulb,
    WaxedCopperBulb,
    WaxedExposedCopperBulb,
    WaxedOxidizedCopperBulb,
    WaxedWeatheredCopperBulb,
    WeatheredCopperBulb,
}
#[allow(dead_code)]
pub struct GeneratedStruct78 {
    pub block_type: GeneratedStruct78Type,
    pub lit: bool,
    pub powered: bool,
}
impl TryFrom<u32> for GeneratedStruct78 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            25720u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::CopperBulb,
                lit: true,
                powered: true,
            }),
            25721u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::CopperBulb,
                lit: true,
                powered: false,
            }),
            25722u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::CopperBulb,
                lit: false,
                powered: true,
            }),
            25723u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::CopperBulb,
                lit: false,
                powered: false,
            }),
            25724u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::ExposedCopperBulb,
                lit: true,
                powered: true,
            }),
            25725u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::ExposedCopperBulb,
                lit: true,
                powered: false,
            }),
            25726u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::ExposedCopperBulb,
                lit: false,
                powered: true,
            }),
            25727u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::ExposedCopperBulb,
                lit: false,
                powered: false,
            }),
            25732u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::OxidizedCopperBulb,
                lit: true,
                powered: true,
            }),
            25733u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::OxidizedCopperBulb,
                lit: true,
                powered: false,
            }),
            25734u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::OxidizedCopperBulb,
                lit: false,
                powered: true,
            }),
            25735u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::OxidizedCopperBulb,
                lit: false,
                powered: false,
            }),
            25736u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedCopperBulb,
                lit: true,
                powered: true,
            }),
            25737u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedCopperBulb,
                lit: true,
                powered: false,
            }),
            25738u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedCopperBulb,
                lit: false,
                powered: true,
            }),
            25739u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedCopperBulb,
                lit: false,
                powered: false,
            }),
            25740u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedExposedCopperBulb,
                lit: true,
                powered: true,
            }),
            25741u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedExposedCopperBulb,
                lit: true,
                powered: false,
            }),
            25742u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedExposedCopperBulb,
                lit: false,
                powered: true,
            }),
            25743u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedExposedCopperBulb,
                lit: false,
                powered: false,
            }),
            25748u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedOxidizedCopperBulb,
                lit: true,
                powered: true,
            }),
            25749u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedOxidizedCopperBulb,
                lit: true,
                powered: false,
            }),
            25750u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedOxidizedCopperBulb,
                lit: false,
                powered: true,
            }),
            25751u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedOxidizedCopperBulb,
                lit: false,
                powered: false,
            }),
            25744u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedWeatheredCopperBulb,
                lit: true,
                powered: true,
            }),
            25745u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedWeatheredCopperBulb,
                lit: true,
                powered: false,
            }),
            25746u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedWeatheredCopperBulb,
                lit: false,
                powered: true,
            }),
            25747u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedWeatheredCopperBulb,
                lit: false,
                powered: false,
            }),
            25728u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WeatheredCopperBulb,
                lit: true,
                powered: true,
            }),
            25729u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WeatheredCopperBulb,
                lit: true,
                powered: false,
            }),
            25730u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WeatheredCopperBulb,
                lit: false,
                powered: true,
            }),
            25731u32 => Ok(GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WeatheredCopperBulb,
                lit: false,
                powered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct78 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::CopperBulb,
                lit: true,
                powered: true,
            } => Ok(25720u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::CopperBulb,
                lit: true,
                powered: false,
            } => Ok(25721u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::CopperBulb,
                lit: false,
                powered: true,
            } => Ok(25722u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::CopperBulb,
                lit: false,
                powered: false,
            } => Ok(25723u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::ExposedCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25724u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::ExposedCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25725u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::ExposedCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25726u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::ExposedCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25727u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::OxidizedCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25732u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::OxidizedCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25733u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::OxidizedCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25734u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::OxidizedCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25735u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25736u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25737u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25738u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25739u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedExposedCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25740u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedExposedCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25741u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedExposedCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25742u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedExposedCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25743u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedOxidizedCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25748u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedOxidizedCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25749u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedOxidizedCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25750u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedOxidizedCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25751u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedWeatheredCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25744u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedWeatheredCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25745u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedWeatheredCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25746u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WaxedWeatheredCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25747u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WeatheredCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25728u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WeatheredCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25729u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WeatheredCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25730u32),
            GeneratedStruct78 {
                block_type: GeneratedStruct78Type::WeatheredCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25731u32),
            _ => Err(()),
        }
    }
}
