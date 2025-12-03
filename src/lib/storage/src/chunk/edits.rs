use crate::chunk::api::ChunkStorage;
use crate::errors::WorldError;
use ferrumc_core::world::block_state_id::BlockStateId;
use ferrumc_core::world::chunk_format::{BlockStates, Chunk, PaletteType, Section};
use ferrumc_general_purpose::data_packing::i32::read_nbit_i32;
use ferrumc_general_purpose::data_packing::u32::write_nbit_u32;
use ferrumc_net_codec::net_types::var_int::VarInt;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, warn};

// --- Constants ---
pub(crate) const SECTION_VOLUME: usize = 4096;
const AIR_ID: u32 = 0;
const VOID_AIR_ID: u32 = 12958;
const CAVE_AIR_ID: u32 = 12959;

// --- Helper Structs ---
pub(crate) struct StorageLocation {
    pub vec_index: usize,
    pub bit_offset: usize,
}

// --- ChunkStorage High-Level API ---

impl ChunkStorage {
    pub fn get_block_and_fetch(&self, x: i32, y: i32, z: i32) -> Result<BlockStateId, WorldError> {
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let chunk = self.get_chunk(chunk_x, chunk_z)?;
        chunk.get_block(x, y, z)
    }

    pub fn set_block_and_fetch(
        &self,
        x: i32,
        y: i32,
        z: i32,
        block: BlockStateId,
    ) -> Result<(), WorldError> {
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;

        // Copy-on-Write
        let mut chunk = self.load_chunk_owned(chunk_x, chunk_z)?;

        debug!("Chunk update at: {}, {}", chunk_x, chunk_z);

        chunk.set_block(x, y, z, block)?;

        // Optimise after modification
        for section in &mut chunk.sections {
            section.optimise()?;
        }

        self.save_chunk(Arc::new(chunk))?;
        Ok(())
    }
}

// --- Chunk Low-Level Logic ---

