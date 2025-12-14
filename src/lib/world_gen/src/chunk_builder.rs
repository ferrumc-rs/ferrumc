//! Chunk builder for world generation.
//!
//! This module provides a `ChunkBuilder` that allows for fast random-access block placement
//! during terrain generation, then efficiently converts the raw data into a packed
//! `FerrumcChunk` format for network transmission and storage.
//!
//! # Example
//!
//! ```ignore
//! use ferrumc_world_gen::chunk_builder::ChunkBuilder;
//! use ferrumc_macros::block;
//!
//! let mut builder = ChunkBuilder::new(0, 0, -64, 384);
//! builder.set_block(0, 0, 0, block!("stone").raw());
//! let chunk = builder.build();
//! ```

use ahash::AHashMap;
use ferrumc_world::structure::{
    FerrumcChunk, FerrumcSection, PalettedContainer, BLOCKS_PER_SECTION,
    MAX_INDIRECT_BITS_BLOCKS, MIN_INDIRECT_BITS_BLOCKS,
};

/// Number of blocks per section (16³).
const SECTION_SIZE: usize = 4096;

/// Section state for optimization.
#[derive(Debug, Clone, Copy)]
enum SectionState {
    /// Section is filled with a single block type (ID stored).
    /// No need to iterate all blocks during build.
    Uniform(u32),
    /// Section has been modified with individual blocks.
    /// Must iterate all blocks during build.
    Mixed,
}

/// A builder for constructing `FerrumcChunk` instances during world generation.
///
/// This builder uses simple arrays for fast random-access block placement during
/// terrain generation, then efficiently packs the data into the palette-based
/// format used by `FerrumcChunk`.
///
/// # Performance
///
/// The builder stores blocks as raw `u32` state IDs in flat arrays, allowing O(1)
/// block access during generation. Sections filled with `fill_section()` are
/// marked as uniform and skip the expensive iteration during `build()`.
#[derive(Debug, Clone)]
pub struct ChunkBuilder {
    /// Chunk X coordinate.
    x: i32,
    /// Chunk Z coordinate.
    z: i32,
    /// Minimum Y level (e.g., -64 for overworld).
    min_y: i16,
    /// Total height in blocks (e.g., 384 for overworld).
    height: i16,
    /// Block data for each section. Key is section index, value is block state IDs.
    sections: Vec<[u32; SECTION_SIZE]>,
    /// Track section state for optimization.
    section_states: Vec<SectionState>,
}

impl ChunkBuilder {
    /// Creates a new `ChunkBuilder` for the given chunk coordinates and dimensions.
    ///
    /// # Arguments
    ///
    /// * `x` - Chunk X coordinate
    /// * `z` - Chunk Z coordinate
    /// * `min_y` - Minimum Y level (typically -64 for overworld, 0 for nether/end)
    /// * `height` - Total height in blocks (typically 384 for overworld, 256 for nether/end)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = ChunkBuilder::new(0, 0, -64, 384);
    /// ```
    #[must_use]
    pub fn new(x: i32, z: i32, min_y: i16, height: i16) -> Self {
        let section_count = (height / 16) as usize;
        // Initialize all sections with air (block state ID 0)
        let sections = vec![[0u32; SECTION_SIZE]; section_count];
        // All sections start as uniform (filled with air)
        let section_states = vec![SectionState::Uniform(0); section_count];

        Self {
            x,
            z,
            min_y,
            height,
            sections,
            section_states,
        }
    }

