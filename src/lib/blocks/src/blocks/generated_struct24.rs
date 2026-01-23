#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct24 {
    pub crafting: bool,
    pub orientation: FrontAndTop,
    pub triggered: bool,
}
impl TryFrom<u32> for GeneratedStruct24 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            27650u32 => Ok(GeneratedStruct24 {
                triggered: true,
                orientation: FrontAndTop::DownEast,
                crafting: true,
            }),
            27651u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::DownEast,
                triggered: false,
            }),
            27652u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownNorth,
                triggered: true,
                crafting: true,
            }),
            27653u32 => Ok(GeneratedStruct24 {
                triggered: false,
                crafting: true,
                orientation: FrontAndTop::DownNorth,
            }),
            27654u32 => Ok(GeneratedStruct24 {
                crafting: true,
                triggered: true,
                orientation: FrontAndTop::DownSouth,
            }),
            27655u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::DownSouth,
                triggered: false,
            }),
            27656u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownWest,
                triggered: true,
                crafting: true,
            }),
            27657u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::DownWest,
                triggered: false,
            }),
            27658u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpEast,
                triggered: true,
                crafting: true,
            }),
            27659u32 => Ok(GeneratedStruct24 {
                crafting: true,
                triggered: false,
                orientation: FrontAndTop::UpEast,
            }),
            27660u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::UpNorth,
                triggered: true,
            }),
            27661u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpNorth,
                triggered: false,
                crafting: true,
            }),
            27662u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpSouth,
                triggered: true,
                crafting: true,
            }),
            27663u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpSouth,
                crafting: true,
                triggered: false,
            }),
            27664u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                triggered: true,
                crafting: true,
            }),
            27665u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                triggered: false,
                crafting: true,
            }),
            27666u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::WestUp,
                triggered: true,
                crafting: true,
            }),
            27667u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::WestUp,
                triggered: false,
            }),
            27668u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::EastUp,
                triggered: true,
            }),
            27669u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                triggered: false,
                crafting: true,
            }),
            27670u32 => Ok(GeneratedStruct24 {
                triggered: true,
                crafting: true,
                orientation: FrontAndTop::NorthUp,
            }),
            27671u32 => Ok(GeneratedStruct24 {
                crafting: true,
                triggered: false,
                orientation: FrontAndTop::NorthUp,
            }),
            27672u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::SouthUp,
                crafting: true,
                triggered: true,
            }),
            27673u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::SouthUp,
                triggered: false,
            }),
            27674u32 => Ok(GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::DownEast,
            }),
            27675u32 => Ok(GeneratedStruct24 {
                triggered: false,
                crafting: false,
                orientation: FrontAndTop::DownEast,
            }),
            27676u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownNorth,
                crafting: false,
                triggered: true,
            }),
            27677u32 => Ok(GeneratedStruct24 {
                triggered: false,
                crafting: false,
                orientation: FrontAndTop::DownNorth,
            }),
            27678u32 => Ok(GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::DownSouth,
            }),
            27679u32 => Ok(GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::DownSouth,
            }),
            27680u32 => Ok(GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::DownWest,
            }),
            27681u32 => Ok(GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::DownWest,
            }),
            27682u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpEast,
                crafting: false,
                triggered: true,
            }),
            27683u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpEast,
                triggered: false,
                crafting: false,
            }),
            27684u32 => Ok(GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::UpNorth,
            }),
            27685u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpNorth,
                crafting: false,
                triggered: false,
            }),
            27686u32 => Ok(GeneratedStruct24 {
                triggered: true,
                orientation: FrontAndTop::UpSouth,
                crafting: false,
            }),
            27687u32 => Ok(GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::UpSouth,
            }),
            27688u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::UpWest,
                triggered: true,
            }),
            27689u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                triggered: false,
                crafting: false,
            }),
            27690u32 => Ok(GeneratedStruct24 {
                triggered: true,
                orientation: FrontAndTop::WestUp,
                crafting: false,
            }),
            27691u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::WestUp,
                triggered: false,
            }),
            27692u32 => Ok(GeneratedStruct24 {
                triggered: true,
                orientation: FrontAndTop::EastUp,
                crafting: false,
            }),
            27693u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::EastUp,
                triggered: false,
            }),
            27694u32 => Ok(GeneratedStruct24 {
                crafting: false,
                triggered: true,
                orientation: FrontAndTop::NorthUp,
            }),
            27695u32 => Ok(GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::NorthUp,
            }),
            27696u32 => Ok(GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::SouthUp,
            }),
            27697u32 => Ok(GeneratedStruct24 {
                triggered: false,
                crafting: false,
                orientation: FrontAndTop::SouthUp,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct24 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct24 {
                triggered: true,
                orientation: FrontAndTop::DownEast,
                crafting: true,
            } => Ok(27650u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::DownEast,
                triggered: false,
            } => Ok(27651u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownNorth,
                triggered: true,
                crafting: true,
            } => Ok(27652u32),
            GeneratedStruct24 {
                triggered: false,
                crafting: true,
                orientation: FrontAndTop::DownNorth,
            } => Ok(27653u32),
            GeneratedStruct24 {
                crafting: true,
                triggered: true,
                orientation: FrontAndTop::DownSouth,
            } => Ok(27654u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::DownSouth,
                triggered: false,
            } => Ok(27655u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownWest,
                triggered: true,
                crafting: true,
            } => Ok(27656u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::DownWest,
                triggered: false,
            } => Ok(27657u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpEast,
                triggered: true,
                crafting: true,
            } => Ok(27658u32),
            GeneratedStruct24 {
                crafting: true,
                triggered: false,
                orientation: FrontAndTop::UpEast,
            } => Ok(27659u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::UpNorth,
                triggered: true,
            } => Ok(27660u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpNorth,
                triggered: false,
                crafting: true,
            } => Ok(27661u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpSouth,
                triggered: true,
                crafting: true,
            } => Ok(27662u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpSouth,
                crafting: true,
                triggered: false,
            } => Ok(27663u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                triggered: true,
                crafting: true,
            } => Ok(27664u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                triggered: false,
                crafting: true,
            } => Ok(27665u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::WestUp,
                triggered: true,
                crafting: true,
            } => Ok(27666u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::WestUp,
                triggered: false,
            } => Ok(27667u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::EastUp,
                triggered: true,
            } => Ok(27668u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                triggered: false,
                crafting: true,
            } => Ok(27669u32),
            GeneratedStruct24 {
                triggered: true,
                crafting: true,
                orientation: FrontAndTop::NorthUp,
            } => Ok(27670u32),
            GeneratedStruct24 {
                crafting: true,
                triggered: false,
                orientation: FrontAndTop::NorthUp,
            } => Ok(27671u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::SouthUp,
                crafting: true,
                triggered: true,
            } => Ok(27672u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::SouthUp,
                triggered: false,
            } => Ok(27673u32),
            GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::DownEast,
            } => Ok(27674u32),
            GeneratedStruct24 {
                triggered: false,
                crafting: false,
                orientation: FrontAndTop::DownEast,
            } => Ok(27675u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownNorth,
                crafting: false,
                triggered: true,
            } => Ok(27676u32),
            GeneratedStruct24 {
                triggered: false,
                crafting: false,
                orientation: FrontAndTop::DownNorth,
            } => Ok(27677u32),
            GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::DownSouth,
            } => Ok(27678u32),
            GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::DownSouth,
            } => Ok(27679u32),
            GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::DownWest,
            } => Ok(27680u32),
            GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::DownWest,
            } => Ok(27681u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpEast,
                crafting: false,
                triggered: true,
            } => Ok(27682u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpEast,
                triggered: false,
                crafting: false,
            } => Ok(27683u32),
            GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::UpNorth,
            } => Ok(27684u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpNorth,
                crafting: false,
                triggered: false,
            } => Ok(27685u32),
            GeneratedStruct24 {
                triggered: true,
                orientation: FrontAndTop::UpSouth,
                crafting: false,
            } => Ok(27686u32),
            GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::UpSouth,
            } => Ok(27687u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::UpWest,
                triggered: true,
            } => Ok(27688u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                triggered: false,
                crafting: false,
            } => Ok(27689u32),
            GeneratedStruct24 {
                triggered: true,
                orientation: FrontAndTop::WestUp,
                crafting: false,
            } => Ok(27690u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::WestUp,
                triggered: false,
            } => Ok(27691u32),
            GeneratedStruct24 {
                triggered: true,
                orientation: FrontAndTop::EastUp,
                crafting: false,
            } => Ok(27692u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::EastUp,
                triggered: false,
            } => Ok(27693u32),
            GeneratedStruct24 {
                crafting: false,
                triggered: true,
                orientation: FrontAndTop::NorthUp,
            } => Ok(27694u32),
            GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::NorthUp,
            } => Ok(27695u32),
            GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::SouthUp,
            } => Ok(27696u32),
            GeneratedStruct24 {
                triggered: false,
                crafting: false,
                orientation: FrontAndTop::SouthUp,
            } => Ok(27697u32),
            _ => Err(()),
        }
    }
}
