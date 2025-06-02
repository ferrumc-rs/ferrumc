use crate::block_id::BlockId;
use crate::chunk_format::{BiomeStates, BlockStates, Chunk, PaletteType};
use crate::WorldError;
use ahash::{AHashMap, AHashSet, AHasher};
use ferrumc_general_purpose::data_packing::i32::read_nbit_i32;
use ferrumc_general_purpose::data_packing::u32::write_nbit_u32;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// A batched block editing utility for a single Minecraft chunk.
///
/// `EditBatch` lets you queue many block edits and apply them all at once with high efficiency.
/// It deduplicates edits, compresses palette usage, and minimizes packed data writes.
///
/// # Example
/// ```
/// # use ferrumc_world::chunk_format::Chunk;
/// # use ferrumc_world::edit_batch::EditBatch;
/// # use ferrumc_world::vanilla_chunk_format::BlockData;
/// # let mut chunk = Chunk::new(0, 0, "overworld".to_string());
/// let mut batch = EditBatch::new(&mut chunk);
/// batch.set_block(1, 64, 1, BlockData { name: "minecraft:stone".to_string(), properties: None });
/// batch.set_block(2, 64, 1, BlockData { name: "minecraft:bricks".to_string(), properties: None });
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
    tmp_palette_map: AHashMap<BlockId, usize>,
    used: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Edit {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
    pub(crate) block: BlockId,
}

fn get_palette_hash(palette: &[VarInt]) -> i32 {
    let mut rolling = 0;
    let mut hasher = AHasher::default();
    for block in palette.iter() {
        (rolling + block.0).hash(&mut hasher);
        rolling = hasher.finish() as i32;
    }
    rolling
}

impl<'a> EditBatch<'a> {
    /// Creates a new `EditBatch` for the given chunk.
    ///
    /// This doesn't return the modified chunk, as the edits are applied in place.
    /// This means you should create the batch, add edits, apply and then use the original chunk
    /// you passed in.
    pub fn new(chunk: &'a mut Chunk) -> Self {
        let map_capacity = 64;
        Self {
            edits: Vec::new(),
            chunk,
            tmp_palette_map: AHashMap::with_capacity(map_capacity),
            used: false,
        }
    }

