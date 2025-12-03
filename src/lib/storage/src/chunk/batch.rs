use crate::chunk::edits::SECTION_VOLUME; // Import constant
use crate::errors::WorldError;
use ferrumc_core::world::block_state_id::BlockStateId;
use ferrumc_core::world::chunk::{BlockStates, Chunk, PaletteType, Section};
use ferrumc_general_purpose::data_packing::u32::write_nbit_u32;
use ferrumc_net_codec::net_types::var_int::VarInt;

use ahash::AHashMap;
use std::collections::HashMap;

/// A batched block editing utility for a single Minecraft chunk.
pub struct EditBatch<'a> {
    pub(crate) edits: Vec<Edit>,
    chunk: &'a mut Chunk,
    used: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct Edit {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
    pub(crate) block: BlockStateId,
}

impl<'a> EditBatch<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            edits: Vec::new(),
            chunk,
            used: false,
        }
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: BlockStateId) {
        self.edits.push(Edit { x, y, z, block });
    }

    /// Applies all edits in the batch to the chunk.
    pub fn apply(&mut self) -> Result<(), WorldError> {
        if self.used {
            return Err(WorldError::InvalidBatchingOperation(
                "EditBatch has already been used".to_string(),
            ));
        }
        if self.edits.is_empty() {
            return Ok(());
        }

        // 1. Group edits by section Y index
        let mut section_edits: AHashMap<i8, Vec<&Edit>> = AHashMap::new();
        for edit in &self.edits {
            let section_y = (edit.y >> 4) as i8;
            section_edits.entry(section_y).or_default().push(edit);
        }

        // 2. Process each section
        for (section_y, edits) in section_edits {
            // A. Get or Create Section
            let section_idx = self.chunk.sections.iter().position(|s| s.y == section_y);
            let section = match section_idx {
                Some(idx) => &mut self.chunk.sections[idx],
                None => {
                    // Create new empty section
                    let new_section = Section {
                        y: section_y,
                        block_states: BlockStates {
                            non_air_blocks: 0,
                            block_data: PaletteType::Single(VarInt(0)),
                            block_counts: HashMap::from([(BlockStateId(0), SECTION_VOLUME as i32)]),
                        },
                        biome_states: Default::default(),
                        block_light: vec![0; 2048],
                        sky_light: vec![255; 2048],
                    };
                    self.chunk.sections.push(new_section);
                    self.chunk.sections.last_mut().unwrap()
                }
            };

            // B. Promote Single -> Indirect
            if let PaletteType::Single(val) = &section.block_states.block_data {
                section.block_states.block_data = PaletteType::Indirect {
                    bits_per_block: 4,
                    data: vec![0; 256], // 256 i64s * 64 bits = 16384 bits / 4 = 4096 blocks
                    palette: vec![*val],
                };
            }

            // C. Apply Edits
            if let PaletteType::Indirect {
                bits_per_block,
                data,
                palette,
            } = &mut section.block_states.block_data
            {
                for edit in edits {
                    let block_varint = VarInt(edit.block.0 as i32);

                    // Find or Add Palette Index
                    let palette_index = palette
                        .iter()
                        .position(|p| *p == block_varint)
                        .unwrap_or_else(|| {
                            let idx = palette.len();
                            palette.push(block_varint);
                            idx
                        });

                    // Use Helper from edits.rs
                    // Note: Ensure Chunk::calculate_storage_loc is pub(crate)
                    let loc = Chunk::calculate_storage_loc(edit.x, edit.y, edit.z, *bits_per_block);

                    let packed_i64 = data.get_mut(loc.vec_index).ok_or_else(|| {
                        WorldError::InvalidBlockStateData("Index out of bounds".into())
                    })?;

                    // Write bits (pass &mut i64 directly as per your utils)
                    write_nbit_u32(
                        packed_i64,
                        loc.bit_offset as u32,
                        palette_index as u32,
                        *bits_per_block,
                    )
                    .map_err(|e| WorldError::InvalidBlockStateData(e.to_string()))?;

                    // Update counts (Simplified increment only)
                    *section
                        .block_states
                        .block_counts
                        .entry(edit.block)
                        .or_insert(0) += 1;
                }
            }

            // D. Recalculate Non-Air
            // We iterate the block_counts map to sum up everything that isn't air
            // (Note: This is slightly expensive but accurate)
            section.block_states.non_air_blocks = section
                .block_states
                .block_counts
                .iter()
                .filter(|(id, _)| ![0, 12958, 12959].contains(&id.0))
                .map(|(_, count)| *count as u16)
                .sum();
        }

        self.edits.clear();
        self.used = true;
        Ok(())
    }
}
