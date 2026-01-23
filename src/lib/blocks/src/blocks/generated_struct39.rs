#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct39 {
    pub face: AttachFace,
    pub facing: Direction,
}
impl TryFrom<u32> for GeneratedStruct39 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            19461u32 => Ok(GeneratedStruct39 {
                face: AttachFace::Floor,
                facing: Direction::North,
            }),
            19462u32 => Ok(GeneratedStruct39 {
                facing: Direction::South,
                face: AttachFace::Floor,
            }),
            19463u32 => Ok(GeneratedStruct39 {
                face: AttachFace::Floor,
                facing: Direction::West,
            }),
            19464u32 => Ok(GeneratedStruct39 {
                face: AttachFace::Floor,
                facing: Direction::East,
            }),
            19465u32 => Ok(GeneratedStruct39 {
                facing: Direction::North,
                face: AttachFace::Wall,
            }),
            19466u32 => Ok(GeneratedStruct39 {
                face: AttachFace::Wall,
                facing: Direction::South,
            }),
            19467u32 => Ok(GeneratedStruct39 {
                facing: Direction::West,
                face: AttachFace::Wall,
            }),
            19468u32 => Ok(GeneratedStruct39 {
                face: AttachFace::Wall,
                facing: Direction::East,
            }),
            19469u32 => Ok(GeneratedStruct39 {
                face: AttachFace::Ceiling,
                facing: Direction::North,
            }),
            19470u32 => Ok(GeneratedStruct39 {
                facing: Direction::South,
                face: AttachFace::Ceiling,
            }),
            19471u32 => Ok(GeneratedStruct39 {
                face: AttachFace::Ceiling,
                facing: Direction::West,
            }),
            19472u32 => Ok(GeneratedStruct39 {
                face: AttachFace::Ceiling,
                facing: Direction::East,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct39 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct39 {
                face: AttachFace::Floor,
                facing: Direction::North,
            } => Ok(19461u32),
            GeneratedStruct39 {
                facing: Direction::South,
                face: AttachFace::Floor,
            } => Ok(19462u32),
            GeneratedStruct39 {
                face: AttachFace::Floor,
                facing: Direction::West,
            } => Ok(19463u32),
            GeneratedStruct39 {
                face: AttachFace::Floor,
                facing: Direction::East,
            } => Ok(19464u32),
            GeneratedStruct39 {
                facing: Direction::North,
                face: AttachFace::Wall,
            } => Ok(19465u32),
            GeneratedStruct39 {
                face: AttachFace::Wall,
                facing: Direction::South,
            } => Ok(19466u32),
            GeneratedStruct39 {
                facing: Direction::West,
                face: AttachFace::Wall,
            } => Ok(19467u32),
            GeneratedStruct39 {
                face: AttachFace::Wall,
                facing: Direction::East,
            } => Ok(19468u32),
            GeneratedStruct39 {
                face: AttachFace::Ceiling,
                facing: Direction::North,
            } => Ok(19469u32),
            GeneratedStruct39 {
                facing: Direction::South,
                face: AttachFace::Ceiling,
            } => Ok(19470u32),
            GeneratedStruct39 {
                face: AttachFace::Ceiling,
                facing: Direction::West,
            } => Ok(19471u32),
            GeneratedStruct39 {
                face: AttachFace::Ceiling,
                facing: Direction::East,
            } => Ok(19472u32),
            _ => Err(()),
        }
    }
}
