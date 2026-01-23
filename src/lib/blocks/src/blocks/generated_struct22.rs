#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum GeneratedStruct22Type {
    ChainCommandBlock,
    CommandBlock,
    RepeatingCommandBlock,
}
#[allow(dead_code)]
pub struct GeneratedStruct22 {
    pub block_type: GeneratedStruct22Type,
    pub conditional: bool,
    pub facing: Direction,
}
impl TryFrom<u32> for GeneratedStruct22 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            13550u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                conditional: true,
                facing: Direction::North,
            }),
            13551u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                facing: Direction::East,
                conditional: true,
            }),
            13552u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                conditional: true,
                facing: Direction::South,
            }),
            13553u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                facing: Direction::West,
                conditional: true,
            }),
            13554u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                facing: Direction::Up,
                conditional: true,
            }),
            13555u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                conditional: true,
                facing: Direction::Down,
            }),
            13556u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                conditional: false,
                facing: Direction::North,
            }),
            13557u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                conditional: false,
                facing: Direction::East,
            }),
            13558u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                facing: Direction::South,
                conditional: false,
            }),
            13559u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                conditional: false,
                facing: Direction::West,
            }),
            13560u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                facing: Direction::Up,
                conditional: false,
            }),
            13561u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                facing: Direction::Down,
                conditional: false,
            }),
            8690u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: true,
                facing: Direction::North,
            }),
            8691u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                facing: Direction::East,
                conditional: true,
            }),
            8692u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: true,
                facing: Direction::South,
            }),
            8693u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: true,
                facing: Direction::West,
            }),
            8694u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: true,
                facing: Direction::Up,
            }),
            8695u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                facing: Direction::Down,
                conditional: true,
            }),
            8696u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                facing: Direction::North,
                conditional: false,
            }),
            8697u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: false,
                facing: Direction::East,
            }),
            8698u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: false,
                facing: Direction::South,
            }),
            8699u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: false,
                facing: Direction::West,
            }),
            8700u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: false,
                facing: Direction::Up,
            }),
            8701u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: false,
                facing: Direction::Down,
            }),
            13538u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::North,
            }),
            13539u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::East,
            }),
            13540u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                facing: Direction::South,
                conditional: true,
            }),
            13541u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::West,
            }),
            13542u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::Up,
            }),
            13543u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                facing: Direction::Down,
                conditional: true,
            }),
            13544u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                facing: Direction::North,
                conditional: false,
            }),
            13545u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: false,
                facing: Direction::East,
            }),
            13546u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                facing: Direction::South,
                conditional: false,
            }),
            13547u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                facing: Direction::West,
                conditional: false,
            }),
            13548u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: false,
                facing: Direction::Up,
            }),
            13549u32 => Ok(GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: false,
                facing: Direction::Down,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct22 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                conditional: true,
                facing: Direction::North,
            } => Ok(13550u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                facing: Direction::East,
                conditional: true,
            } => Ok(13551u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                conditional: true,
                facing: Direction::South,
            } => Ok(13552u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                facing: Direction::West,
                conditional: true,
            } => Ok(13553u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                facing: Direction::Up,
                conditional: true,
            } => Ok(13554u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                conditional: true,
                facing: Direction::Down,
            } => Ok(13555u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                conditional: false,
                facing: Direction::North,
            } => Ok(13556u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                conditional: false,
                facing: Direction::East,
            } => Ok(13557u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                facing: Direction::South,
                conditional: false,
            } => Ok(13558u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                conditional: false,
                facing: Direction::West,
            } => Ok(13559u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                facing: Direction::Up,
                conditional: false,
            } => Ok(13560u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::ChainCommandBlock,
                facing: Direction::Down,
                conditional: false,
            } => Ok(13561u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: true,
                facing: Direction::North,
            } => Ok(8690u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                facing: Direction::East,
                conditional: true,
            } => Ok(8691u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: true,
                facing: Direction::South,
            } => Ok(8692u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: true,
                facing: Direction::West,
            } => Ok(8693u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: true,
                facing: Direction::Up,
            } => Ok(8694u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                facing: Direction::Down,
                conditional: true,
            } => Ok(8695u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                facing: Direction::North,
                conditional: false,
            } => Ok(8696u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: false,
                facing: Direction::East,
            } => Ok(8697u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: false,
                facing: Direction::South,
            } => Ok(8698u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: false,
                facing: Direction::West,
            } => Ok(8699u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: false,
                facing: Direction::Up,
            } => Ok(8700u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::CommandBlock,
                conditional: false,
                facing: Direction::Down,
            } => Ok(8701u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::North,
            } => Ok(13538u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::East,
            } => Ok(13539u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                facing: Direction::South,
                conditional: true,
            } => Ok(13540u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::West,
            } => Ok(13541u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::Up,
            } => Ok(13542u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                facing: Direction::Down,
                conditional: true,
            } => Ok(13543u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                facing: Direction::North,
                conditional: false,
            } => Ok(13544u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: false,
                facing: Direction::East,
            } => Ok(13545u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                facing: Direction::South,
                conditional: false,
            } => Ok(13546u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                facing: Direction::West,
                conditional: false,
            } => Ok(13547u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: false,
                facing: Direction::Up,
            } => Ok(13548u32),
            GeneratedStruct22 {
                block_type: GeneratedStruct22Type::RepeatingCommandBlock,
                conditional: false,
                facing: Direction::Down,
            } => Ok(13549u32),
            _ => Err(()),
        }
    }
}