    /// Sets a block at the given chunk-relative coordinates.
    ///
    /// This won't have any effect until `apply()` is called.
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: impl Into<BlockId>) {
        self.edits.push(Edit {
            x,
            y,
            z,
            block: block.into(),
        });
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

        let mut section_edits: AHashMap<i8, Vec<Option<&Edit>>> = AHashMap::new();
        let mut all_blocks = AHashSet::new();

        // Convert edits into per-section sparse arrays (Vec<Option<&Edit>>),
        // using block index (0..4095) as the key instead of hashing 3D coords
        for edit in &self.edits {
            let section_index = (edit.y >> 4) as i8;
            // Compute linear index within section (16x16x16 = 4096 blocks)
            let index = ((edit.y & 0xf) * 256 + (edit.z & 0xf) * 16 + (edit.x & 0xf)) as usize;
            let section_vec = section_edits
                .entry(section_index)
                .or_insert_with(|| vec![None; 4096]);
            section_vec[index] = Some(edit);
            all_blocks.insert(&edit.block);
        }

        for (section_y, edits_vec) in section_edits {
            if edits_vec.is_empty() || edits_vec.iter().all(|e| e.is_none()) {
                continue;
            }
            let section_maybe = self.chunk.sections.iter_mut().find(|s| s.y == section_y);
            // let first_edit = edits_vec
            //     .iter()
            //     .find(|e| e.is_some())
            //     .expect("Section should have at least one edit")
            //     .as_ref()
            //     .unwrap();
            let mut block_count_adds = AHashMap::new();
            let mut block_count_removes = AHashMap::new();

            let section = match section_maybe {
                Some(section) => {
                    // If the section exists, we can just use it
                    section
                }
                None => &mut {
                    // If the section doesn't exist, create it
                    let new_section = crate::chunk_format::Section {
                        y: section_y,
                        block_states: BlockStates {
                            non_air_blocks: 0,
                            block_data: PaletteType::Single(VarInt::default()),
                            block_counts: HashMap::from([(BlockId::default(), 4096)]),
                        },
                        // Biomes don't really matter for this, so we can just use empty data
                        biome_states: BiomeStates {
                            bits_per_biome: 0,
                            data: vec![],
                            palette: vec![],
                        },
                        block_light: vec![255; 2048],
                        sky_light: vec![255; 2048],
                    };
                    self.chunk.sections.push(new_section);
                    self.chunk
                        .sections
                        .iter_mut()
                        .find(|s| s.y == section_y)
                        .expect("Section should exist after push")
                },
            };

            // // check if all the edits in 1 section are the same
            // let all_same = edits_vec
            //     .iter()
            //     .flatten()
            //     .all(|edit| edit.block == first_edit.block);
            // // Check if applying all edits would result in a section full of the same block
            // if all_same {
            //     if section
            //         .block_states
            //         .block_counts
            //         .get(&first_edit.block)
            //         .unwrap_or(&0)
            //         + edits_vec.len() as i32
            //         == 4096
            //     {
            //         // If all blocks are the same, we can just set the whole section to that block
            //         section.fill(first_edit.block.clone())?;
            //     }
            //     continue;
            // }

            // Convert from Single to Indirect palette if needed to support multiple block types
            if let PaletteType::Single(val) = &section.block_states.block_data {
                section.block_states.block_data = PaletteType::Indirect {
                    bits_per_block: 4,
                    data: vec![0; 256],
                    palette: vec![*val],
                };
            }

            let PaletteType::Indirect {
                bits_per_block,
                data,
                palette,
            } = &mut section.block_states.block_data
            else {
                return Err(WorldError::InvalidBlockStateData(
                    "Unsupported palette type".to_string(),
                ));
            };

            // Hash current palette so we can detect changes after edits
            let palette_hash = get_palette_hash(palette);

            // Rebuild temporary palette index lookup (block ID -> palette index)
            self.tmp_palette_map.clear();
            for (i, p) in palette.iter().enumerate() {
                self.tmp_palette_map.insert(BlockId::from_varint(*p), i);
            }

            // Determine how many blocks fit into each i64 (based on bits per block)
            let blocks_per_i64 = (64f64 / *bits_per_block as f64).floor() as usize;

            for maybe_edit in edits_vec.iter() {
                let Some(edit) = maybe_edit else { continue };
                let index = ((edit.y & 0xf) * 256 + (edit.z & 0xf) * 16 + (edit.x & 0xf)) as usize;

                let palette_index = if let Some(&idx) = self.tmp_palette_map.get(&edit.block) {
                    idx
                } else {
                    let idx = palette.len();
                    palette.push(edit.block.to_varint());
                    self.tmp_palette_map.insert(edit.block, idx);
                    idx
                };

                // Calculate i64 slot and bit offset for packed storage
                let i64_index = index / blocks_per_i64;
                let offset = (index % blocks_per_i64) * (*bits_per_block as usize);

                debug_assert!(
                    i64_index < data.len(),
                    "i64_index {} out of bounds for data (len {})",
                    i64_index,
                    data.len()
                );

                // Unsafe is safe here because i64_index is verified by debug_assert
                let packed = unsafe { data.get_unchecked_mut(i64_index) };

                // get old block
                let old_block_index =
                    read_nbit_i32(packed, *bits_per_block as usize, offset as u32).map_err(
                        |e| WorldError::InvalidBlockStateData(format!("Unpacking error: {e}")),
                    )?;
                // If the block is the same, skip
                if old_block_index == palette_index as i32 {
                    continue;
                }

                if let Some(old_block_id) = palette.get(old_block_index as usize) {
                    if let Some(count) =
                        block_count_removes.get_mut(&BlockId::from_varint(*old_block_id))
                    {
                        *count -= 1;
                    } else {
                        block_count_removes.insert(BlockId::from_varint(*old_block_id), 1);
                    }
                }

                if let Some(count) = block_count_adds.get_mut(&edit.block) {
                    *count += 1;
                } else {
                    block_count_adds.insert(edit.block, 1);
                }

                write_nbit_u32(packed, offset as u32, palette_index as u32, *bits_per_block)
                    .map_err(|e| {
                        WorldError::InvalidBlockStateData(format!("Packing error: {e}"))
                    })?;
            }

            // Update block counts
            for (block_id, count) in block_count_adds {
                let current_count = section
                    .block_states
                    .block_counts
                    .entry(block_id)
                    .or_insert(0);
                *current_count += count;
            }

            for (block_id, count) in block_count_removes {
                let current_count = section
                    .block_states
                    .block_counts
                    .entry(block_id)
                    .or_insert(0);
                *current_count -= count;
            }

            section.block_states.non_air_blocks = *section
                .block_states
                .block_counts
                .get(&BlockId::default())
                .unwrap_or(&4096) as u16;

            // Only optimise if the palette changed after edits
            if get_palette_hash(palette) != palette_hash {
                section.optimise()?;
            }
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
    use crate::vanilla_chunk_format::BlockData;

    fn make_test_block(name: &str) -> BlockId {
        BlockData {
            name: name.to_string(),
            properties: None,
        }
        .to_block_id()
    }

    #[test]
    fn test_single_block_edit() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string());
        let block = make_test_block("minecraft:stone");

        let mut batch = EditBatch::new(&mut chunk);
        batch.set_block(1, 1, 1, block);
        batch.apply().unwrap();

        let got = chunk.get_block(1, 1, 1).unwrap();
        assert_eq!(got, block);
    }

    #[test]
    fn test_multi_block_edits() {
        let mut chunk = Chunk::new(0, 0, "overworld".to_string());
        let stone = make_test_block("minecraft:stone");
        let dirt = make_test_block("minecraft:dirt");

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
