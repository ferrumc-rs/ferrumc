pub mod heightmap;
pub mod light;
pub mod network;
mod palette;
pub mod section;

use crate::block_state_id::BlockStateId;
use crate::chunk::heightmap::Heightmaps;
use crate::chunk::section::{ChunkSection, AIR};
use crate::errors::WorldError;
use crate::pos::{BlockPos, ChunkBlockPos, ChunkHeight};
use crate::vanilla_chunk_format::VanillaChunk;
use crate::World;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct Chunk {
    pub sections: Box<[ChunkSection]>,
    height: ChunkHeight,

    heightmaps: Option<Heightmaps>,
}

impl Chunk {
    pub fn new_empty() -> Chunk {
        Self {
            sections: vec![ChunkSection::new_uniform(AIR); 24].into_boxed_slice(),
            height: ChunkHeight::new(-64, 384),
            heightmaps: None,
        }
    }

    pub fn new_empty_with_height(height: ChunkHeight) -> Chunk {
        Self {
            sections: vec![ChunkSection::new_uniform(AIR); (height.height / 16) as usize].into_boxed_slice(),
            height,
            heightmaps: None,
        }
    }

    pub fn new_with_sections(sections: [ChunkSection; 24]) -> Chunk {
        Self {
            sections: sections.to_vec().into_boxed_slice(),
            height: ChunkHeight::new(-64, 384),
            heightmaps: None,
        }
    }

    pub fn fill_section(&mut self, y: i8, state: BlockStateId) {
        let section = y as i16 + -self.height.min_y / 16;
        assert!(section >= 0);

        self.sections[section as usize] = ChunkSection::new_uniform(state)
    }

    pub fn get_block(&self, pos: ChunkBlockPos) -> BlockStateId {
        let section = (pos.y() + -self.height.min_y) / 16;
        assert!(section >= 0);

        self.sections[section as usize].get_block(pos.section_block_pos())
    }

    pub fn set_block(&mut self, pos: ChunkBlockPos, id: BlockStateId) {
        let section = (pos.y() + -self.height.min_y) / 16;
        assert!(section >= 0);

        self.sections[section as usize].set_block(pos.section_block_pos(), id);
    }
}

impl TryFrom<&VanillaChunk> for Chunk {
    type Error = WorldError;

    fn try_from(_value: &VanillaChunk) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl World {
    /// Retrieves the block data at the specified coordinates in the given dimension.
    /// Under the hood, this function just fetches the chunk containing the block and then calls
    /// [`Chunk::get_block`] on it.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the block.
    /// * `y` - The y-coordinate of the block.
    /// * `z` - The z-coordinate of the block.
    /// * `dimension` - The dimension in which the block is located.
    ///
    /// # Returns
    ///
    /// * `Ok(BlockData)` - The block data at the specified coordinates.
    /// * `Err(WorldError)` - If an error occurs while retrieving the block data.
    ///
    /// # Errors
    ///
    /// * `WorldError::SectionOutOfBounds` - If the section containing the block is out of bounds.
    /// * `WorldError::ChunkNotFound` - If the chunk or block data is not found.
    /// * `WorldError::InvalidBlockStateData` - If the block state data is invalid.
    pub fn get_block_and_fetch(
        &self,
        pos: BlockPos,
        dimension: &str,
    ) -> Result<BlockStateId, WorldError> {
        let chunk = self.load_chunk(pos.chunk(), dimension)?;
        Ok(chunk.get_block(pos.chunk_block_pos()))
    }

    /// Sets the block data at the specified coordinates in the given dimension.
    /// Under the hood, this function just fetches the chunk containing the block and then calls
    /// [`Chunk::set_block`] on it.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the block.
    /// * `y` - The y-coordinate of the block.
    /// * `z` - The z-coordinate of the block.
    /// * `dimension` - The dimension in which the block is located.
    /// * `block` - The block data to set.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the block data is successfully set.
    /// * `Err(WorldError)` - If an error occurs while setting the block data.
    pub fn set_block_and_fetch(
        &self,
        pos: BlockPos,
        dimension: &str,
        block: BlockStateId,
    ) -> Result<(), WorldError> {
        let mut chunk = self.load_chunk_mut(pos.chunk(), dimension)?;
        chunk.set_block(pos.chunk_block_pos(), block);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::{BlockStateId, Chunk, ChunkBlockPos};
    use ferrumc_macros::block;

    #[test]
    fn test_read_write() {
        let mut chunk = Chunk::new_empty();

        chunk.set_block(ChunkBlockPos::new(0, 0, 0), block!("stone"));
        chunk.set_block(ChunkBlockPos::new(0, 16, 1), block!("dirt"));

        assert_eq!(
            chunk.get_block(ChunkBlockPos::new(0, 0, 0)),
            block!("stone")
        );
        assert_eq!(
            chunk.get_block(ChunkBlockPos::new(0, 16, 1)),
            block!("dirt")
        );
    }
}
