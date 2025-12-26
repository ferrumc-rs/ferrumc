use crate::chunk::section::direct::DirectSection;
use crate::chunk::section::paletted::PalettedSection;
use crate::chunk::section::uniform::UniformSection;

mod uniform;
mod paletted;
mod direct;

pub const CHUNK_SECTION_LENGTH: usize = 16 * 16 * 16;

enum ChunkSectionType {
    Uniform(UniformSection),
    Paletted(PalettedSection),
    Direct(DirectSection),
}

pub struct ChunkSection {
    inner: ChunkSectionType,
    // todo: add light data
}