    /// Returns the number of sections in this chunk.
    #[inline]
    #[must_use]
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }

    /// Converts a world Y coordinate to a section index.
    ///
    /// Returns `None` if the Y coordinate is outside this chunk's bounds.
    #[inline]
    fn y_to_section_index(&self, y: i32) -> Option<usize> {
        let relative_y = y - i32::from(self.min_y);
        if relative_y < 0 || relative_y >= i32::from(self.height) {
            return None;
        }
        Some((relative_y / 16) as usize)
    }

    /// Converts local block coordinates to an index within a section array.
    ///
    /// # Arguments
    ///
    /// * `x` - Local X coordinate (0-15)
    /// * `y` - Local Y coordinate within section (0-15)
    /// * `z` - Local Z coordinate (0-15)
    #[inline]
    const fn block_index(x: u8, y: u8, z: u8) -> usize {
        // Standard Minecraft block index: Y * 256 + Z * 16 + X
        (y as usize) * 256 + (z as usize) * 16 + (x as usize)
    }

    /// Sets a block at the given local coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - Local X coordinate within chunk (0-15)
    /// * `y` - World Y coordinate
    /// * `z` - Local Z coordinate within chunk (0-15)
    /// * `block_id` - Block state ID to set
    ///
    /// # Panics
    ///
    /// Panics if coordinates are out of bounds.
    pub fn set_block(&mut self, x: u8, y: i32, z: u8, block_id: u32) {
        debug_assert!(x < 16, "X coordinate out of bounds: {}", x);
        debug_assert!(z < 16, "Z coordinate out of bounds: {}", z);

        let section_index = self
            .y_to_section_index(y)
            .expect("Y coordinate out of bounds");

        let local_y = ((y - i32::from(self.min_y)) % 16) as u8;
        let block_index = Self::block_index(x, local_y, z);

        self.sections[section_index][block_index] = block_id;
        // Mark section as mixed since we're setting individual blocks
        self.section_states[section_index] = SectionState::Mixed;
    }

    /// Gets the block at the given local coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - Local X coordinate within chunk (0-15)
    /// * `y` - World Y coordinate
    /// * `z` - Local Z coordinate within chunk (0-15)
    ///
    /// # Returns
    ///
    /// The block state ID at the given coordinates, or `None` if out of bounds.
    #[must_use]
    pub fn get_block(&self, x: u8, y: i32, z: u8) -> Option<u32> {
        if x >= 16 || z >= 16 {
            return None;
        }

        let section_index = self.y_to_section_index(y)?;
        let local_y = ((y - i32::from(self.min_y)) % 16) as u8;
        let block_index = Self::block_index(x, local_y, z);

        Some(self.sections[section_index][block_index])
    }

    /// Fills an entire section with a single block type.
    ///
    /// This is more efficient than setting blocks individually when filling
    /// large areas with the same block. Sections filled this way are optimized
    /// during `build()` to skip block-by-block iteration.
    ///
    /// # Arguments
    ///
    /// * `section_y` - The section Y index (not world Y coordinate)
    /// * `block_id` - Block state ID to fill with
    ///
    /// # Panics
    ///
    /// Panics if the section index is out of bounds.
    pub fn fill_section(&mut self, section_y: i8, block_id: u32) {
        let section_index = (section_y - (self.min_y / 16) as i8) as usize;
        if section_index < self.sections.len() {
            self.sections[section_index].fill(block_id);
            // Mark as uniform - build() can skip iteration for this section
            self.section_states[section_index] = SectionState::Uniform(block_id);
        }
    }

    /// Builds the final `FerrumcChunk` from the builder data.
    ///
    /// This method:
    /// 1. For uniform sections: Creates `PalettedContainer::Single` instantly
    /// 2. For mixed sections: Calculates palettes and packs data
    /// 3. Computes non-air block counts
    ///
    /// # Returns
    ///
    /// A `FerrumcChunk` ready for network transmission or storage.
    #[must_use]
    pub fn build(self) -> FerrumcChunk {
        let mut ferrumc_sections = Vec::with_capacity(self.sections.len());

        for (section_data, state) in self.sections.iter().zip(self.section_states.iter()) {
            let section = match state {
                SectionState::Uniform(block_id) => {
                    // Fast path: uniform section, no iteration needed
                    Self::build_uniform_section(*block_id)
                }
                SectionState::Mixed => {
                    // Slow path: mixed section, must iterate
                    Self::build_mixed_section(section_data)
                }
            };
            ferrumc_sections.push(section);
        }

        FerrumcChunk {
            x: self.x,
            z: self.z,
            min_y: self.min_y,
            height: self.height,
            sections: ferrumc_sections,
            heightmaps: Vec::new(),      // TODO: Calculate heightmaps
            block_entities: Vec::new(),  // World gen typically doesn't create block entities
        }
    }

    /// Builds a uniform section (all same block) without iteration.
    #[inline]
    fn build_uniform_section(block_id: u32) -> FerrumcSection {
        // Non-air count: 4096 if not air, 0 if air
        let non_air_blocks = if block_id == 0 { 0 } else { BLOCKS_PER_SECTION as u16 };
        
        FerrumcSection {
            block_count: non_air_blocks,
            block_states: PalettedContainer::Single(block_id),
            biomes: PalettedContainer::Single(0), // Default to plains biome
            sky_light: Some(vec![0xFF; 2048]),    // Full sky light
            block_light: Some(vec![0x00; 2048]),  // No block light
        }
    }

    /// Builds a mixed section from raw block data (requires iteration).
    fn build_mixed_section(blocks: &[u32; SECTION_SIZE]) -> FerrumcSection {
        // Count unique blocks and build palette using AHashMap for speed
        let mut block_counts: AHashMap<u32, u16> = AHashMap::default();
        for &block_id in blocks.iter() {
            *block_counts.entry(block_id).or_insert(0) += 1;
        }

        let unique_count = block_counts.len();

        // Calculate non-air blocks (assuming air = 0)
        let non_air_blocks = BLOCKS_PER_SECTION as u16
            - block_counts.get(&0).copied().unwrap_or(0);

        // Determine palette type based on unique block count
        let block_states = if unique_count == 1 {
            // Single block type - use Single palette
            let single_id = *block_counts.keys().next().unwrap();
            PalettedContainer::Single(single_id)
        } else {
            // Multiple block types - use Indirect palette
            let bits_per_entry = Self::calculate_bits_per_entry(unique_count);

            // Build palette (mapping from palette index to global block state ID)
            let mut palette: Vec<u32> = block_counts.keys().copied().collect();
            palette.sort_unstable(); // Consistent ordering

            // Build reverse mapping (global ID to palette index) using AHashMap
            let id_to_palette: AHashMap<u32, u32> = palette
                .iter()
                .enumerate()
                .map(|(idx, &id)| (id, idx as u32))
                .collect();

            // Pack data into u64 array
            let data = Self::pack_blocks(blocks, &id_to_palette, bits_per_entry);

            PalettedContainer::Indirect {
                bits_per_entry,
                palette,
                data,
            }
        };

        FerrumcSection {
            block_count: non_air_blocks,
            block_states,
            biomes: PalettedContainer::Single(0), // Default to plains biome
            sky_light: Some(vec![0xFF; 2048]),    // Full sky light
            block_light: Some(vec![0x00; 2048]),  // No block light
        }
    }

    /// Calculates the bits per entry needed for the given number of unique values.
    ///
    /// For block states, the minimum is 4 bits and maximum before switching to
    /// direct palette is 8 bits.
    fn calculate_bits_per_entry(unique_count: usize) -> u8 {
        let raw_bits = (unique_count as f64).log2().ceil() as u8;
        raw_bits.clamp(MIN_INDIRECT_BITS_BLOCKS, MAX_INDIRECT_BITS_BLOCKS)
    }

    /// Packs block indices into a u64 array using the Minecraft long array format.
    ///
    /// Entries are packed from LSB to MSB within each u64, with padding at the
    /// end of each u64 if entries don't divide evenly.
    fn pack_blocks(
        blocks: &[u32; SECTION_SIZE],
        id_to_palette: &AHashMap<u32, u32>,
        bits_per_entry: u8,
    ) -> Vec<u64> {
        let bits = bits_per_entry as usize;
        let entries_per_long = 64 / bits;
        let num_longs = (SECTION_SIZE + entries_per_long - 1) / entries_per_long;

        let mut data = vec![0u64; num_longs];
        let mask = (1u64 << bits) - 1;

        for (index, &block_id) in blocks.iter().enumerate() {
            let palette_index = u64::from(id_to_palette[&block_id]);
            let long_index = index / entries_per_long;
            let bit_offset = (index % entries_per_long) * bits;

            data[long_index] |= (palette_index & mask) << bit_offset;
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_builder() {
        let builder = ChunkBuilder::new(0, 0, -64, 384);
        assert_eq!(builder.section_count(), 24);
    }

    #[test]
    fn test_set_and_get_block() {
        let mut builder = ChunkBuilder::new(0, 0, -64, 384);
        
        // Set a stone block (ID 1 for testing)
        builder.set_block(0, 0, 0, 1);
        assert_eq!(builder.get_block(0, 0, 0), Some(1));
        
        // Default is air (0)
        assert_eq!(builder.get_block(1, 0, 0), Some(0));
    }

    #[test]
    fn test_fill_section() {
        let mut builder = ChunkBuilder::new(0, 0, -64, 384);
        
        // Fill section at Y=-64 (section index 0, section_y = -4)
        builder.fill_section(-4, 1);
        
        // All blocks in that section should be 1
        for x in 0..16u8 {
            for y in -64..(-64 + 16) {
                for z in 0..16u8 {
                    assert_eq!(builder.get_block(x, y, z), Some(1));
                }
            }
        }
    }

    #[test]
    fn test_build_empty_chunk() {
        let builder = ChunkBuilder::new(0, 0, -64, 384);
        let chunk = builder.build();
        
        assert_eq!(chunk.x, 0);
        assert_eq!(chunk.z, 0);
        assert_eq!(chunk.min_y, -64);
        assert_eq!(chunk.height, 384);
        assert_eq!(chunk.sections.len(), 24);
        
        // All sections should be air (single palette with ID 0)
        for section in &chunk.sections {
            assert!(matches!(section.block_states, PalettedContainer::Single(0)));
            assert_eq!(section.block_count, 0);
        }
    }

    #[test]
    fn test_build_mixed_section() {
        let mut builder = ChunkBuilder::new(0, 0, -64, 384);
        
        // Set some stone blocks
        for x in 0..8u8 {
            for z in 0..16u8 {
                builder.set_block(x, -64, z, 1); // Stone
            }
        }
        
        let chunk = builder.build();
        let section = &chunk.sections[0];
        
        // Should have half stone, half air
        assert_eq!(section.block_count, 128); // 8 * 16 = 128 stone blocks in first layer
        
        // Should be indirect palette (2 block types)
        assert!(matches!(section.block_states, PalettedContainer::Indirect { .. }));
    }

    #[test]
    fn test_calculate_bits_per_entry() {
        // 1 unique = would be single, but if forced to indirect: 4 bits min
        assert_eq!(ChunkBuilder::calculate_bits_per_entry(2), 4);
        assert_eq!(ChunkBuilder::calculate_bits_per_entry(16), 4);
        assert_eq!(ChunkBuilder::calculate_bits_per_entry(17), 5);
        assert_eq!(ChunkBuilder::calculate_bits_per_entry(32), 5);
        assert_eq!(ChunkBuilder::calculate_bits_per_entry(256), 8);
    }
}
