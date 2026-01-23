#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct17 {
    pub bottom: bool,
    pub distance: i32,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct17 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            19395u32 => Ok(GeneratedStruct17 {
                bottom: true,
                waterlogged: true,
                distance: 0i32,
            }),
            19396u32 => Ok(GeneratedStruct17 {
                bottom: true,
                distance: 0i32,
                waterlogged: false,
            }),
            19397u32 => Ok(GeneratedStruct17 {
                distance: 1i32,
                waterlogged: true,
                bottom: true,
            }),
            19398u32 => Ok(GeneratedStruct17 {
                bottom: true,
                waterlogged: false,
                distance: 1i32,
            }),
            19399u32 => Ok(GeneratedStruct17 {
                bottom: true,
                distance: 2i32,
                waterlogged: true,
            }),
            19400u32 => Ok(GeneratedStruct17 {
                bottom: true,
                waterlogged: false,
                distance: 2i32,
            }),
            19401u32 => Ok(GeneratedStruct17 {
                waterlogged: true,
                distance: 3i32,
                bottom: true,
            }),
            19402u32 => Ok(GeneratedStruct17 {
                bottom: true,
                distance: 3i32,
                waterlogged: false,
            }),
            19403u32 => Ok(GeneratedStruct17 {
                distance: 4i32,
                waterlogged: true,
                bottom: true,
            }),
            19404u32 => Ok(GeneratedStruct17 {
                bottom: true,
                waterlogged: false,
                distance: 4i32,
            }),
            19405u32 => Ok(GeneratedStruct17 {
                distance: 5i32,
                bottom: true,
                waterlogged: true,
            }),
            19406u32 => Ok(GeneratedStruct17 {
                distance: 5i32,
                bottom: true,
                waterlogged: false,
            }),
            19407u32 => Ok(GeneratedStruct17 {
                bottom: true,
                waterlogged: true,
                distance: 6i32,
            }),
            19408u32 => Ok(GeneratedStruct17 {
                distance: 6i32,
                bottom: true,
                waterlogged: false,
            }),
            19409u32 => Ok(GeneratedStruct17 {
                bottom: true,
                distance: 7i32,
                waterlogged: true,
            }),
            19410u32 => Ok(GeneratedStruct17 {
                bottom: true,
                distance: 7i32,
                waterlogged: false,
            }),
            19411u32 => Ok(GeneratedStruct17 {
                bottom: false,
                distance: 0i32,
                waterlogged: true,
            }),
            19412u32 => Ok(GeneratedStruct17 {
                waterlogged: false,
                distance: 0i32,
                bottom: false,
            }),
            19413u32 => Ok(GeneratedStruct17 {
                bottom: false,
                distance: 1i32,
                waterlogged: true,
            }),
            19414u32 => Ok(GeneratedStruct17 {
                bottom: false,
                distance: 1i32,
                waterlogged: false,
            }),
            19415u32 => Ok(GeneratedStruct17 {
                waterlogged: true,
                distance: 2i32,
                bottom: false,
            }),
            19416u32 => Ok(GeneratedStruct17 {
                waterlogged: false,
                distance: 2i32,
                bottom: false,
            }),
            19417u32 => Ok(GeneratedStruct17 {
                bottom: false,
                distance: 3i32,
                waterlogged: true,
            }),
            19418u32 => Ok(GeneratedStruct17 {
                bottom: false,
                waterlogged: false,
                distance: 3i32,
            }),
            19419u32 => Ok(GeneratedStruct17 {
                distance: 4i32,
                waterlogged: true,
                bottom: false,
            }),
            19420u32 => Ok(GeneratedStruct17 {
                waterlogged: false,
                distance: 4i32,
                bottom: false,
            }),
            19421u32 => Ok(GeneratedStruct17 {
                waterlogged: true,
                bottom: false,
                distance: 5i32,
            }),
            19422u32 => Ok(GeneratedStruct17 {
                distance: 5i32,
                waterlogged: false,
                bottom: false,
            }),
            19423u32 => Ok(GeneratedStruct17 {
                distance: 6i32,
                waterlogged: true,
                bottom: false,
            }),
            19424u32 => Ok(GeneratedStruct17 {
                distance: 6i32,
                bottom: false,
                waterlogged: false,
            }),
            19425u32 => Ok(GeneratedStruct17 {
                distance: 7i32,
                bottom: false,
                waterlogged: true,
            }),
            19426u32 => Ok(GeneratedStruct17 {
                waterlogged: false,
                bottom: false,
                distance: 7i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct17 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct17 {
                bottom: true,
                waterlogged: true,
                distance: 0i32,
            } => Ok(19395u32),
            GeneratedStruct17 {
                bottom: true,
                distance: 0i32,
                waterlogged: false,
            } => Ok(19396u32),
            GeneratedStruct17 {
                distance: 1i32,
                waterlogged: true,
                bottom: true,
            } => Ok(19397u32),
            GeneratedStruct17 {
                bottom: true,
                waterlogged: false,
                distance: 1i32,
            } => Ok(19398u32),
            GeneratedStruct17 {
                bottom: true,
                distance: 2i32,
                waterlogged: true,
            } => Ok(19399u32),
            GeneratedStruct17 {
                bottom: true,
                waterlogged: false,
                distance: 2i32,
            } => Ok(19400u32),
            GeneratedStruct17 {
                waterlogged: true,
                distance: 3i32,
                bottom: true,
            } => Ok(19401u32),
            GeneratedStruct17 {
                bottom: true,
                distance: 3i32,
                waterlogged: false,
            } => Ok(19402u32),
            GeneratedStruct17 {
                distance: 4i32,
                waterlogged: true,
                bottom: true,
            } => Ok(19403u32),
            GeneratedStruct17 {
                bottom: true,
                waterlogged: false,
                distance: 4i32,
            } => Ok(19404u32),
            GeneratedStruct17 {
                distance: 5i32,
                bottom: true,
                waterlogged: true,
            } => Ok(19405u32),
            GeneratedStruct17 {
                distance: 5i32,
                bottom: true,
                waterlogged: false,
            } => Ok(19406u32),
            GeneratedStruct17 {
                bottom: true,
                waterlogged: true,
                distance: 6i32,
            } => Ok(19407u32),
            GeneratedStruct17 {
                distance: 6i32,
                bottom: true,
                waterlogged: false,
            } => Ok(19408u32),
            GeneratedStruct17 {
                bottom: true,
                distance: 7i32,
                waterlogged: true,
            } => Ok(19409u32),
            GeneratedStruct17 {
                bottom: true,
                distance: 7i32,
                waterlogged: false,
            } => Ok(19410u32),
            GeneratedStruct17 {
                bottom: false,
                distance: 0i32,
                waterlogged: true,
            } => Ok(19411u32),
            GeneratedStruct17 {
                waterlogged: false,
                distance: 0i32,
                bottom: false,
            } => Ok(19412u32),
            GeneratedStruct17 {
                bottom: false,
                distance: 1i32,
                waterlogged: true,
            } => Ok(19413u32),
            GeneratedStruct17 {
                bottom: false,
                distance: 1i32,
                waterlogged: false,
            } => Ok(19414u32),
            GeneratedStruct17 {
                waterlogged: true,
                distance: 2i32,
                bottom: false,
            } => Ok(19415u32),
            GeneratedStruct17 {
                waterlogged: false,
                distance: 2i32,
                bottom: false,
            } => Ok(19416u32),
            GeneratedStruct17 {
                bottom: false,
                distance: 3i32,
                waterlogged: true,
            } => Ok(19417u32),
            GeneratedStruct17 {
                bottom: false,
                waterlogged: false,
                distance: 3i32,
            } => Ok(19418u32),
            GeneratedStruct17 {
                distance: 4i32,
                waterlogged: true,
                bottom: false,
            } => Ok(19419u32),
            GeneratedStruct17 {
                waterlogged: false,
                distance: 4i32,
                bottom: false,
            } => Ok(19420u32),
            GeneratedStruct17 {
                waterlogged: true,
                bottom: false,
                distance: 5i32,
            } => Ok(19421u32),
            GeneratedStruct17 {
                distance: 5i32,
                waterlogged: false,
                bottom: false,
            } => Ok(19422u32),
            GeneratedStruct17 {
                distance: 6i32,
                waterlogged: true,
                bottom: false,
            } => Ok(19423u32),
            GeneratedStruct17 {
                distance: 6i32,
                bottom: false,
                waterlogged: false,
            } => Ok(19424u32),
            GeneratedStruct17 {
                distance: 7i32,
                bottom: false,
                waterlogged: true,
            } => Ok(19425u32),
            GeneratedStruct17 {
                waterlogged: false,
                bottom: false,
                distance: 7i32,
            } => Ok(19426u32),
            _ => Err(()),
        }
    }
}