impl Chunk {
    /// Helper to calculate bit storage location.
    /// Used by both `edits.rs` and `edit_batch.rs`.
    #[inline]
    pub(crate) fn calculate_storage_loc(
        x: i32,
        y: i32,
        z: i32,
        bits_per_block: u8,
    ) -> StorageLocation {
        let lx = (x & 0xF) as usize;
        let ly = (y & 0xF) as usize;
        let lz = (z & 0xF) as usize;
        // Index: (Y * 16 * 16) + (Z * 16) + X
        let block_index = (ly * 256) + (lz * 16) + lx;

        let blocks_per_long = 64 / bits_per_block as usize;
        let vec_index = block_index / blocks_per_long;
        let bit_offset = (block_index % blocks_per_long) * bits_per_block as usize;

        StorageLocation {
            vec_index,
            bit_offset,
        }
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Result<BlockStateId, WorldError> {
        let section_y = (y >> 4) as i8;
        let section = self
            .sections
            .iter()
            .find(|s| s.y == section_y)
            .ok_or(WorldError::SectionOutOfBounds(y >> 4))?;

        match &section.block_states.block_data {
            PaletteType::Single(val) => Ok(BlockStateId(val.0 as u32)),
            PaletteType::Indirect {
                bits_per_block,
                data,
                palette,
            } => {
                if palette.is_empty() {
                    return Err(WorldError::InvalidBlockStateData("Empty palette".into()));
                }
                if palette.len() == 1 || *bits_per_block == 0 {
                    return Ok(BlockStateId(palette[0].0 as u32));
                }

                let loc = Self::calculate_storage_loc(x, y, z, *bits_per_block);

                let packed_i64 =
                    data.get(loc.vec_index)
                        .ok_or(WorldError::InvalidBlockStateData(
                            "Index out of bounds".into(),
                        ))?;

                // Read using &i64
                let id =
                    read_nbit_i32(packed_i64, *bits_per_block as usize, loc.bit_offset as u32)?;
                let pid = palette.get(id as usize).ok_or(WorldError::ChunkNotFound)?;

                Ok(BlockStateId(pid.0 as u32))
            }
            PaletteType::Direct => todo!("Direct palette get_block"),
        }
    }

    pub fn set_block(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        block: BlockStateId,
    ) -> Result<(), WorldError> {
        let old_block = self.get_block(x, y, z)?;
        if old_block == block {
            return Ok(());
        }

        let section_y = (y >> 4) as i8;
        let section = self
            .sections
            .iter_mut()
            .find(|s| s.y == section_y)
            .ok_or(WorldError::SectionOutOfBounds(y >> 4))?;

        // Promote Single -> Indirect
        if let PaletteType::Single(val) = &section.block_states.block_data {
            section.block_states.block_data = PaletteType::Indirect {
                bits_per_block: 4,
                data: vec![0; 256],
                palette: vec![*val],
            };
        }

        match &mut section.block_states.block_data {
            PaletteType::Single(_) => panic!("Unreachable"),
            PaletteType::Indirect {
                bits_per_block,
                data,
                palette,
            } => {
                // Update counts
                if let Entry::Occupied(mut e) = section.block_states.block_counts.entry(old_block) {
                    let c = e.get_mut();
                    if *c > 0 {
                        *c -= 1;
                    }
                }
                *section.block_states.block_counts.entry(block).or_insert(0) += 1;

                // Find/Add Palette Index
                let block_varint = VarInt(block.0 as i32);
                let palette_index = palette
                    .iter()
                    .position(|p| *p == block_varint)
                    .unwrap_or_else(|| {
                        let idx = palette.len();
                        palette.push(block_varint);
                        idx
                    });

                // Write Data
                let loc = Self::calculate_storage_loc(x, y, z, *bits_per_block);
                let packed_i64 =
                    data.get_mut(loc.vec_index)
                        .ok_or(WorldError::InvalidBlockStateData(
                            "Index out of bounds".into(),
                        ))?;

                write_nbit_u32(
                    packed_i64,
                    loc.bit_offset as u32,
                    palette_index as u32,
                    *bits_per_block,
                )
                .map_err(|e| WorldError::InvalidBlockStateData(e.to_string()))?;
            }
            PaletteType::Direct => todo!("Direct set_block"),
        }

        // Update Non-Air Count
        section.block_states.non_air_blocks = section
            .block_states
            .block_counts
            .iter()
            .filter(|(b, _)| ![AIR_ID, VOID_AIR_ID, CAVE_AIR_ID].contains(&b.0))
            .map(|(_, c)| *c as u16)
            .sum();

        Ok(())
    }

    pub fn set_section(&mut self, section_y: i8, block: BlockStateId) -> Result<(), WorldError> {
        if let Some(section) = self.sections.iter_mut().find(|s| s.y == section_y) {
            section.fill(block)?;
        } else {
            // Create new section if missing
            let mut new_section = Section {
                y: section_y,
                block_states: BlockStates {
                    non_air_blocks: 0,
                    block_data: PaletteType::Single(VarInt(0)),
                    block_counts: HashMap::new(),
                },
                biome_states: Default::default(),
                block_light: vec![0; 2048],
                sky_light: vec![255; 2048],
            };
            new_section.fill(block)?;
            self.sections.push(new_section);
        }
        Ok(())
    }

    pub fn fill(&mut self, block: BlockStateId) -> Result<(), WorldError> {
        for section in &mut self.sections {
            section.fill(block)?;
        }
        Ok(())
    }
}

impl Section {
    pub fn fill(&mut self, block: BlockStateId) -> Result<(), WorldError> {
        self.block_states.block_data = PaletteType::Single(VarInt(block.0 as i32));
        self.block_states.block_counts = HashMap::from([(block, SECTION_VOLUME as i32)]);

        if [AIR_ID, VOID_AIR_ID, CAVE_AIR_ID].contains(&block.0) {
            self.block_states.non_air_blocks = 0;
        } else {
            self.block_states.non_air_blocks = SECTION_VOLUME as u16;
        }
        Ok(())
    }

    pub fn optimise(&mut self) -> Result<(), WorldError> {
        match &mut self.block_states.block_data {
            PaletteType::Single(_) => Ok(()),
            PaletteType::Indirect {
                bits_per_block,
                data,
                palette,
            } => {
                // 1. Identify Unused
                let mut remove_indexes = Vec::new();
                for (block, count) in &self.block_states.block_counts {
                    if *count <= 0 {
                        let b_varint = VarInt(block.0 as i32);
                        if let Some(index) = palette.iter().position(|p| *p == b_varint) {
                            remove_indexes.push(index);
                        }
                    }
                }
                remove_indexes.sort_unstable_by(|a, b| b.cmp(a));

                // 2. Remap Data
                for index in remove_indexes {
                    for long in &mut *data {
                        let mut i = 0;
                        while (i + *bits_per_block as usize) <= 64 {
                            let val =
                                read_nbit_i32(long, *bits_per_block as usize, i as u32)? as usize;
                            if val > index {
                                write_nbit_u32(long, i as u32, (val - 1) as u32, *bits_per_block)
                                    .map_err(|e| WorldError::InvalidBlockStateData(e.to_string()))?;
                            }
                            i += *bits_per_block as usize;
                        }
                    }
                    if index < palette.len() {
                        palette.remove(index);
                    }
                }

                // 3. Convert to Single
                if palette.len() == 1 {
                    let block = BlockStateId(palette[0].0 as u32);
                    self.block_states.block_data = PaletteType::Single(palette[0]);
                    self.block_states.block_counts.clear();
                    self.block_states
                        .block_counts
                        .insert(block, SECTION_VOLUME as i32);
                }
                Ok(())
            }
            PaletteType::Direct => todo!("Optimize Direct"),
        }
    }
}
