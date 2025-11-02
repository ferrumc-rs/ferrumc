use crate::block_state_id::{BlockStateId, ID2BLOCK};
use crate::chunk_format::{BlockStates, Chunk, PaletteType, Section};
use crate::errors::WorldError;
use crate::World;
use ferrumc_general_purpose::data_packing::i32::read_nbit_i32;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, warn};

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
        x: i32,
        y: i32,
        z: i32,
        dimension: &str,
    ) -> Result<BlockStateId, WorldError> {
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let chunk = self.load_chunk(chunk_x, chunk_z, dimension)?;
        chunk.get_block(x, y, z)
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
        x: i32,
        y: i32,
        z: i32,
        dimension: &str,
        block: BlockStateId,
    ) -> Result<(), WorldError> {
        if ID2BLOCK.get(block.0 as usize).is_none() {
            return Err(WorldError::InvalidBlockStateId(block.0));
        };
        // Get chunk
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let mut chunk = self.load_chunk_owned(chunk_x, chunk_z, dimension)?;

        debug!("Chunk: {}, {}", chunk_x, chunk_z);

        chunk.set_block(x, y, z, block)?;
        for section in &mut chunk.sections {
            section.optimise()?;
        }

        // Save chunk
        self.save_chunk(Arc::new(chunk))?;
        Ok(())
    }
}

impl BlockStates {
    pub fn resize(&mut self, new_bit_size: usize) -> Result<(), WorldError> {
        match &mut self.block_data {
            PaletteType::Single(val) => {
                self.block_data = PaletteType::Indirect {
                    bits_per_block: new_bit_size as u8,
                    data: vec![],
                    palette: vec![*val; 1],
                }
            }
            PaletteType::Indirect {
                bits_per_block,
                data,
                palette,
            } => {
                // Step 1: Read existing packed data into a list of normal integers
                let mut normalised_ints = Vec::with_capacity(4096);
                let mut values_read = 0;

                for long in data {
                    let mut bit_offset = 0;

                    while bit_offset + *bits_per_block as usize <= 64 {
                        if values_read >= 4096 {
                            break;
                        }

                        // Extract value at the current bit offset
                        let value =
                            read_nbit_i32(long, *bits_per_block as usize, bit_offset as u32)?;
                        let max_int_value = (1 << new_bit_size) - 1;
                        if value > max_int_value {
                            return Err(WorldError::InvalidBlockStateData(format!(
                                "Value {value} exceeds maximum value for {new_bit_size}-bit block state"
                            )));
                        }
                        normalised_ints.push(value);
                        values_read += 1;

                        bit_offset += *bits_per_block as usize;
                    }

                    // Stop reading if we’ve already hit 4096 values
                    if values_read >= 4096 {
                        break;
                    }
                }

                // Check if we read exactly 4096 block states
                if normalised_ints.len() != 4096 {
                    return Err(WorldError::InvalidBlockStateData(format!(
                        "Expected 4096 block states, but got {}",
                        normalised_ints.len()
                    )));
                }

                // Step 2: Write the normalised integers into the new packed format
                let mut new_data = Vec::new();
                let mut current_long: i64 = 0;
                let mut bit_position = 0;

                for &value in &normalised_ints {
                    current_long |= (value as i64) << bit_position;
                    bit_position += new_bit_size;

                    if bit_position >= 64 {
                        new_data.push(current_long);
                        current_long = (value as i64) >> (new_bit_size - (bit_position - 64));
                        bit_position -= 64;
                    }
                }

                // Push any remaining bits in the final long
                if bit_position > 0 {
                    new_data.push(current_long);
                }

                // Verify the size of the new data matches expectations
                let expected_size = (4096 * new_bit_size).div_ceil(64);
                if new_data.len() != expected_size {
                    return Err(WorldError::InvalidBlockStateData(format!(
                        "Expected packed data size of {}, but got {}",
                        expected_size,
                        new_data.len()
                    )));
                }
                // Update the chunk with the new packed data and a bit size
                self.block_data = PaletteType::Indirect {
                    bits_per_block: new_bit_size as u8,
                    data: new_data,
                    palette: palette.clone(),
                }
            }
            _ => {
                todo!("Implement resizing for direct palette")
            }
        };
        Ok(())
    }
}

