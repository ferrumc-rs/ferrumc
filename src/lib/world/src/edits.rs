use crate::chunk_format::BLOCK2ID;
use crate::errors::WorldError;
use crate::vanilla_chunk_format::BlockData;
use crate::World;
use tracing::debug;

impl World {
    /// Asynchronously retrieves the block data at the specified coordinates in the given dimension.
    /// Under the hood, this function just fetches the chunk containing the block and then calls
    /// [`crate::chunk_format::Chunk::get_block`] on it.
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
        x: i32,
        y: i32,
        z: i32,
        dimension: &str,
    ) -> Result<BlockData, WorldError> {
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let chunk = self.load_chunk(chunk_x, chunk_z, dimension)?;
        chunk.get_block(x, y, z)
    }

    /// Asynchronously sets the block data at the specified coordinates in the given dimension.
    /// Under the hood, this function just fetches the chunk containing the block and then calls
    /// [`crate::chunk_format::Chunk::set_block`] on it.
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
        x: i32,
        y: i32,
        z: i32,
        dimension: &str,
        block: BlockData,
    ) -> Result<(), WorldError> {
        if !BLOCK2ID.contains_key(&block) {
            return Err(WorldError::InvalidBlock(block));
        };
        // Get chunk
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let mut chunk = self.load_chunk(chunk_x, chunk_z, dimension)?;

        debug!("Chunk: {}, {}", chunk_x, chunk_z);

        chunk.set_block(x, y, z, block)?;
        for section in &mut chunk.sections {
            section.optimise()?;
        }

        // Save chunk
        self.save_chunk(chunk)?;
        Ok(())
    }
}
