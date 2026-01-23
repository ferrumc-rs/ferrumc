#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum GeneratedStruct85Type {
    HeavyWeightedPressurePlate,
    LightWeightedPressurePlate,
    Target,
}
#[allow(dead_code)]
pub struct GeneratedStruct85 {
    pub block_type: GeneratedStruct85Type,
    pub power: i32,
}
impl TryFrom<u32> for GeneratedStruct85 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            9968u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 0i32,
            }),
            9969u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 1i32,
            }),
            9970u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 2i32,
            }),
            9971u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 3i32,
            }),
            9972u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 4i32,
            }),
            9973u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 5i32,
            }),
            9974u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 6i32,
            }),
            9975u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 7i32,
            }),
            9976u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 8i32,
            }),
            9977u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 9i32,
            }),
            9978u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 10i32,
            }),
            9979u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 11i32,
            }),
            9980u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 12i32,
            }),
            9981u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 13i32,
            }),
            9982u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 14i32,
            }),
            9983u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 15i32,
            }),
            9952u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 0i32,
            }),
            9953u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 1i32,
            }),
            9954u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 2i32,
            }),
            9955u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 3i32,
            }),
            9956u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 4i32,
            }),
            9957u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 5i32,
            }),
            9958u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 6i32,
            }),
            9959u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 7i32,
            }),
            9960u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 8i32,
            }),
            9961u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 9i32,
            }),
            9962u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 10i32,
            }),
            9963u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 11i32,
            }),
            9964u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 12i32,
            }),
            9965u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 13i32,
            }),
            9966u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 14i32,
            }),
            9967u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 15i32,
            }),
            20409u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 0i32,
            }),
            20410u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 1i32,
            }),
            20411u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 2i32,
            }),
            20412u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 3i32,
            }),
            20413u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 4i32,
            }),
            20414u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 5i32,
            }),
            20415u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 6i32,
            }),
            20416u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 7i32,
            }),
            20417u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 8i32,
            }),
            20418u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 9i32,
            }),
            20419u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 10i32,
            }),
            20420u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 11i32,
            }),
            20421u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 12i32,
            }),
            20422u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 13i32,
            }),
            20423u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 14i32,
            }),
            20424u32 => Ok(GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 15i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct85 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 0i32,
            } => Ok(9968u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 1i32,
            } => Ok(9969u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 2i32,
            } => Ok(9970u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 3i32,
            } => Ok(9971u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 4i32,
            } => Ok(9972u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 5i32,
            } => Ok(9973u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 6i32,
            } => Ok(9974u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 7i32,
            } => Ok(9975u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 8i32,
            } => Ok(9976u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 9i32,
            } => Ok(9977u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 10i32,
            } => Ok(9978u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 11i32,
            } => Ok(9979u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 12i32,
            } => Ok(9980u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 13i32,
            } => Ok(9981u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 14i32,
            } => Ok(9982u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::HeavyWeightedPressurePlate,
                power: 15i32,
            } => Ok(9983u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 0i32,
            } => Ok(9952u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 1i32,
            } => Ok(9953u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 2i32,
            } => Ok(9954u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 3i32,
            } => Ok(9955u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 4i32,
            } => Ok(9956u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 5i32,
            } => Ok(9957u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 6i32,
            } => Ok(9958u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 7i32,
            } => Ok(9959u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 8i32,
            } => Ok(9960u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 9i32,
            } => Ok(9961u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 10i32,
            } => Ok(9962u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 11i32,
            } => Ok(9963u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 12i32,
            } => Ok(9964u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 13i32,
            } => Ok(9965u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 14i32,
            } => Ok(9966u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::LightWeightedPressurePlate,
                power: 15i32,
            } => Ok(9967u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 0i32,
            } => Ok(20409u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 1i32,
            } => Ok(20410u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 2i32,
            } => Ok(20411u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 3i32,
            } => Ok(20412u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 4i32,
            } => Ok(20413u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 5i32,
            } => Ok(20414u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 6i32,
            } => Ok(20415u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 7i32,
            } => Ok(20416u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 8i32,
            } => Ok(20417u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 9i32,
            } => Ok(20418u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 10i32,
            } => Ok(20419u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 11i32,
            } => Ok(20420u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 12i32,
            } => Ok(20421u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 13i32,
            } => Ok(20422u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 14i32,
            } => Ok(20423u32),
            GeneratedStruct85 {
                block_type: GeneratedStruct85Type::Target,
                power: 15i32,
            } => Ok(20424u32),
            _ => Err(()),
        }
    }
}