impl Chunk {
    /// Sets the block at the specified coordinates to the specified block data.
    /// If the block is the same as the old block, nothing happens.
    /// If the block is not in the palette, it is added.
    /// If the palette is in single block mode, it is converted to palette'd mode.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the block.
    /// * `y` - The y-coordinate of the block.
    /// * `z` - The z-coordinate of the block.
    /// * `block` - The block data to set the block to.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the block was successfully set.
    /// * `Err(WorldError)` - If an error occurs while setting the block.
    ///
    /// ### Note
    /// The positions are modulo'd by 16 to get the block index in the section anyway, so converting
    /// the coordinates to section coordinates isn't really necessary, but you should probably do it
    /// anyway for readability's sake.
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: BlockStateId) -> Result<(), WorldError> {
        // Get old block
        let old_block = self.get_block(x, y, z)?;
        if old_block == block {
            // debug!("Block is the same as the old block");
            return Ok(());
        }
        // Get section
        let section = self
            .sections
            .iter_mut()
            .find(|section| section.y == (y >> 4) as i8)
            .ok_or(WorldError::SectionOutOfBounds(y >> 4))?;

        let mut converted = false;
        let mut new_contents = PaletteType::Indirect {
            bits_per_block: 4,
            data: vec![],
            palette: vec![],
        };

        if let PaletteType::Single(val) = &section.block_states.block_data {
            new_contents = PaletteType::Indirect {
                bits_per_block: 4,
                data: vec![0; 256],
                palette: vec![*val],
            };
            converted = true;
        }

        if converted {
            section.block_states.block_data = new_contents;
        }

        // Do different things based on the palette type
        match &mut section.block_states.block_data {
            PaletteType::Single(_val) => {
                panic!("Single palette type should have been converted to indirect palette type");
            }
            PaletteType::Indirect {
                bits_per_block,
                data,
                palette,
            } => {
                // debug!("Indirect mode");
                match section.block_states.block_counts.entry(old_block) {
                    Entry::Occupied(mut occ_entry) => {
                        let count = occ_entry.get_mut();
                        if *count <= 0 {
                            return match old_block.to_block_data() {
                                Some(block_data) => {
                                    error!("Block count is zero for block: {:?}", block_data);
                                    Err(WorldError::InvalidBlockStateData(format!(
                                        "Block count is zero for block: {block_data:?}"
                                    )))
                                }
                                None => {
                                    error!(
                                        "Block count is zero for unknown block state ID: {}",
                                        old_block.0
                                    );
                                    Err(WorldError::InvalidBlockStateId(old_block.0))
                                }
                            };
                        }
                        *count -= 1;
                    }
                    Entry::Vacant(empty_entry) => {
                        warn!("Block not found in block counts: {:?}", old_block);
                        empty_entry.insert(0);
                    }
                }
                // Add new block
                if let Some(e) = section.block_states.block_counts.get(&block) {
                    section.block_states.block_counts.insert(block, e + 1);
                } else {
                    // debug!("Adding block to block counts");
                    section.block_states.block_counts.insert(block, 1);
                }
                // let required_bits = max((palette.len() as f32).log2().ceil() as u8, 4);
                // if *bits_per_block != required_bits {
                //     section.block_states.resize(required_bits as usize)?;
                // }
                // Get block index
                let block_palette_index = palette
                    .iter()
                    .position(|p| *p == block.to_varint())
                    .unwrap_or_else(|| {
                        // Add block to palette if it doesn't exist
                        let index = palette.len() as i16;
                        palette.push(block.to_varint());
                        index as usize
                    });
                // Set block
                let blocks_per_i64 = (64f64 / *bits_per_block as f64).floor() as usize;
                let index =
                    ((y.abs() & 0xf) * 256 + (z.abs() & 0xf) * 16 + (x.abs() & 0xf)) as usize;
                let i64_index = index / blocks_per_i64;
                let packed_u64 =
                    data.get_mut(i64_index)
                        .ok_or(WorldError::InvalidBlockStateData(format!(
                            "Invalid block state data at index {i64_index}"
                        )))?;
                let offset = (index % blocks_per_i64) * *bits_per_block as usize;
                if let Err(e) = ferrumc_general_purpose::data_packing::u32::write_nbit_u32(
                    packed_u64,
                    offset as u32,
                    block_palette_index as u32,
                    *bits_per_block,
                ) {
                    return Err(WorldError::InvalidBlockStateData(format!(
                        "Failed to write block: {e}"
                    )));
                }
            }
            PaletteType::Direct { .. } => {
                todo!("Implement direct palette for set_block");
            }
        }

        section.block_states.non_air_blocks = section
            .block_states
            .block_counts
            .iter()
            .filter(|(block, _)| {
                // Air, void air and cave air respectively
                ![0, 12958, 12959].contains(&block.0)
            })
            .map(|(_, count)| *count as u16)
            .sum();

        self.sections
            .iter_mut()
            .for_each(|section| section.optimise().unwrap());
        Ok(())
    }

    /// Gets the block at the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the block.
    /// * `y` - The y-coordinate of the block.
    /// * `z` - The z-coordinate of the block.
    ///
    /// # Returns
    ///
    /// * `Ok(BlockData)` - The block data at the specified coordinates.
    /// * `Err(WorldError)` - If an error occurs while retrieving the block data.
    ///
    /// ### Note
    /// The positions are modulo'd by 16 to get the block index in the section anyway, so converting
    /// the coordinates to section coordinates isn't really necessary, but you should probably do it
    /// anyway for readability's sake.
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Result<BlockStateId, WorldError> {
        let section = self
            .sections
            .iter()
            .find(|section| section.y == (y / 16) as i8)
            .ok_or(WorldError::SectionOutOfBounds(y >> 4))?;
        match &section.block_states.block_data {
            PaletteType::Single(val) => Ok(BlockStateId::from_varint(*val)),
            PaletteType::Indirect {
                bits_per_block,
                data,
                palette,
            } => {
                if palette.len() == 1 || *bits_per_block == 0 {
                    return Ok(BlockStateId::from_varint(palette[0]));
                }
                let blocks_per_i64 = (64f64 / *bits_per_block as f64).floor() as usize;
                let index = ((y & 0xf) * 256 + (z & 0xf) * 16 + (x & 0xf)) as usize;
                let i64_index = index / blocks_per_i64;
                let packed_u64 = data
                    .get(i64_index)
                    .ok_or(WorldError::InvalidBlockStateData(format!(
                        "Invalid block state data at index {i64_index}"
                    )))?;
                let offset = (index % blocks_per_i64) * *bits_per_block as usize;
                let id = ferrumc_general_purpose::data_packing::u32::read_nbit_u32(
                    packed_u64,
                    *bits_per_block,
                    offset as u32,
                )?;
                let palette_id = palette.get(id as usize).ok_or(WorldError::ChunkNotFound)?;
                Ok(BlockStateId::from_varint(*palette_id))
            }
            &PaletteType::Direct { .. } => todo!("Implement direct palette for get_block"),
        }
    }

    /// Sets the section at the specified index to the specified block data.
    /// If the section is out of bounds, an error is returned.
    ///
    /// # Arguments
    ///
    /// * `section` - The index of the section to set.
    /// * `block` - The block data to set the section to.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the section was successfully set.
    /// * `Err(WorldError)` - If an error occurs while setting the section.
    pub fn set_section(&mut self, section_y: i8, block: BlockStateId) -> Result<(), WorldError> {
        if let Some(section) = self
            .sections
            .iter_mut()
            .find(|section| section.y == section_y)
        {
            section.fill(block)
        } else {
            Err(WorldError::SectionOutOfBounds(section_y as i32))
        }
    }

    /// Fills the chunk with the specified block.
    ///
    /// # Arguments
    ///
    /// * `block` - The block data to fill the chunk with.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the chunk was successfully filled.
    /// * `Err(WorldError)` - If an error occurs while filling the chunk.
    pub fn fill(&mut self, block: BlockStateId) -> Result<(), WorldError> {
        for section in &mut self.sections {
            section.fill(block)?;
        }
        Ok(())
    }
}

