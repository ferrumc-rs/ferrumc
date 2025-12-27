mod section;
mod palette;
mod light;
mod heightmap;
mod pos;

pub use pos::*;
use crate::chunk::section::ChunkSection;

pub type BlockStateId = u16;

pub struct Chunk {
    sections: [ChunkSection; 20],
    min_y: i16,
}

impl Chunk {
    pub fn new_empty() -> Chunk {
        Self {
            sections: core::array::from_fn(|_| ChunkSection::new_uniform(0)),
            min_y: -64,
        }
    }

    pub fn new_with_sections(sections: [ChunkSection; 20]) -> Chunk {
        Self {
            sections,
            min_y: -64,
        }
    }

    pub fn get_block(&self, pos: ChunkBlockPos) -> BlockStateId {
        let section = (pos.y + -self.min_y) / 16;
        assert!(section >= 0);

        self.sections[section as usize].get_block(pos.into())
    }

    pub fn set_block(&mut self, pos: ChunkBlockPos, id: BlockStateId) {
        let section = (pos.y + -self.min_y) / 16;
        assert!(section >= 0);

        self.sections[section as usize].set_block(pos.into(), id);
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::{Chunk, ChunkBlockPos};

    #[test]
    fn test_read_write() {
        let mut chunk = Chunk::new_empty();

        chunk.set_block(ChunkBlockPos { x: 0, y: 0, z: 0 }, 1);
        chunk.set_block(ChunkBlockPos { x: 0, y: 16, z: 1 }, 2);

        assert_eq!(chunk.get_block(ChunkBlockPos { x: 0, y: 0, z: 0 }), 1);
        assert_eq!(chunk.get_block(ChunkBlockPos { x: 0, y: 16, z: 1 }), 2);
    }
}