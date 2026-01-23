#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct86 {
    pub power: i32,
    pub sculk_sensor_phase: SculkSensorPhase,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct86 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            23347u32 => Ok(GeneratedStruct86 {
                power: 0i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            }),
            23348u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                power: 0i32,
            }),
            23349u32 => Ok(GeneratedStruct86 {
                power: 0i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23350u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 0i32,
            }),
            23351u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 0i32,
            }),
            23352u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
                power: 0i32,
            }),
            23353u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            }),
            23354u32 => Ok(GeneratedStruct86 {
                power: 1i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            }),
            23355u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23356u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23357u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
                power: 1i32,
            }),
            23358u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
                power: 1i32,
            }),
            23359u32 => Ok(GeneratedStruct86 {
                power: 2i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            }),
            23360u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
                power: 2i32,
            }),
            23361u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23362u32 => Ok(GeneratedStruct86 {
                power: 2i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23363u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            }),
            23364u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            }),
            23365u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            }),
            23366u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
                power: 3i32,
            }),
            23367u32 => Ok(GeneratedStruct86 {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23368u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23369u32 => Ok(GeneratedStruct86 {
                power: 3i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            }),
            23370u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 3i32,
                waterlogged: false,
            }),
            23371u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                power: 4i32,
                waterlogged: true,
            }),
            23372u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                power: 4i32,
            }),
            23373u32 => Ok(GeneratedStruct86 {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23374u32 => Ok(GeneratedStruct86 {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23375u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            }),
            23376u32 => Ok(GeneratedStruct86 {
                power: 4i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            }),
            23377u32 => Ok(GeneratedStruct86 {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23378u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            }),
            23379u32 => Ok(GeneratedStruct86 {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23380u32 => Ok(GeneratedStruct86 {
                power: 5i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23381u32 => Ok(GeneratedStruct86 {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23382u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            }),
            23383u32 => Ok(GeneratedStruct86 {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23384u32 => Ok(GeneratedStruct86 {
                power: 6i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            }),
            23385u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 6i32,
            }),
            23386u32 => Ok(GeneratedStruct86 {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23387u32 => Ok(GeneratedStruct86 {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23388u32 => Ok(GeneratedStruct86 {
                power: 6i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            }),
            23389u32 => Ok(GeneratedStruct86 {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23390u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                power: 7i32,
            }),
            23391u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 7i32,
                waterlogged: true,
            }),
            23392u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
                power: 7i32,
            }),
            23393u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 7i32,
                waterlogged: true,
            }),
            23394u32 => Ok(GeneratedStruct86 {
                power: 7i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            }),
            23395u32 => Ok(GeneratedStruct86 {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23396u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            }),
            23397u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 8i32,
                waterlogged: true,
            }),
            23398u32 => Ok(GeneratedStruct86 {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23399u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 8i32,
                waterlogged: true,
            }),
            23400u32 => Ok(GeneratedStruct86 {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23401u32 => Ok(GeneratedStruct86 {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23402u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                power: 9i32,
            }),
            23403u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23404u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23405u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 9i32,
            }),
            23406u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
                power: 9i32,
            }),
            23407u32 => Ok(GeneratedStruct86 {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23408u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
                power: 10i32,
            }),
            23409u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
                power: 10i32,
            }),
            23410u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 10i32,
            }),
            23411u32 => Ok(GeneratedStruct86 {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23412u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
                power: 10i32,
            }),
            23413u32 => Ok(GeneratedStruct86 {
                power: 11i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            }),
            23414u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
                power: 11i32,
            }),
            23415u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23416u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23417u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 11i32,
                waterlogged: true,
            }),
            23418u32 => Ok(GeneratedStruct86 {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23419u32 => Ok(GeneratedStruct86 {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23420u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            }),
            23421u32 => Ok(GeneratedStruct86 {
                power: 12i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23422u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
                power: 12i32,
            }),
            23423u32 => Ok(GeneratedStruct86 {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23424u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 12i32,
                waterlogged: false,
            }),
            23425u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
                power: 13i32,
            }),
            23426u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                power: 13i32,
                waterlogged: false,
            }),
            23427u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
                power: 13i32,
            }),
            23428u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
                power: 13i32,
            }),
            23429u32 => Ok(GeneratedStruct86 {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23430u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
                power: 13i32,
            }),
            23431u32 => Ok(GeneratedStruct86 {
                power: 14i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            }),
            23432u32 => Ok(GeneratedStruct86 {
                power: 14i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            }),
            23433u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 14i32,
                waterlogged: true,
            }),
            23434u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 14i32,
                waterlogged: false,
            }),
            23435u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            }),
            23436u32 => Ok(GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 14i32,
            }),
            23437u32 => Ok(GeneratedStruct86 {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23438u32 => Ok(GeneratedStruct86 {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23439u32 => Ok(GeneratedStruct86 {
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 15i32,
            }),
            23440u32 => Ok(GeneratedStruct86 {
                power: 15i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Active,
            }),
            23441u32 => Ok(GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
                power: 15i32,
            }),
            23442u32 => Ok(GeneratedStruct86 {
                power: 15i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct86 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct86 {
                power: 0i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            } => Ok(23347u32),
            GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                power: 0i32,
            } => Ok(23348u32),
            GeneratedStruct86 {
                power: 0i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23349u32),
            GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 0i32,
            } => Ok(23350u32),
            GeneratedStruct86 {
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 0i32,
            } => Ok(23351u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
                power: 0i32,
            } => Ok(23352u32),
            GeneratedStruct86 {
                waterlogged: true,
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            } => Ok(23353u32),
            GeneratedStruct86 {
                power: 1i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            } => Ok(23354u32),
            GeneratedStruct86 {
                waterlogged: true,
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23355u32),
            GeneratedStruct86 {
                waterlogged: false,
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23356u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
                power: 1i32,
            } => Ok(23357u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
                power: 1i32,
            } => Ok(23358u32),
            GeneratedStruct86 {
                power: 2i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            } => Ok(23359u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
                power: 2i32,
            } => Ok(23360u32),
            GeneratedStruct86 {
                waterlogged: true,
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23361u32),
            GeneratedStruct86 {
                power: 2i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23362u32),
            GeneratedStruct86 {
                waterlogged: true,
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            } => Ok(23363u32),
            GeneratedStruct86 {
                waterlogged: false,
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            } => Ok(23364u32),
            GeneratedStruct86 {
                waterlogged: true,
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            } => Ok(23365u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
                power: 3i32,
            } => Ok(23366u32),
            GeneratedStruct86 {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23367u32),
            GeneratedStruct86 {
                waterlogged: false,
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23368u32),
            GeneratedStruct86 {
                power: 3i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            } => Ok(23369u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 3i32,
                waterlogged: false,
            } => Ok(23370u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                power: 4i32,
                waterlogged: true,
            } => Ok(23371u32),
            GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                power: 4i32,
            } => Ok(23372u32),
            GeneratedStruct86 {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23373u32),
            GeneratedStruct86 {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23374u32),
            GeneratedStruct86 {
                waterlogged: true,
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            } => Ok(23375u32),
            GeneratedStruct86 {
                power: 4i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            } => Ok(23376u32),
            GeneratedStruct86 {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23377u32),
            GeneratedStruct86 {
                waterlogged: false,
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            } => Ok(23378u32),
            GeneratedStruct86 {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23379u32),
            GeneratedStruct86 {
                power: 5i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23380u32),
            GeneratedStruct86 {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23381u32),
            GeneratedStruct86 {
                waterlogged: false,
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            } => Ok(23382u32),
            GeneratedStruct86 {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23383u32),
            GeneratedStruct86 {
                power: 6i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            } => Ok(23384u32),
            GeneratedStruct86 {
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 6i32,
            } => Ok(23385u32),
            GeneratedStruct86 {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23386u32),
            GeneratedStruct86 {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23387u32),
            GeneratedStruct86 {
                power: 6i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            } => Ok(23388u32),
            GeneratedStruct86 {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23389u32),
            GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                power: 7i32,
            } => Ok(23390u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 7i32,
                waterlogged: true,
            } => Ok(23391u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
                power: 7i32,
            } => Ok(23392u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 7i32,
                waterlogged: true,
            } => Ok(23393u32),
            GeneratedStruct86 {
                power: 7i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            } => Ok(23394u32),
            GeneratedStruct86 {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23395u32),
            GeneratedStruct86 {
                waterlogged: false,
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            } => Ok(23396u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 8i32,
                waterlogged: true,
            } => Ok(23397u32),
            GeneratedStruct86 {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23398u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 8i32,
                waterlogged: true,
            } => Ok(23399u32),
            GeneratedStruct86 {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23400u32),
            GeneratedStruct86 {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23401u32),
            GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                power: 9i32,
            } => Ok(23402u32),
            GeneratedStruct86 {
                waterlogged: true,
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23403u32),
            GeneratedStruct86 {
                waterlogged: false,
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23404u32),
            GeneratedStruct86 {
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 9i32,
            } => Ok(23405u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
                power: 9i32,
            } => Ok(23406u32),
            GeneratedStruct86 {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23407u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
                power: 10i32,
            } => Ok(23408u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
                power: 10i32,
            } => Ok(23409u32),
            GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 10i32,
            } => Ok(23410u32),
            GeneratedStruct86 {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23411u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
                power: 10i32,
            } => Ok(23412u32),
            GeneratedStruct86 {
                power: 11i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            } => Ok(23413u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
                power: 11i32,
            } => Ok(23414u32),
            GeneratedStruct86 {
                waterlogged: true,
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23415u32),
            GeneratedStruct86 {
                waterlogged: false,
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23416u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 11i32,
                waterlogged: true,
            } => Ok(23417u32),
            GeneratedStruct86 {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23418u32),
            GeneratedStruct86 {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23419u32),
            GeneratedStruct86 {
                waterlogged: false,
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            } => Ok(23420u32),
            GeneratedStruct86 {
                power: 12i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23421u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
                power: 12i32,
            } => Ok(23422u32),
            GeneratedStruct86 {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23423u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 12i32,
                waterlogged: false,
            } => Ok(23424u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
                power: 13i32,
            } => Ok(23425u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                power: 13i32,
                waterlogged: false,
            } => Ok(23426u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
                power: 13i32,
            } => Ok(23427u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
                power: 13i32,
            } => Ok(23428u32),
            GeneratedStruct86 {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23429u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
                power: 13i32,
            } => Ok(23430u32),
            GeneratedStruct86 {
                power: 14i32,
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            } => Ok(23431u32),
            GeneratedStruct86 {
                power: 14i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
            } => Ok(23432u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 14i32,
                waterlogged: true,
            } => Ok(23433u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 14i32,
                waterlogged: false,
            } => Ok(23434u32),
            GeneratedStruct86 {
                waterlogged: true,
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            } => Ok(23435u32),
            GeneratedStruct86 {
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                power: 14i32,
            } => Ok(23436u32),
            GeneratedStruct86 {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23437u32),
            GeneratedStruct86 {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23438u32),
            GeneratedStruct86 {
                waterlogged: true,
                sculk_sensor_phase: SculkSensorPhase::Active,
                power: 15i32,
            } => Ok(23439u32),
            GeneratedStruct86 {
                power: 15i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Active,
            } => Ok(23440u32),
            GeneratedStruct86 {
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
                power: 15i32,
            } => Ok(23441u32),
            GeneratedStruct86 {
                power: 15i32,
                waterlogged: false,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
            } => Ok(23442u32),
            _ => Err(()),
        }
    }
}
