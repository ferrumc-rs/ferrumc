use crate::chunk_format::{BLOCK2ID, ID2BLOCK};
use crate::errors::WorldError;
use crate::errors::WorldError::InvalidBlockStateData;
use crate::vanilla_chunk_format::BlockData;
use crate::World;
use std::cmp::max;

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

    pub async fn set_block(
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
        let mut chunk = self.load_chunk(chunk_x, chunk_z, dimension).await?;
        let section = chunk
            .sections
            .iter_mut()
            .find(|section| section.y == (y >> 4) as i8)
            .ok_or(WorldError::SectionOutOfBounds(y >> 4))?;
        let mut bits_per_block = section.block_states.bits_per_block;
        // Remove old block
        let old_block = self.get_block(x, y, z, dimension).await?;
        let old_block_count = section
            .block_states
            .block_counts
            .get_mut(&old_block)
            .expect("Block not found");
        *old_block_count -= 1;
        if *old_block_count == 0 {
            section.block_states.block_counts.remove(&old_block);
        }
        let block_id = BLOCK2ID
            .get(&block)
            .ok_or(WorldError::InvalidBlock(block.clone()))?;
        // Add new block
        if let Some(e) = section.block_states.block_counts.get(&block) {
            section.block_states.block_counts.insert(block, e + 1);
        } else {
            section.block_states.block_counts.insert(block, 1);
        }
        // Check if we need to resize bits_per_block
        let new_bits_per_block = max(
            (section.block_states.block_counts.len() as f32)
                .log2()
                .ceil() as u8,
            4,
        );
        if new_bits_per_block != bits_per_block {
            section.block_states.resize(new_bits_per_block as usize)?;
            bits_per_block = new_bits_per_block;
        }
        // Get block index
        let mut block_palette_index = -1i16;
        for (index, palette) in section.block_states.palette.iter().enumerate() {
            if palette.val == *block_id {
                block_palette_index = index as i16;
                break;
            }
        }
        // Add block to palette if it doesn't exist
        if block_palette_index == -1 {
            block_palette_index = section.block_states.palette.len() as i16;
            section.block_states.palette.push((*block_id).into());
        }
        // Set block
        let blocks_per_i64 = (64f64 / bits_per_block as f64).floor() as usize;
        let index = ((y & 0xf) * 256 + (z & 0xf) * 16 + (x & 0xf)) as usize;
        let i64_index = index / blocks_per_i64;
        let packed_u64 =
            section
                .block_states
                .data
                .get_mut(i64_index)
                .ok_or(InvalidBlockStateData(format!(
                    "Invalid block state data at index {}",
                    i64_index
                )))?;
        let offset = (index % blocks_per_i64) * bits_per_block as usize;
        if let Err(e) = ferrumc_general_purpose::data_packing::u32::write_nbit_u32(
            packed_u64,
            offset as u32,
            block_palette_index as u32,
            bits_per_block,
        ) {
            return Err(InvalidBlockStateData(format!(
                "Failed to write block: {}",
                e
            )));
        }
        // Save chunk
        self.save_chunk(chunk).await?;
        // TODO: Remove empty palette entries
        Ok(())
    }
}
