use crate::chunk_format::ID2BLOCK;
use crate::errors::WorldError;
use crate::vanilla_chunk_format::BlockData;
use crate::World;

impl World {
    /// Asynchronously retrieves the block data at the specified coordinates in the given dimension.
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
    pub async fn get_block(
        &self,
        x: i32,
        y: i32,
        z: i32,
        dimension: &str,
    ) -> Result<BlockData, WorldError> {
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let chunk = self.load_chunk(chunk_x, chunk_z, dimension).await?;
        let section = chunk
            .sections
            .iter()
            .find(|section| section.y == (y >> 4) as i8)
            .ok_or(WorldError::SectionOutOfBounds(y >> 4))?;
        if section.block_states.palette.len() == 1 {
            return ID2BLOCK
                .get(&section.block_states.palette[0].val)
                .cloned()
                .ok_or(WorldError::ChunkNotFound);
        }
        let bits_per_block = section.block_states.bits_per_block as usize;
        let data = &section.block_states.data;
        let blocks_per_i64 = (64f64 / bits_per_block as f64).floor() as usize;
        let index = ((y & 0xf) * 256 + (z & 0xf) * 16 + (x & 0xf)) as usize;
        let i64_index = index / blocks_per_i64;
        let packed_u64 = data
            .get(i64_index)
            .ok_or(WorldError::InvalidBlockStateData(format!(
                "Invalid block state data at index {}",
                i64_index
            )))?;
        let offset = (index % blocks_per_i64) * bits_per_block;
        let id = ferrumc_general_purpose::data_packing::u32::read_nbit_u32(
            packed_u64,
            bits_per_block as u8,
            offset as u32,
        )?;
        let palette_id = section
            .block_states
            .palette
            .get(id as usize)
            .ok_or(WorldError::ChunkNotFound)?;
        Ok(crate::chunk_format::ID2BLOCK
            .get(&palette_id.val)
            .unwrap_or(&BlockData::default())
            .clone())
    }
}
