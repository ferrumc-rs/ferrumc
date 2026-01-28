use crate::CocoaBeansBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for CocoaBeansBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            8203u32 => Ok(CocoaBeansBlock {
                age: 0i32,
                facing: Direction::North,
            }),
            8204u32 => Ok(CocoaBeansBlock {
                age: 0i32,
                facing: Direction::South,
            }),
            8205u32 => Ok(CocoaBeansBlock {
                age: 0i32,
                facing: Direction::West,
            }),
            8206u32 => Ok(CocoaBeansBlock {
                age: 0i32,
                facing: Direction::East,
            }),
            8207u32 => Ok(CocoaBeansBlock {
                age: 1i32,
                facing: Direction::North,
            }),
            8208u32 => Ok(CocoaBeansBlock {
                age: 1i32,
                facing: Direction::South,
            }),
            8209u32 => Ok(CocoaBeansBlock {
                age: 1i32,
                facing: Direction::West,
            }),
            8210u32 => Ok(CocoaBeansBlock {
                age: 1i32,
                facing: Direction::East,
            }),
            8211u32 => Ok(CocoaBeansBlock {
                age: 2i32,
                facing: Direction::North,
            }),
            8212u32 => Ok(CocoaBeansBlock {
                age: 2i32,
                facing: Direction::South,
            }),
            8213u32 => Ok(CocoaBeansBlock {
                age: 2i32,
                facing: Direction::West,
            }),
            8214u32 => Ok(CocoaBeansBlock {
                age: 2i32,
                facing: Direction::East,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for CocoaBeansBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            CocoaBeansBlock {
                age: 0i32,
                facing: Direction::North,
            } => Ok(8203u32),
            CocoaBeansBlock {
                age: 0i32,
                facing: Direction::South,
            } => Ok(8204u32),
            CocoaBeansBlock {
                age: 0i32,
                facing: Direction::West,
            } => Ok(8205u32),
            CocoaBeansBlock {
                age: 0i32,
                facing: Direction::East,
            } => Ok(8206u32),
            CocoaBeansBlock {
                age: 1i32,
                facing: Direction::North,
            } => Ok(8207u32),
            CocoaBeansBlock {
                age: 1i32,
                facing: Direction::South,
            } => Ok(8208u32),
            CocoaBeansBlock {
                age: 1i32,
                facing: Direction::West,
            } => Ok(8209u32),
            CocoaBeansBlock {
                age: 1i32,
                facing: Direction::East,
            } => Ok(8210u32),
            CocoaBeansBlock {
                age: 2i32,
                facing: Direction::North,
            } => Ok(8211u32),
            CocoaBeansBlock {
                age: 2i32,
                facing: Direction::South,
            } => Ok(8212u32),
            CocoaBeansBlock {
                age: 2i32,
                facing: Direction::West,
            } => Ok(8213u32),
            CocoaBeansBlock {
                age: 2i32,
                facing: Direction::East,
            } => Ok(8214u32),
            _ => Err(()),
        }
    }
}
