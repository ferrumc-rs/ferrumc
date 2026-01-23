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
                crafting: true,
                orientation: FrontAndTop::DownEast,
                triggered: true,
            }),
            27651u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::DownEast,
                triggered: false,
            }),
            27652u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownNorth,
                crafting: true,
                triggered: true,
            }),
            27653u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownNorth,
                triggered: false,
                crafting: true,
            }),
            27654u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownSouth,
                crafting: true,
                triggered: true,
            }),
            27655u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownSouth,
                triggered: false,
                crafting: true,
            }),
            27656u32 => Ok(GeneratedStruct24 {
                crafting: true,
                triggered: true,
                orientation: FrontAndTop::DownWest,
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
                triggered: true,
                crafting: true,
                orientation: FrontAndTop::UpNorth,
            }),
            27661u32 => Ok(GeneratedStruct24 {
                triggered: false,
                crafting: true,
                orientation: FrontAndTop::UpNorth,
            }),
            27662u32 => Ok(GeneratedStruct24 {
                crafting: true,
                triggered: true,
                orientation: FrontAndTop::UpSouth,
            }),
            27663u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpSouth,
                triggered: false,
                crafting: true,
            }),
            27664u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::UpWest,
                triggered: true,
            }),
            27665u32 => Ok(GeneratedStruct24 {
                crafting: true,
                triggered: false,
                orientation: FrontAndTop::UpWest,
            }),
            27666u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::WestUp,
                triggered: true,
            }),
            27667u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::WestUp,
                crafting: true,
                triggered: false,
            }),
            27668u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                crafting: true,
                triggered: true,
            }),
            27669u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                crafting: true,
                triggered: false,
            }),
            27670u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::NorthUp,
                triggered: true,
            }),
            27671u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::NorthUp,
                triggered: false,
            }),
            27672u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::SouthUp,
                triggered: true,
            }),
            27673u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::SouthUp,
                crafting: true,
                triggered: false,
            }),
            27674u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownEast,
                triggered: true,
                crafting: false,
            }),
            27675u32 => Ok(GeneratedStruct24 {
                triggered: false,
                orientation: FrontAndTop::DownEast,
                crafting: false,
            }),
            27676u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownNorth,
                triggered: true,
                crafting: false,
            }),
            27677u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::DownNorth,
                triggered: false,
            }),
            27678u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::DownSouth,
                triggered: true,
            }),
            27679u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownSouth,
                crafting: false,
                triggered: false,
            }),
            27680u32 => Ok(GeneratedStruct24 {
                crafting: false,
                triggered: true,
                orientation: FrontAndTop::DownWest,
            }),
            27681u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownWest,
                triggered: false,
                crafting: false,
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
                orientation: FrontAndTop::UpNorth,
                triggered: true,
                crafting: false,
            }),
            27685u32 => Ok(GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::UpNorth,
            }),
            27686u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpSouth,
                triggered: true,
                crafting: false,
            }),
            27687u32 => Ok(GeneratedStruct24 {
                triggered: false,
                orientation: FrontAndTop::UpSouth,
                crafting: false,
            }),
            27688u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                crafting: false,
                triggered: true,
            }),
            27689u32 => Ok(GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::UpWest,
            }),
            27690u32 => Ok(GeneratedStruct24 {
                crafting: false,
                triggered: true,
                orientation: FrontAndTop::WestUp,
            }),
            27691u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::WestUp,
                crafting: false,
                triggered: false,
            }),
            27692u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                triggered: true,
                crafting: false,
            }),
            27693u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                crafting: false,
                triggered: false,
            }),
            27694u32 => Ok(GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::NorthUp,
            }),
            27695u32 => Ok(GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::NorthUp,
            }),
            27696u32 => Ok(GeneratedStruct24 {
                crafting: false,
                triggered: true,
                orientation: FrontAndTop::SouthUp,
            }),
            27697u32 => Ok(GeneratedStruct24 {
                triggered: false,
                orientation: FrontAndTop::SouthUp,
                crafting: false,
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
                crafting: true,
                orientation: FrontAndTop::DownEast,
                triggered: true,
            } => Ok(27650u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::DownEast,
                triggered: false,
            } => Ok(27651u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownNorth,
                crafting: true,
                triggered: true,
            } => Ok(27652u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownNorth,
                triggered: false,
                crafting: true,
            } => Ok(27653u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownSouth,
                crafting: true,
                triggered: true,
            } => Ok(27654u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownSouth,
                triggered: false,
                crafting: true,
            } => Ok(27655u32),
            GeneratedStruct24 {
                crafting: true,
                triggered: true,
                orientation: FrontAndTop::DownWest,
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
                triggered: true,
                crafting: true,
                orientation: FrontAndTop::UpNorth,
            } => Ok(27660u32),
            GeneratedStruct24 {
                triggered: false,
                crafting: true,
                orientation: FrontAndTop::UpNorth,
            } => Ok(27661u32),
            GeneratedStruct24 {
                crafting: true,
                triggered: true,
                orientation: FrontAndTop::UpSouth,
            } => Ok(27662u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpSouth,
                triggered: false,
                crafting: true,
            } => Ok(27663u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::UpWest,
                triggered: true,
            } => Ok(27664u32),
            GeneratedStruct24 {
                crafting: true,
                triggered: false,
                orientation: FrontAndTop::UpWest,
            } => Ok(27665u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::WestUp,
                triggered: true,
            } => Ok(27666u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::WestUp,
                crafting: true,
                triggered: false,
            } => Ok(27667u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                crafting: true,
                triggered: true,
            } => Ok(27668u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                crafting: true,
                triggered: false,
            } => Ok(27669u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::NorthUp,
                triggered: true,
            } => Ok(27670u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::NorthUp,
                triggered: false,
            } => Ok(27671u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::SouthUp,
                triggered: true,
            } => Ok(27672u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::SouthUp,
                crafting: true,
                triggered: false,
            } => Ok(27673u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownEast,
                triggered: true,
                crafting: false,
            } => Ok(27674u32),
            GeneratedStruct24 {
                triggered: false,
                orientation: FrontAndTop::DownEast,
                crafting: false,
            } => Ok(27675u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownNorth,
                triggered: true,
                crafting: false,
            } => Ok(27676u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::DownNorth,
                triggered: false,
            } => Ok(27677u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::DownSouth,
                triggered: true,
            } => Ok(27678u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownSouth,
                crafting: false,
                triggered: false,
            } => Ok(27679u32),
            GeneratedStruct24 {
                crafting: false,
                triggered: true,
                orientation: FrontAndTop::DownWest,
            } => Ok(27680u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownWest,
                triggered: false,
                crafting: false,
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
                orientation: FrontAndTop::UpNorth,
                triggered: true,
                crafting: false,
            } => Ok(27684u32),
            GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::UpNorth,
            } => Ok(27685u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpSouth,
                triggered: true,
                crafting: false,
            } => Ok(27686u32),
            GeneratedStruct24 {
                triggered: false,
                orientation: FrontAndTop::UpSouth,
                crafting: false,
            } => Ok(27687u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                crafting: false,
                triggered: true,
            } => Ok(27688u32),
            GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::UpWest,
            } => Ok(27689u32),
            GeneratedStruct24 {
                crafting: false,
                triggered: true,
                orientation: FrontAndTop::WestUp,
            } => Ok(27690u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::WestUp,
                crafting: false,
                triggered: false,
            } => Ok(27691u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                triggered: true,
                crafting: false,
            } => Ok(27692u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                crafting: false,
                triggered: false,
            } => Ok(27693u32),
            GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::NorthUp,
            } => Ok(27694u32),
            GeneratedStruct24 {
                crafting: false,
                triggered: false,
                orientation: FrontAndTop::NorthUp,
            } => Ok(27695u32),
            GeneratedStruct24 {
                crafting: false,
                triggered: true,
                orientation: FrontAndTop::SouthUp,
            } => Ok(27696u32),
            GeneratedStruct24 {
                triggered: false,
                orientation: FrontAndTop::SouthUp,
                crafting: false,
            } => Ok(27697u32),
            _ => Err(()),
        }
    }
}
