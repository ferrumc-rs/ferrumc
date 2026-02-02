use crate::LiquidBlock;
use crate::LiquidBlockType;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for LiquidBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            102u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 0i32,
            }),
            103u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 1i32,
            }),
            104u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 2i32,
            }),
            105u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 3i32,
            }),
            106u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 4i32,
            }),
            107u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 5i32,
            }),
            108u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 6i32,
            }),
            109u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 7i32,
            }),
            110u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 8i32,
            }),
            111u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 9i32,
            }),
            112u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 10i32,
            }),
            113u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 11i32,
            }),
            114u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 12i32,
            }),
            115u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 13i32,
            }),
            116u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 14i32,
            }),
            117u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 15i32,
            }),
            86u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 0i32,
            }),
            87u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 1i32,
            }),
            88u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 2i32,
            }),
            89u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 3i32,
            }),
            90u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 4i32,
            }),
            91u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 5i32,
            }),
            92u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 6i32,
            }),
            93u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 7i32,
            }),
            94u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 8i32,
            }),
            95u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 9i32,
            }),
            96u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 10i32,
            }),
            97u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 11i32,
            }),
            98u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 12i32,
            }),
            99u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 13i32,
            }),
            100u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 14i32,
            }),
            101u32 => Ok(LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 15i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for LiquidBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 0i32,
            } => Ok(102u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 1i32,
            } => Ok(103u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 2i32,
            } => Ok(104u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 3i32,
            } => Ok(105u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 4i32,
            } => Ok(106u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 5i32,
            } => Ok(107u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 6i32,
            } => Ok(108u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 7i32,
            } => Ok(109u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 8i32,
            } => Ok(110u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 9i32,
            } => Ok(111u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 10i32,
            } => Ok(112u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 11i32,
            } => Ok(113u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 12i32,
            } => Ok(114u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 13i32,
            } => Ok(115u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 14i32,
            } => Ok(116u32),
            LiquidBlock {
                block_type: LiquidBlockType::Lava,
                level: 15i32,
            } => Ok(117u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 0i32,
            } => Ok(86u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 1i32,
            } => Ok(87u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 2i32,
            } => Ok(88u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 3i32,
            } => Ok(89u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 4i32,
            } => Ok(90u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 5i32,
            } => Ok(91u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 6i32,
            } => Ok(92u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 7i32,
            } => Ok(93u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 8i32,
            } => Ok(94u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 9i32,
            } => Ok(95u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 10i32,
            } => Ok(96u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 11i32,
            } => Ok(97u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 12i32,
            } => Ok(98u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 13i32,
            } => Ok(99u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 14i32,
            } => Ok(100u32),
            LiquidBlock {
                block_type: LiquidBlockType::Water,
                level: 15i32,
            } => Ok(101u32),
            _ => Err(()),
        }
    }
}
