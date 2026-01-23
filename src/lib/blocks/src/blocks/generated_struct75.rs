#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum GeneratedStruct75Type {
    Composter,
    Lava,
    PowderSnowCauldron,
    Water,
    WaterCauldron,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct75 {
    pub block_type: GeneratedStruct75Type,
    pub level: i32,
}
impl TryInto<u32> for GeneratedStruct75 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Composter,
                level: 0i32,
            } => Ok(20400u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Composter,
                level: 1i32,
            } => Ok(20401u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Composter,
                level: 2i32,
            } => Ok(20402u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Composter,
                level: 3i32,
            } => Ok(20403u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Composter,
                level: 4i32,
            } => Ok(20404u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Composter,
                level: 5i32,
            } => Ok(20405u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Composter,
                level: 6i32,
            } => Ok(20406u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Composter,
                level: 7i32,
            } => Ok(20407u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Composter,
                level: 8i32,
            } => Ok(20408u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 0i32,
            } => Ok(102u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 1i32,
            } => Ok(103u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 2i32,
            } => Ok(104u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 3i32,
            } => Ok(105u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 4i32,
            } => Ok(106u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 5i32,
            } => Ok(107u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 6i32,
            } => Ok(108u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 7i32,
            } => Ok(109u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 8i32,
            } => Ok(110u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 9i32,
            } => Ok(111u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 10i32,
            } => Ok(112u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 11i32,
            } => Ok(113u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 12i32,
            } => Ok(114u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 13i32,
            } => Ok(115u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 14i32,
            } => Ok(116u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Lava,
                level: 15i32,
            } => Ok(117u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::PowderSnowCauldron,
                level: 1i32,
            } => Ok(8187u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::PowderSnowCauldron,
                level: 2i32,
            } => Ok(8188u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::PowderSnowCauldron,
                level: 3i32,
            } => Ok(8189u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 0i32,
            } => Ok(86u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 1i32,
            } => Ok(87u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 2i32,
            } => Ok(88u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 3i32,
            } => Ok(89u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 4i32,
            } => Ok(90u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 5i32,
            } => Ok(91u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 6i32,
            } => Ok(92u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 7i32,
            } => Ok(93u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 8i32,
            } => Ok(94u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 9i32,
            } => Ok(95u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 10i32,
            } => Ok(96u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 11i32,
            } => Ok(97u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 12i32,
            } => Ok(98u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 13i32,
            } => Ok(99u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 14i32,
            } => Ok(100u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::Water,
                level: 15i32,
            } => Ok(101u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::WaterCauldron,
                level: 1i32,
            } => Ok(8183u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::WaterCauldron,
                level: 2i32,
            } => Ok(8184u32),
            GeneratedStruct75 {
                block_type: GeneratedStruct75Type::WaterCauldron,
                level: 3i32,
            } => Ok(8185u32),
            _ => Err(()),
        }
    }
}
