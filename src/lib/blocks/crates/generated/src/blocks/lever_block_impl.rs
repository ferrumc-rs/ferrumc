use crate::LeverBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for LeverBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            5802u32 => Ok(LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::North,
                powered: true,
            }),
            5803u32 => Ok(LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::North,
                powered: false,
            }),
            5804u32 => Ok(LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::South,
                powered: true,
            }),
            5805u32 => Ok(LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::South,
                powered: false,
            }),
            5806u32 => Ok(LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::West,
                powered: true,
            }),
            5807u32 => Ok(LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::West,
                powered: false,
            }),
            5808u32 => Ok(LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::East,
                powered: true,
            }),
            5809u32 => Ok(LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::East,
                powered: false,
            }),
            5810u32 => Ok(LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::North,
                powered: true,
            }),
            5811u32 => Ok(LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::North,
                powered: false,
            }),
            5812u32 => Ok(LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::South,
                powered: true,
            }),
            5813u32 => Ok(LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::South,
                powered: false,
            }),
            5814u32 => Ok(LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::West,
                powered: true,
            }),
            5815u32 => Ok(LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::West,
                powered: false,
            }),
            5816u32 => Ok(LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::East,
                powered: true,
            }),
            5817u32 => Ok(LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::East,
                powered: false,
            }),
            5818u32 => Ok(LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::North,
                powered: true,
            }),
            5819u32 => Ok(LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::North,
                powered: false,
            }),
            5820u32 => Ok(LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::South,
                powered: true,
            }),
            5821u32 => Ok(LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::South,
                powered: false,
            }),
            5822u32 => Ok(LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::West,
                powered: true,
            }),
            5823u32 => Ok(LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::West,
                powered: false,
            }),
            5824u32 => Ok(LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::East,
                powered: true,
            }),
            5825u32 => Ok(LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::East,
                powered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for LeverBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::North,
                powered: true,
            } => Ok(5802u32),
            LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::North,
                powered: false,
            } => Ok(5803u32),
            LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::South,
                powered: true,
            } => Ok(5804u32),
            LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::South,
                powered: false,
            } => Ok(5805u32),
            LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::West,
                powered: true,
            } => Ok(5806u32),
            LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::West,
                powered: false,
            } => Ok(5807u32),
            LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::East,
                powered: true,
            } => Ok(5808u32),
            LeverBlock {
                face: AttachFace::Floor,
                facing: Direction::East,
                powered: false,
            } => Ok(5809u32),
            LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::North,
                powered: true,
            } => Ok(5810u32),
            LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::North,
                powered: false,
            } => Ok(5811u32),
            LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::South,
                powered: true,
            } => Ok(5812u32),
            LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::South,
                powered: false,
            } => Ok(5813u32),
            LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::West,
                powered: true,
            } => Ok(5814u32),
            LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::West,
                powered: false,
            } => Ok(5815u32),
            LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::East,
                powered: true,
            } => Ok(5816u32),
            LeverBlock {
                face: AttachFace::Wall,
                facing: Direction::East,
                powered: false,
            } => Ok(5817u32),
            LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::North,
                powered: true,
            } => Ok(5818u32),
            LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::North,
                powered: false,
            } => Ok(5819u32),
            LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::South,
                powered: true,
            } => Ok(5820u32),
            LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::South,
                powered: false,
            } => Ok(5821u32),
            LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::West,
                powered: true,
            } => Ok(5822u32),
            LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::West,
                powered: false,
            } => Ok(5823u32),
            LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::East,
                powered: true,
            } => Ok(5824u32),
            LeverBlock {
                face: AttachFace::Ceiling,
                facing: Direction::East,
                powered: false,
            } => Ok(5825u32),
            _ => Err(()),
        }
    }
}
