use crate::GrindstoneBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for GrindstoneBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            19461u32 => Ok(GrindstoneBlock {
                face: AttachFace::Floor,
                facing: Direction::North,
            }),
            19462u32 => Ok(GrindstoneBlock {
                face: AttachFace::Floor,
                facing: Direction::South,
            }),
            19463u32 => Ok(GrindstoneBlock {
                face: AttachFace::Floor,
                facing: Direction::West,
            }),
            19464u32 => Ok(GrindstoneBlock {
                face: AttachFace::Floor,
                facing: Direction::East,
            }),
            19465u32 => Ok(GrindstoneBlock {
                face: AttachFace::Wall,
                facing: Direction::North,
            }),
            19466u32 => Ok(GrindstoneBlock {
                face: AttachFace::Wall,
                facing: Direction::South,
            }),
            19467u32 => Ok(GrindstoneBlock {
                face: AttachFace::Wall,
                facing: Direction::West,
            }),
            19468u32 => Ok(GrindstoneBlock {
                face: AttachFace::Wall,
                facing: Direction::East,
            }),
            19469u32 => Ok(GrindstoneBlock {
                face: AttachFace::Ceiling,
                facing: Direction::North,
            }),
            19470u32 => Ok(GrindstoneBlock {
                face: AttachFace::Ceiling,
                facing: Direction::South,
            }),
            19471u32 => Ok(GrindstoneBlock {
                face: AttachFace::Ceiling,
                facing: Direction::West,
            }),
            19472u32 => Ok(GrindstoneBlock {
                face: AttachFace::Ceiling,
                facing: Direction::East,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GrindstoneBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GrindstoneBlock {
                face: AttachFace::Floor,
                facing: Direction::North,
            } => Ok(19461u32),
            GrindstoneBlock {
                face: AttachFace::Floor,
                facing: Direction::South,
            } => Ok(19462u32),
            GrindstoneBlock {
                face: AttachFace::Floor,
                facing: Direction::West,
            } => Ok(19463u32),
            GrindstoneBlock {
                face: AttachFace::Floor,
                facing: Direction::East,
            } => Ok(19464u32),
            GrindstoneBlock {
                face: AttachFace::Wall,
                facing: Direction::North,
            } => Ok(19465u32),
            GrindstoneBlock {
                face: AttachFace::Wall,
                facing: Direction::South,
            } => Ok(19466u32),
            GrindstoneBlock {
                face: AttachFace::Wall,
                facing: Direction::West,
            } => Ok(19467u32),
            GrindstoneBlock {
                face: AttachFace::Wall,
                facing: Direction::East,
            } => Ok(19468u32),
            GrindstoneBlock {
                face: AttachFace::Ceiling,
                facing: Direction::North,
            } => Ok(19469u32),
            GrindstoneBlock {
                face: AttachFace::Ceiling,
                facing: Direction::South,
            } => Ok(19470u32),
            GrindstoneBlock {
                face: AttachFace::Ceiling,
                facing: Direction::West,
            } => Ok(19471u32),
            GrindstoneBlock {
                face: AttachFace::Ceiling,
                facing: Direction::East,
            } => Ok(19472u32),
            _ => Err(()),
        }
    }
}
