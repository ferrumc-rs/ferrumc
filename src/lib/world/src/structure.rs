//! Packet-optimized chunk data structures.
//!
//! This module defines chunk structures that are designed to be **network-native**,
//! meaning they closely mirror the Minecraft protocol format (1.21.x) for efficient
//! serialization and minimal transformation when sending to clients.
//!
//! # Design Goals
//!
//! 1. **No HashMaps** - Use paletted containers with packed data for O(1) access
//! 2. **Protocol Native** - Structure matches the wire format exactly
//! 3. **Memory Efficient** - Packed bit arrays minimize memory footprint
//! 4. **Zero-Copy Friendly** - Raw byte arrays for heightmaps and block entities
//!
//! # Architecture
//!
//! ```text
//! FerrumcChunk
//! ├── metadata (x, z, min_y, height)
//! ├── sections: Vec<FerrumcSection>
//! │   └── FerrumCSection
//! │       ├── block_count: u16
//! │       ├── block_states: PalettedContainer
//! │       ├── biomes: PalettedContainer
//! │       └── light data (optional)
//! ├── heightmaps: Vec<u8>  (raw NBT)
//! └── block_entities: Vec<u8>  (raw NBT)
//! ```
//!
//! # Paletted Container
//!
//! The `PalettedContainer` enum represents the three palette formats used by Minecraft:
//!
//! - **Single**: All entries are the same value (0 bits per entry)
//! - **Indirect**: Small palette with indices packed into `u64` array
//! - **Direct**: No palette, global IDs packed directly (15 bits for blocks, 6 for biomes)

// ============================================================================
// Chunk Structure
// ============================================================================

/// A chunk in the packet-optimized format.
///
/// This structure is designed to be serialized directly to the Minecraft protocol
/// with minimal transformation. Each field corresponds to a component of the
/// `ChunkData` packet.
///
/// # Fields
///
/// - `x`, `z`: Chunk coordinates in chunk space (block coords >> 4)
/// - `min_y`: The minimum Y level (e.g., -64 for overworld)
/// - `height`: Total height in blocks (e.g., 384 for overworld)
/// - `sections`: Array of 16-block-tall sections (height / 16 sections)
/// - `heightmaps`: Pre-encoded NBT containing MOTION_BLOCKING and WORLD_SURFACE
/// - `block_entities`: Pre-encoded NBT array of block entities
#[derive(Debug, Clone)]
pub struct FerrumcChunk {
    /// Chunk X coordinate (block X >> 4).
    pub x: i32,
    /// Chunk Z coordinate (block Z >> 4).
    pub z: i32,
    /// Minimum Y level of this chunk (e.g., -64).
    pub min_y: i16,
    /// Total height of this chunk in blocks (e.g., 384).
    pub height: i16,
    /// The chunk sections, ordered from bottom to top.
    pub sections: Vec<FerrumcSection>,
    /// Raw NBT bytes for heightmaps (MOTION_BLOCKING, WORLD_SURFACE).
    pub heightmaps: Vec<u8>,
    /// Raw NBT bytes for block entities array.
    pub block_entities: Vec<u8>,
}

impl FerrumcChunk {
    /// Creates a new empty chunk at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - Chunk X coordinate
    /// * `z` - Chunk Z coordinate
    /// * `min_y` - Minimum Y level (typically -64 for overworld)
    /// * `height` - Total height in blocks (typically 384 for overworld)
    #[must_use]
    pub fn new(x: i32, z: i32, min_y: i16, height: i16) -> Self {
        let section_count = (height / 16) as usize;
        Self {
            x,
            z,
            min_y,
            height,
            sections: Vec::with_capacity(section_count),
            heightmaps: Vec::new(),
            block_entities: Vec::new(),
        }
    }

    /// Returns the number of sections in this chunk.
    #[inline]
    #[must_use]
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }

    /// Returns the expected number of sections based on height.
    #[inline]
    #[must_use]
    pub fn expected_section_count(&self) -> usize {
        (self.height / 16) as usize
    }

    /// Converts a block Y coordinate to a section index.
    ///
    /// Returns `None` if the Y coordinate is outside this chunk's bounds.
    #[inline]
    #[must_use]
    pub fn y_to_section_index(&self, y: i32) -> Option<usize> {
        let relative_y = y - self.min_y as i32;
        if relative_y < 0 || relative_y >= self.height as i32 {
            return None;
        }
        Some((relative_y / 16) as usize)
    }
}

