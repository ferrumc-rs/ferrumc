use crate::block_state_id::BlockStateId;
use crate::chunk_format::{Chunk, PaletteType};
use crate::WorldError;
use ahash::AHashMap;
use ferrumc_general_purpose::data_packing::i32::read_nbit_i32;
use ferrumc_general_purpose::data_packing::u32::write_nbit_u32;
use ferrumc_macros::block;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::cmp::max;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use tracing::warn;

/// A batched block editing utility for a single Minecraft chunk.
///
/// `EditBatch` lets you queue many block edits and apply them all at once with high efficiency.
/// It deduplicates edits, compresses palette usage, and minimizes packed data writes.
///
/// # Example
/// ```
/// # use ferrumc_macros::block;
/// # use ferrumc_world::block_state_id::BlockStateId;
/// # use ferrumc_world::chunk_format::Chunk;
/// # use ferrumc_world::edit_batch::EditBatch;
/// # use ferrumc_world::vanilla_chunk_format::BlockData;
/// # let mut chunk = Chunk::new(0, 0, "overworld".to_string());
/// let mut batch = EditBatch::new(&mut chunk);
/// batch.set_block(1, 64, 1, block!("stone"));
/// batch.set_block(2, 64, 1, block!("stone"));
/// batch.apply().unwrap();
/// ```
///
/// `EditBatch` is single-use. After `apply()`, reuse it by creating a new one.
/// # Note
/// This is much faster than calling `set_block` for each block individually, but slower than filling
/// entire sections with the same block type. If you need to fill a section with the same block type, use
/// `Chunk::set_section` instead. However, there is a small amount of memory overhead for setting
/// up the batch, so if you only need to set one or two blocks, it's better to just call `set_block`
pub struct EditBatch<'a> {
    pub(crate) edits: Vec<Edit>,
    chunk: &'a mut Chunk,
    used: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Edit {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
    pub(crate) block: BlockStateId,
}