impl Section {
    /// Fills the section with the specified block.
    ///
    /// # Arguments
    ///
    /// * `block` - The block data to fill the section with.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the section was successfully filled.
    /// * `Err(WorldError)` - If an error occurs while filling the section.
    pub fn fill(&mut self, block: BlockStateId) -> Result<(), WorldError> {
        self.block_states.block_data = PaletteType::Single(block.to_varint());
        self.block_states.block_counts = HashMap::from([(block, 4096)]);
        // Air, void air and cave air respectively
        if [0, 12958, 12959].contains(&block.0) {
            self.block_states.non_air_blocks = 0;
        } else {
            self.block_states.non_air_blocks = 4096;
        }
        Ok(())
    }

    /// This function trims out unnecessary data from the section. Primarily it does 2 things:
    ///
    /// 1. Removes any palette entries that are not used in the block states data.
    ///
    /// 2. If there is only one block in the palette, it converts the palette to single block mode.
    pub fn optimise(&mut self) -> Result<(), WorldError> {
        match &mut self.block_states.block_data {
            PaletteType::Single(_) => {
                // If the section is already in single block mode, there's nothing to optimise
                return Ok(());
            }
            PaletteType::Indirect {
                bits_per_block,
                data,
                palette,
            } => {
                // Remove empty blocks from palette
                let mut remove_indexes = Vec::new();
                for (block, count) in &self.block_states.block_counts {
                    if *count <= 0 {
                        let index = palette.iter().position(|p| *p == block.to_varint());
                        if let Some(index) = index {
                            remove_indexes.push(index);
                        } else {
                            return Err(WorldError::InvalidBlockStateId(block.0));
                        }
                    }
                }
                for index in remove_indexes {
                    // Decrement any data entries that are higher than the removed index
                    for data_point in &mut *data {
                        let mut i = 0;
                        while (i + *bits_per_block as usize) < 64 {
                            let block_index =
                                ferrumc_general_purpose::data_packing::u32::read_nbit_u32(
                                    data_point,
                                    *bits_per_block,
                                    i as u32,
                                )?;
                            if block_index > index as u32 {
                                ferrumc_general_purpose::data_packing::u32::write_nbit_u32(
                                    data_point,
                                    i as u32,
                                    block_index - 1,
                                    *bits_per_block,
                                )?;
                            }
                            i += *bits_per_block as usize;
                        }
                    }
                }

                {
                    // If there is only one block in the palette, convert to single block mode
                    if palette.len() == 1 {
                        let block = BlockStateId::from(palette[0]);
                        self.block_states.block_data = PaletteType::Single(palette[0]);
                        self.block_states.block_counts.clear();
                        self.block_states.block_counts.insert(block, 4096);
                    }
                }
            }
            PaletteType::Direct { .. } => {
                todo!("Implement optimisation for direct palette");
            }
        };

        Ok(())
    }
}