// ============================================================================
// Section Structure
// ============================================================================

/// A 16x16x16 section of a chunk.
///
/// Each section contains:
/// - Block states (4096 blocks in paletted format)
/// - Biomes (64 biome entries in paletted format, 4x4x4 grid)
/// - Optional light data (sky light and block light)
///
/// # Block Count
///
/// The `block_count` field is the number of non-air blocks in this section.
/// This is sent to the client for rendering optimization - sections with
/// 0 blocks can be skipped entirely.
#[derive(Debug, Clone)]
pub struct FerrumcSection {
    /// Number of non-air blocks in this section (0-4096).
    pub block_count: u16,
    /// Block states for this section (4096 entries).
    pub block_states: PalettedContainer,
    /// Biome data for this section (64 entries, 4x4x4 grid).
    pub biomes: PalettedContainer,
    /// Sky light data (2048 bytes, 4 bits per block). None if not present.
    pub sky_light: Option<Vec<u8>>,
    /// Block light data (2048 bytes, 4 bits per block). None if not present.
    pub block_light: Option<Vec<u8>>,
}

impl FerrumcSection {
    /// Creates an empty section filled with air and plains biome.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            block_count: 0,
            block_states: PalettedContainer::Single(0), // Air
            biomes: PalettedContainer::Single(0),       // Plains (or default biome)
            sky_light: None,
            block_light: None,
        }
    }

    /// Creates a section filled with a single block type.
    #[must_use]
    pub fn filled(block_state_id: u32, biome_id: u32) -> Self {
        let block_count = if block_state_id == 0 { 0 } else { 4096 };
        Self {
            block_count,
            block_states: PalettedContainer::Single(block_state_id),
            biomes: PalettedContainer::Single(biome_id),
            sky_light: None,
            block_light: None,
        }
    }

    /// Returns `true` if this section contains only air.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.block_count == 0
    }
}

// ============================================================================
// Paletted Container
// ============================================================================

/// A paletted container for block states or biomes.
///
/// Minecraft uses three palette formats depending on the number of unique values:
///
/// | Format   | Bits/Entry | Description                                    |
/// |----------|------------|------------------------------------------------|
/// | Single   | 0          | All entries are the same value                 |
/// | Indirect | 4-8 (blocks) / 1-3 (biomes) | Palette indices packed into u64 array |
/// | Direct   | 15 (blocks) / 6 (biomes) | Global IDs packed directly          |
///
/// # Data Layout
///
/// The `data` field stores entries packed into `u64` values (Java Long Array format).
/// Entries are packed from LSB to MSB, with padding at the end of each `u64` if
/// entries don't divide evenly.
///
/// For example, with 5 bits per entry, each `u64` holds 12 entries (60 bits used,
/// 4 bits padding).
#[derive(Debug, Clone)]
pub enum PalettedContainer {
    /// All entries have the same value (0 bits per entry).
    ///
    /// The contained `u32` is the global ID (block state ID or biome ID).
    Single(u32),

    /// Indirect palette: entries are indices into a local palette.
    ///
    /// - `bits_per_entry`: Number of bits per index (4-8 for blocks, 1-3 for biomes)
    /// - `palette`: Maps indices to global IDs
    /// - `data`: Packed indices as `u64` array (Java Long Array)
    Indirect {
        /// Bits per palette index.
        bits_per_entry: u8,
        /// Local palette mapping index → global ID.
        palette: Vec<u32>,
        /// Packed palette indices.
        data: Vec<u64>,
    },

    /// Direct palette: entries are global IDs packed directly.
    ///
    /// - `bits_per_entry`: 15 for block states, 6 for biomes
    /// - `data`: Packed global IDs as `u64` array (Java Long Array)
    Direct {
        /// Bits per entry (15 for blocks, 6 for biomes).
        bits_per_entry: u8,
        /// Packed global IDs.
        data: Vec<u64>,
    },
}

impl PalettedContainer {
    // ========================================================================
    // Constructors
    // ========================================================================