impl<'a> EditBatch<'a> {
    /// Creates a new `EditBatch` for the given chunk.
    ///
    /// This doesn't return the modified chunk, as the edits are applied in place.
    /// This means you should create the batch, add edits, apply and then use the original chunk
    /// you passed in.
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            edits: Vec::new(),
            chunk,
            used: false,
        }
    }

    /// Sets a block at the given chunk-relative coordinates.
    ///
    /// This won't have any effect until `apply()` is called.
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: BlockStateId) {
        self.edits.push(Edit { x, y, z, block });
    }

    /// Applies all edits in the batch to the chunk.
    ///
    /// This will modify the chunk in place and clear the batch.
    /// Will return an error if the batch has already been used or if there are no edits.
    pub fn apply(&mut self) -> Result<(), WorldError> {
        if self.used {
            return Err(WorldError::InvalidBatchingOperation(
                "EditBatch has already been used".to_string(),
            ));
        }
        if self.edits.is_empty() {
            return Err(WorldError::InvalidBatchingOperation(
                "No edits to apply".to_string(),
            ));
        }

        // Removed duplicates, last edit wins
        let mut unique_edits: AHashMap<(i32, i32, i32), BlockStateId> =
            AHashMap::with_capacity(self.edits.len());
        for edit in &self.edits {
            unique_edits.insert((edit.x, edit.y, edit.z), edit.block);
        }

        // Figure out which sections are affected
        let edited_sections: Vec<i8> = self
            .chunk
            .sections
            .iter()
            .filter(|section| {
                let section_y = section.y as i32 * 16;
                unique_edits
                    .keys()
                    .any(|&(_, y, _)| y >= section_y && y < section_y + 16)
            })
            .map(|section| section.y)
            .collect();

        // Apply edits to each affected section
        for &section_y in &edited_sections {
            let section = self
                .chunk
                .sections
                .iter_mut()
                .find(|s| s.y == section_y)
                .expect("Section should exist");

            let mut format_changed = false;
            let mut new_format = None;

            if let PaletteType::Single(block_data) = section.block_states.block_data {
                // If we have more than one unique block, we need to change format
                if unique_edits.len() > 1 {
                    format_changed = true;
                    new_format = Some(PaletteType::Indirect {
                        bits_per_block: 4, // Start with 4 bits per block
                        data: vec![0; 256],
                        palette: vec![VarInt(block_data.0)],
                    });
                } else {
                    // Check if the single block is different
                    let only_edit = unique_edits.values().next().unwrap();
                    if *only_edit != BlockStateId::from(block_data) {
                        // Change the single block
                        new_format = Some(PaletteType::Single(VarInt::from(*only_edit)));
                    }
                }
            }

            if format_changed {
                section.block_states.block_data = new_format.unwrap();
            }

            let section_edits: Vec<(&(i32, i32, i32), &BlockStateId)> = unique_edits
                .iter()
                .filter(|&(&(_, y, _), _)| {
                    let sec_y = section.y as i32 * 16;
                    y >= sec_y && y < sec_y + 16
                })
                .collect();

            // Build new palette
            let mut new_palette: Vec<VarInt> = match &section.block_states.block_data {
                PaletteType::Single(block_data) => vec![*block_data],
                PaletteType::Indirect { palette, .. } => palette.clone(),
                PaletteType::Direct { .. } => {
                    unimplemented!("Direct palette not implemented in EditBatch")
                }
            };

            for &(_, block) in &section_edits {
                let var_int = VarInt::from(*block);
                if !new_palette.contains(&var_int) {
                    new_palette.push(var_int);
                }
            }

            let bpe = max((new_palette.len() as f32).log2().ceil() as usize, 4);
            section.block_states.resize(bpe)?;

            let PaletteType::Indirect {
                bits_per_block,
                data,
                palette,
            } = &mut section.block_states.block_data
            else {
                return Err(WorldError::InvalidBlockStateData(
                    "Expected Indirect palette after resizing".to_string(),
                ));
            };

            *bits_per_block = bpe as u8;
            *palette = new_palette;

            // Apply edits
            let blocks_per_i64 = (64f64 / bpe as f64).floor() as usize;

            for &(&(x, y, z), block) in &section_edits {
                let index =
                    ((y.abs() & 0xf) * 256 + (z.abs() & 0xf) * 16 + (x.abs() & 0xf)) as usize;
                let i64_index = index / blocks_per_i64;
                let packed_u64 =
                    data.get_mut(i64_index)
                        .ok_or(WorldError::InvalidBlockStateData(format!(
                            "Invalid block state data at index {i64_index}"
                        )))?;
                let offset = (index % blocks_per_i64) * *bits_per_block as usize;
                let block_palette_index = palette
                    .iter()
                    .position(|&b| b == VarInt::from(*block))
                    .ok_or(WorldError::InvalidBlockStateData(format!(
                    "Block {:?} not found in palette",
                    block
                )))?;
                let old_value = read_nbit_i32(packed_u64, *bits_per_block as usize, offset as u32)
                    .map_err(|e| {
                        WorldError::InvalidBlockStateData(format!(
                            "Failed to read old block value: {e}"
                        ))
                    })?;
                if let Err(e) = write_nbit_u32(
                    packed_u64,
                    offset as u32,
                    block_palette_index as u32,
                    *bits_per_block,
                ) {
                    return Err(WorldError::InvalidBlockStateData(format!(
                        "Failed to write block: {e}"
                    )));
                }

                // Update block counts
                let old_block_state_id = if let Some(old_block) = palette.get(old_value as usize) {
                    BlockStateId::from(*old_block)
                } else {
                    continue; // Skip if old block not found
                };
                match section.block_states.block_counts.entry(old_block_state_id) {
                    Entry::Occupied(v) => {
                        let count = v.into_mut();
                        *count = count.saturating_sub(1);
                        if *count == 0 {
                            section
                                .block_states
                                .block_counts
                                .remove(&old_block_state_id);
                        }
                    }
                    Entry::Vacant(_) => {
                        warn!(
                            "Tried to decrement block count for {:?} but it was not found",
                            old_block_state_id
                        );
                    }
                }
                *section.block_states.block_counts.entry(*block).or_insert(0) += 1;
            }
            // Calculate non-air block count
            section.block_states.non_air_blocks = section
                .block_states
                .block_counts
                .iter()
                .filter(|(&block, _)| {
                    ![block!("air"), block!("cave_air"), block!("void_air")].contains(&block)
                })
                .map(|(_, &count)| count as u16)
                .sum();
            // section.optimise()?;
        }

        // Clear edits after applying
        self.edits.clear();
        self.used = true;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_format::Chunk;
    use ferrumc_macros::block;

    #[test]
    fn test_single_block_edit() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string());
        let block = block!("minecraft:stone");

        let mut batch = EditBatch::new(&mut chunk);
        batch.set_block(1, 1, 1, block);
        batch.apply().unwrap();

        let got = chunk.get_block(1, 1, 1).unwrap();
        assert_eq!(got, block);
    }

    #[test]
    fn test_multi_block_edits() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string());
        let stone = block!("minecraft:stone");
        let dirt = block!("minecraft:dirt");

        let mut batch = EditBatch::new(&mut chunk);
        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    let block = if (x + y + z) % 2 == 0 { stone } else { dirt };
                    batch.set_block(x, y, z, block);
                }
            }
        }
        batch.apply().unwrap();

        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    let expected = if (x + y + z) % 2 == 0 { &stone } else { &dirt };
                    let got = chunk.get_block(x, y, z).unwrap();
                    assert_eq!(&got, expected);
                }
            }
        }
    }
}
