use crate::ScaffoldingBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for ScaffoldingBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            19395u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 0i32,
                waterlogged: true,
            }),
            19396u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 0i32,
                waterlogged: false,
            }),
            19397u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 1i32,
                waterlogged: true,
            }),
            19398u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 1i32,
                waterlogged: false,
            }),
            19399u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 2i32,
                waterlogged: true,
            }),
            19400u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 2i32,
                waterlogged: false,
            }),
            19401u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 3i32,
                waterlogged: true,
            }),
            19402u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 3i32,
                waterlogged: false,
            }),
            19403u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 4i32,
                waterlogged: true,
            }),
            19404u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 4i32,
                waterlogged: false,
            }),
            19405u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 5i32,
                waterlogged: true,
            }),
            19406u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 5i32,
                waterlogged: false,
            }),
            19407u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 6i32,
                waterlogged: true,
            }),
            19408u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 6i32,
                waterlogged: false,
            }),
            19409u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 7i32,
                waterlogged: true,
            }),
            19410u32 => Ok(ScaffoldingBlock {
                bottom: true,
                distance: 7i32,
                waterlogged: false,
            }),
            19411u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 0i32,
                waterlogged: true,
            }),
            19412u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 0i32,
                waterlogged: false,
            }),
            19413u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 1i32,
                waterlogged: true,
            }),
            19414u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 1i32,
                waterlogged: false,
            }),
            19415u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 2i32,
                waterlogged: true,
            }),
            19416u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 2i32,
                waterlogged: false,
            }),
            19417u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 3i32,
                waterlogged: true,
            }),
            19418u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 3i32,
                waterlogged: false,
            }),
            19419u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 4i32,
                waterlogged: true,
            }),
            19420u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 4i32,
                waterlogged: false,
            }),
            19421u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 5i32,
                waterlogged: true,
            }),
            19422u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 5i32,
                waterlogged: false,
            }),
            19423u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 6i32,
                waterlogged: true,
            }),
            19424u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 6i32,
                waterlogged: false,
            }),
            19425u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 7i32,
                waterlogged: true,
            }),
            19426u32 => Ok(ScaffoldingBlock {
                bottom: false,
                distance: 7i32,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for ScaffoldingBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            ScaffoldingBlock {
                bottom: true,
                distance: 0i32,
                waterlogged: true,
            } => Ok(19395u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 0i32,
                waterlogged: false,
            } => Ok(19396u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 1i32,
                waterlogged: true,
            } => Ok(19397u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 1i32,
                waterlogged: false,
            } => Ok(19398u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 2i32,
                waterlogged: true,
            } => Ok(19399u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 2i32,
                waterlogged: false,
            } => Ok(19400u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 3i32,
                waterlogged: true,
            } => Ok(19401u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 3i32,
                waterlogged: false,
            } => Ok(19402u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 4i32,
                waterlogged: true,
            } => Ok(19403u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 4i32,
                waterlogged: false,
            } => Ok(19404u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 5i32,
                waterlogged: true,
            } => Ok(19405u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 5i32,
                waterlogged: false,
            } => Ok(19406u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 6i32,
                waterlogged: true,
            } => Ok(19407u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 6i32,
                waterlogged: false,
            } => Ok(19408u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 7i32,
                waterlogged: true,
            } => Ok(19409u32),
            ScaffoldingBlock {
                bottom: true,
                distance: 7i32,
                waterlogged: false,
            } => Ok(19410u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 0i32,
                waterlogged: true,
            } => Ok(19411u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 0i32,
                waterlogged: false,
            } => Ok(19412u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 1i32,
                waterlogged: true,
            } => Ok(19413u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 1i32,
                waterlogged: false,
            } => Ok(19414u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 2i32,
                waterlogged: true,
            } => Ok(19415u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 2i32,
                waterlogged: false,
            } => Ok(19416u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 3i32,
                waterlogged: true,
            } => Ok(19417u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 3i32,
                waterlogged: false,
            } => Ok(19418u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 4i32,
                waterlogged: true,
            } => Ok(19419u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 4i32,
                waterlogged: false,
            } => Ok(19420u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 5i32,
                waterlogged: true,
            } => Ok(19421u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 5i32,
                waterlogged: false,
            } => Ok(19422u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 6i32,
                waterlogged: true,
            } => Ok(19423u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 6i32,
                waterlogged: false,
            } => Ok(19424u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 7i32,
                waterlogged: true,
            } => Ok(19425u32),
            ScaffoldingBlock {
                bottom: false,
                distance: 7i32,
                waterlogged: false,
            } => Ok(19426u32),
            _ => Err(()),
        }
    }
}