    /// Creates a single-valued container.
    #[inline]
    #[must_use]
    pub const fn single(global_id: u32) -> Self {
        Self::Single(global_id)
    }

    /// Creates an indirect palette container.
    #[must_use]
    pub fn indirect(bits_per_entry: u8, palette: Vec<u32>, data: Vec<u64>) -> Self {
        Self::Indirect {
            bits_per_entry,
            palette,
            data,
        }
    }

    /// Creates a direct palette container.
    #[must_use]
    pub fn direct(bits_per_entry: u8, data: Vec<u64>) -> Self {
        Self::Direct {
            bits_per_entry,
            data,
        }
    }

    // ========================================================================
    // Queries
    // ========================================================================

    /// Returns the bits per entry for this container.
    #[inline]
    #[must_use]
    pub fn bits_per_entry(&self) -> u8 {
        match self {
            Self::Single(_) => 0,
            Self::Indirect { bits_per_entry, .. } => *bits_per_entry,
            Self::Direct { bits_per_entry, .. } => *bits_per_entry,
        }
    }

    /// Returns `true` if this is a single-valued container.
    #[inline]
    #[must_use]
    pub fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }

    /// Returns the palette size, or `None` for single/direct containers.
    #[inline]
    #[must_use]
    pub fn palette_size(&self) -> Option<usize> {
        match self {
            Self::Single(_) => None,
            Self::Indirect { palette, .. } => Some(palette.len()),
            Self::Direct { .. } => None,
        }
    }

    // ========================================================================
    // Entry Access
    // ========================================================================

    /// Gets the global ID at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The linear index (0-4095 for blocks, 0-63 for biomes)
    ///
    /// # Returns
    ///
    /// The global ID at that position, or `None` if the index is invalid.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<u32> {
        match self {
            Self::Single(id) => Some(*id),
            Self::Indirect {
                bits_per_entry,
                palette,
                data,
            } => {
                let palette_index = Self::read_packed(data, *bits_per_entry, index)?;
                palette.get(palette_index as usize).copied()
            }
            Self::Direct {
                bits_per_entry,
                data,
            } => Self::read_packed(data, *bits_per_entry, index),
        }
    }

    /// Reads a value from a packed `u64` array.
    ///
    /// # Arguments
    ///
    /// * `data` - The packed data array
    /// * `bits` - Bits per entry
    /// * `index` - The entry index to read
    fn read_packed(data: &[u64], bits: u8, index: usize) -> Option<u32> {
        if bits == 0 {
            return None;
        }

        let bits = bits as usize;
        let entries_per_long = 64 / bits;
        let long_index = index / entries_per_long;
        let bit_offset = (index % entries_per_long) * bits;

        let long = *data.get(long_index)?;
        let mask = (1u64 << bits) - 1;
        Some(((long >> bit_offset) & mask) as u32)
    }
}

impl Default for PalettedContainer {
    fn default() -> Self {
        Self::Single(0)
    }
}

// ============================================================================
// Constants
// ============================================================================

/// Number of blocks per section (16³).
pub const BLOCKS_PER_SECTION: usize = 16 * 16 * 16; // 4096

/// Number of biome entries per section (4³).
pub const BIOMES_PER_SECTION: usize = 4 * 4 * 4; // 64

/// Bits per entry for direct block state palette.
pub const DIRECT_BITS_BLOCKS: u8 = 15;

/// Bits per entry for direct biome palette.
pub const DIRECT_BITS_BIOMES: u8 = 6;

/// Minimum bits per entry for indirect block state palette.
pub const MIN_INDIRECT_BITS_BLOCKS: u8 = 4;

/// Maximum bits per entry for indirect block state palette.
pub const MAX_INDIRECT_BITS_BLOCKS: u8 = 8;

/// Minimum bits per entry for indirect biome palette.
pub const MIN_INDIRECT_BITS_BIOMES: u8 = 1;

/// Maximum bits per entry for indirect biome palette.
pub const MAX_INDIRECT_BITS_BIOMES: u8 = 3;

/// Size of light data array in bytes (4 bits per block, 4096 blocks).
pub const LIGHT_DATA_SIZE: usize = 2048;
