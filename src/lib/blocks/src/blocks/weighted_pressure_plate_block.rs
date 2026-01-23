#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum WeightedPressurePlateBlockType {
    HeavyWeightedPressurePlate,
    LightWeightedPressurePlate,
    Target,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WeightedPressurePlateBlock {
    pub block_type: WeightedPressurePlateBlockType,
    pub power: i32,
}
impl WeightedPressurePlateBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<WeightedPressurePlateBlock>();
}
impl TryFrom<u32> for WeightedPressurePlateBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            9968u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 0i32,
            }),
            9969u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 1i32,
            }),
            9970u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 2i32,
            }),
            9971u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 3i32,
            }),
            9972u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 4i32,
            }),
            9973u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 5i32,
            }),
            9974u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 6i32,
            }),
            9975u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 7i32,
            }),
            9976u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 8i32,
            }),
            9977u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 9i32,
            }),
            9978u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 10i32,
            }),
            9979u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 11i32,
            }),
            9980u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 12i32,
            }),
            9981u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 13i32,
            }),
            9982u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 14i32,
            }),
            9983u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 15i32,
            }),
            9952u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 0i32,
            }),
            9953u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 1i32,
            }),
            9954u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 2i32,
            }),
            9955u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 3i32,
            }),
            9956u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 4i32,
            }),
            9957u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 5i32,
            }),
            9958u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 6i32,
            }),
            9959u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 7i32,
            }),
            9960u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 8i32,
            }),
            9961u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 9i32,
            }),
            9962u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 10i32,
            }),
            9963u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 11i32,
            }),
            9964u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 12i32,
            }),
            9965u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 13i32,
            }),
            9966u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 14i32,
            }),
            9967u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 15i32,
            }),
            20409u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 0i32,
            }),
            20410u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 1i32,
            }),
            20411u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 2i32,
            }),
            20412u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 3i32,
            }),
            20413u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 4i32,
            }),
            20414u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 5i32,
            }),
            20415u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 6i32,
            }),
            20416u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 7i32,
            }),
            20417u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 8i32,
            }),
            20418u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 9i32,
            }),
            20419u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 10i32,
            }),
            20420u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 11i32,
            }),
            20421u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 12i32,
            }),
            20422u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 13i32,
            }),
            20423u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 14i32,
            }),
            20424u32 => Ok(WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 15i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for WeightedPressurePlateBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 0i32,
            } => Ok(9968u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 1i32,
            } => Ok(9969u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 2i32,
            } => Ok(9970u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 3i32,
            } => Ok(9971u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 4i32,
            } => Ok(9972u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 5i32,
            } => Ok(9973u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 6i32,
            } => Ok(9974u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 7i32,
            } => Ok(9975u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 8i32,
            } => Ok(9976u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 9i32,
            } => Ok(9977u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 10i32,
            } => Ok(9978u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 11i32,
            } => Ok(9979u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 12i32,
            } => Ok(9980u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 13i32,
            } => Ok(9981u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 14i32,
            } => Ok(9982u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::HeavyWeightedPressurePlate,
                power: 15i32,
            } => Ok(9983u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 0i32,
            } => Ok(9952u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 1i32,
            } => Ok(9953u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 2i32,
            } => Ok(9954u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 3i32,
            } => Ok(9955u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 4i32,
            } => Ok(9956u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 5i32,
            } => Ok(9957u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 6i32,
            } => Ok(9958u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 7i32,
            } => Ok(9959u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 8i32,
            } => Ok(9960u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 9i32,
            } => Ok(9961u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 10i32,
            } => Ok(9962u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 11i32,
            } => Ok(9963u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 12i32,
            } => Ok(9964u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 13i32,
            } => Ok(9965u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 14i32,
            } => Ok(9966u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::LightWeightedPressurePlate,
                power: 15i32,
            } => Ok(9967u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 0i32,
            } => Ok(20409u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 1i32,
            } => Ok(20410u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 2i32,
            } => Ok(20411u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 3i32,
            } => Ok(20412u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 4i32,
            } => Ok(20413u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 5i32,
            } => Ok(20414u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 6i32,
            } => Ok(20415u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 7i32,
            } => Ok(20416u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 8i32,
            } => Ok(20417u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 9i32,
            } => Ok(20418u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 10i32,
            } => Ok(20419u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 11i32,
            } => Ok(20420u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 12i32,
            } => Ok(20421u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 13i32,
            } => Ok(20422u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 14i32,
            } => Ok(20423u32),
            WeightedPressurePlateBlock {
                block_type: WeightedPressurePlateBlockType::Target,
                power: 15i32,
            } => Ok(20424u32),
            _ => Err(()),
        }
    }
}
