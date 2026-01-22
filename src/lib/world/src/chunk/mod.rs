pub mod heightmap;
pub mod light;
pub mod network;
mod palette;
pub mod section;

use crate::block_state_id::BlockStateId;
use crate::chunk::heightmap::Heightmaps;
use crate::chunk::section::{ChunkSection, AIR};
use crate::errors::WorldError;
use crate::pos::{BlockPos, ChunkBlockPos, ChunkHeight, ChunkPos};
use crate::vanilla_chunk_format::VanillaChunk;
use crate::World;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;
use ferrumc_macros::block;
use tracing::debug;

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct Chunk {
    pub sections: Box<[ChunkSection]>,
    height: ChunkHeight,

    heightmaps: Option<Heightmaps>,
}

impl Chunk {
    /// Returns a chunk that is completely filled with air.
    ///
    /// This uses the overworld [`ChunkHeight`] (-64..320) as the chunk's height.
    ///
    /// # Returns
    ///
    /// * An empty chunk filled with air using the overworld [`ChunkHeight`].
    pub fn new_empty() -> Chunk {
        Self::new_empty_with_height(ChunkHeight::new(-64, 384))
    }

    /// Returns a chunk that is completely filled with air.
    ///
    /// # Arguments
    ///
    /// * `height` - The [`ChunkHeight`] that this chunk should be set to
    ///
    /// # Returns
    ///
    /// * An empty chunk filled with air using the given [`ChunkHeight`].
    pub fn new_empty_with_height(height: ChunkHeight) -> Chunk {
        Self {
            sections: vec![ChunkSection::new_uniform(AIR); (height.height / 16) as usize]
                .into_boxed_slice(),
            height,
            heightmaps: Some(Heightmaps::default()),
        }
    }

    /// Creates a chunk using the given sections and height.
    ///
    /// # Arguments
    ///
    /// * `sections` - The sections to fill the chunk with. These should be in order from the bottom of the world at index 0 and the top at the end of the slice.
    /// * `height` - The [`ChunkHeight`] to use.
    ///
    /// # Asserts
    ///
    /// * debug_assert_eq: `sections` contains enough [`ChunkSection`]s to fill the chunk based on the given [`ChunkHeight`].
    ///
    /// # Returns
    ///
    /// * A chunk using the given sections and [`ChunkHeight`]
    pub fn new_with_sections(sections: &[ChunkSection], height: ChunkHeight) -> Chunk {
        debug_assert_eq!(height.height as usize / 16, sections.len());

        Self {
            sections: sections.to_vec().into_boxed_slice(),
            height,
            heightmaps: Some(Heightmaps::default()),
        }
    }

    /// Fills an entire [`ChunkSection`] with the given block.
    ///
    /// # Arguments
    ///
    /// * `y` - The y of the section to fill.
    /// * `state` - The [`BlockStateId`] to fill the section with.
    ///
    /// # Asserts
    ///
    /// * `assert` - Checks if the given y value is in range of the height of the chunk.
    pub fn fill_section(&mut self, y: i8, state: BlockStateId) {
        assert!(y as i16 >= self.height.min_y / 16);
        assert!((y as i16) < (self.height.min_y + self.height.height as i16) / 16);

        let section = y as i16 + -self.height.min_y / 16;

        self.sections[section as usize] = ChunkSection::new_uniform(state)
    }

    /// Fills the entire chunk with the given block.
    ///
    /// # Arguments
    ///
    /// * `state` - The [`BlockStateId`] of the block to fill the chunk with.
    pub fn fill(&mut self, state: BlockStateId) {
        for section in &mut self.sections {
            *section = ChunkSection::new_uniform(state);
        }
    }

    /// Gets a block in the chunk.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the block to get.
    ///
    /// # Returns
    ///
    /// * The [`BlockStateId`] of the block at the requested position. If the position is above the maximum y of the chunk, air is always returned.
    ///   If the position is below the minimum y of the chunk, void air is always returned.
    pub fn get_block(&self, pos: ChunkBlockPos) -> BlockStateId {
        let section = (pos.y() + -self.height.min_y) / 16;
        if section < 0 {
            return block!("void_air");
        }

        if section as usize >= self.sections.len() {
            return block!("air");
        }

        self.sections[section as usize].get_block(pos.section_block_pos())
    }

    /// Sets a block in the chunk.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the block to set within the chunk.
    /// * `id` - The [`BlockStateId`] of the block to set.
    ///
    /// # Asserts
    ///
    /// * `assert` - Checks to ensure that the given position is in-bounds.
    pub fn set_block(&mut self, pos: ChunkBlockPos, id: BlockStateId) {
        let section = (pos.y() + -self.height.min_y) / 16;
        assert!(section >= 0);
        assert!(section as usize <= self.sections.len());

        self.sections[section as usize].set_block(pos.section_block_pos(), id);
    }

    fn section_index(&self, pos: ChunkBlockPos) -> Option<usize> {
        let idx = (pos.y() + -self.height.min_y) / 16;
        (idx >= 0).then_some(idx as usize)
    }

    pub fn get_section(&self, pos: ChunkBlockPos) -> Option<&ChunkSection> {
        self.section_index(pos)
            .and_then(|idx| self.sections.get(idx))
    }

    pub fn get_section_mut(&mut self, pos: ChunkBlockPos) -> Option<&mut ChunkSection> {
        self.section_index(pos)
            .and_then(|idx| self.sections.get_mut(idx))
    }
}

impl TryFrom<&VanillaChunk> for Chunk {
    type Error = WorldError;

    fn try_from(value: &VanillaChunk) -> Result<Self, Self::Error> {
        let mut sections = vec![ChunkSection::new_uniform(AIR); 24];

        if value.status != "minecraft:full" {
            return Err(WorldError::CorruptedChunkData(0, 0));
        }

        for section in value
            .sections
            .as_ref()
            .ok_or(WorldError::CorruptedChunkData(
                value.x_pos as _,
                value.z_pos as _,
            ))?
            .iter()
        {
            sections[(section.y + 4).clamp(0, 23) as usize] = ChunkSection::try_from(section)?;
        }

        Ok(Chunk {
            sections: sections.into_boxed_slice(),
            height: ChunkHeight::new(-64, 384),
            heightmaps: value
                .heightmaps
                .as_ref()
                .and_then(|v| Heightmaps::try_from(v).ok()),
        })
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
    use rayon::prelude::*;
    use std::thread;
    use std::time::{Duration, Instant};

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

    #[test]
    #[ignore]
    fn test_memory() {
        let now = Instant::now();

        let _chunks: Vec<_> = (0..16_000)
            .par_bridge()
            .map(|v| {
                println!("generating chunk {}", v);
                let mut chunk = Chunk::new_empty();

                for x in 0..16 {
                    for z in 0..16 {
                        for y in -64..70 {
                            chunk.set_block(ChunkBlockPos::new(x, y, z), block!("stone"));
                        }
                    }
                }

                chunk
            })
            .collect();

        println!("done. time elapsed: {:?}", now.elapsed());

        thread::sleep(Duration::from_secs(30))
    }
}
