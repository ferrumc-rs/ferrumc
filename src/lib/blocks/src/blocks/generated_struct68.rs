#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum GeneratedStruct68Type {
    Lantern,
    SoulLantern,
}
#[allow(dead_code)]
pub struct GeneratedStruct68 {
    pub block_type: GeneratedStruct68Type,
    pub hanging: bool,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct68 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            19526u32 => Ok(GeneratedStruct68 {
                block_type: GeneratedStruct68Type::Lantern,
                hanging: true,
                waterlogged: true,
            }),
            19527u32 => Ok(GeneratedStruct68 {
                block_type: GeneratedStruct68Type::Lantern,
                hanging: true,
                waterlogged: false,
            }),
            19528u32 => Ok(GeneratedStruct68 {
                block_type: GeneratedStruct68Type::Lantern,
                hanging: false,
                waterlogged: true,
            }),
            19529u32 => Ok(GeneratedStruct68 {
                block_type: GeneratedStruct68Type::Lantern,
                hanging: false,
                waterlogged: false,
            }),
            19530u32 => Ok(GeneratedStruct68 {
                block_type: GeneratedStruct68Type::SoulLantern,
                waterlogged: true,
                hanging: true,
            }),
            19531u32 => Ok(GeneratedStruct68 {
                block_type: GeneratedStruct68Type::SoulLantern,
                hanging: true,
                waterlogged: false,
            }),
            19532u32 => Ok(GeneratedStruct68 {
                block_type: GeneratedStruct68Type::SoulLantern,
                hanging: false,
                waterlogged: true,
            }),
            19533u32 => Ok(GeneratedStruct68 {
                block_type: GeneratedStruct68Type::SoulLantern,
                waterlogged: false,
                hanging: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct68 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct68 {
                block_type: GeneratedStruct68Type::Lantern,
                hanging: true,
                waterlogged: true,
            } => Ok(19526u32),
            GeneratedStruct68 {
                block_type: GeneratedStruct68Type::Lantern,
                hanging: true,
                waterlogged: false,
            } => Ok(19527u32),
            GeneratedStruct68 {
                block_type: GeneratedStruct68Type::Lantern,
                hanging: false,
                waterlogged: true,
            } => Ok(19528u32),
            GeneratedStruct68 {
                block_type: GeneratedStruct68Type::Lantern,
                hanging: false,
                waterlogged: false,
            } => Ok(19529u32),
            GeneratedStruct68 {
                block_type: GeneratedStruct68Type::SoulLantern,
                waterlogged: true,
                hanging: true,
            } => Ok(19530u32),
            GeneratedStruct68 {
                block_type: GeneratedStruct68Type::SoulLantern,
                hanging: true,
                waterlogged: false,
            } => Ok(19531u32),
            GeneratedStruct68 {
                block_type: GeneratedStruct68Type::SoulLantern,
                hanging: false,
                waterlogged: true,
            } => Ok(19532u32),
            GeneratedStruct68 {
                block_type: GeneratedStruct68Type::SoulLantern,
                waterlogged: false,
                hanging: false,
            } => Ok(19533u32),
            _ => Err(()),
        }
    }
}
