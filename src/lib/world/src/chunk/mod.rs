pub mod section;
mod palette;
pub mod light;
pub mod heightmap;
pub mod network;

use deepsize::DeepSizeOf;
use crate::chunk::heightmap::Heightmaps;
use crate::chunk::section::ChunkSection;
use crate::pos::ChunkBlockPos;

pub type BlockStateId = u16;

#[derive(Clone, DeepSizeOf)]
pub struct Chunk {
    pub sections: [ChunkSection; 24],
    min_y: i16,

    heightmaps: Option<Heightmaps>,
}

impl Chunk {
    pub fn new_empty() -> Chunk {
        Self {
            sections: core::array::from_fn(|_| ChunkSection::new_uniform(0)),
            min_y: -64,
            heightmaps: None,
        }
    }

    pub fn new_with_sections(sections: [ChunkSection; 24]) -> Chunk {
        Self {
            sections,
            min_y: -64,
            heightmaps: None,
        }
    }

    pub fn set_section(&mut self, y: i8, state: BlockStateId) {
        let section = y as i16 + -self.min_y / 16;
        assert!(section >= 0);

        self.sections[section as usize] = ChunkSection::new_uniform(state)
    }

    pub fn get_block(&self, pos: ChunkBlockPos) -> BlockStateId {
        let section = (pos.y() + -self.min_y) / 16;
        assert!(section >= 0);

        self.sections[section as usize].get_block(pos.section_block_pos())
    }

    pub fn set_block(&mut self, pos: ChunkBlockPos, id: BlockStateId) {
        let section = (pos.y() + -self.min_y) / 16;
        assert!(section >= 0);

        self.update_heightmaps(pos, id);

        self.sections[section as usize].set_block(pos.section_block_pos(), id);
    }

    pub fn update_heightmaps(&mut self, pos: ChunkBlockPos, block: BlockStateId) {
        // TODO: this should be implemented
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::{Chunk, ChunkBlockPos};

    #[test]
    fn test_read_write() {
        let mut chunk = Chunk::new_empty();

        chunk.set_block(ChunkBlockPos::new(0, 0, 0), 1);
        chunk.set_block(ChunkBlockPos::new(0, 16 ,1), 2);

        assert_eq!(chunk.get_block(ChunkBlockPos::new(0, 0, 0)), 1);
        assert_eq!(chunk.get_block(ChunkBlockPos::new(0, 16, 1)), 2);
    }
}