use crate::block_id::BlockId;
use crate::chunk_format::{BiomeStates, Section};
use crate::errors::WorldError;
use crate::to_index;
use bevy_math::IVec3;
use ferrumc_general_purpose::palette::Palette;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::trace;

impl Section {
    /// Creates a new empty section with the given Y level.
    ///
    /// # Arguments
    ///
    /// * `level` - The Y level of the section.
    /// # Examples
    /// ```rust
    /// use ferrumc_world::chunk_format::Section;
    ///
    /// let section = Section::new(0);
    pub fn new(level: i8) -> Self {
        Self {
            y: level,
            block_states: Palette::new(4096, BlockId::default(), 15),
            // Add other fields as necessary
            biome_states: BiomeStates {
                bits_per_biome: 0,
                data: vec![],
                palette: vec![VarInt::from(0)],
            },
            block_light: vec![255; 2048],
            sky_light: vec![255; 2048],
        }
    }
    /// Sets a block in the section at the given position.
    ///
    /// The position is relativized to the section (0-15) but you
    /// should still convert to relative coordinates before calling this function for readability.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the block to set, relative to the section.
    /// * `block` - The BlockId to set at the given position.
    /// # Errors
    /// * Returns `WorldError::OutOfBounds` if the position is out of bounds.
    /// # Examples
    /// ```rust
    /// use bevy_math::IVec3;
    /// use ferrumc_world::chunk_format::Section;
    /// use ferrumc_world::block_id::BlockId;
    /// use ferrumc_world::errors::WorldError;
    ///
    /// fn main() -> Result<(), WorldError> {
    ///     let mut section = Section::new(0);
    ///     let pos = IVec3::new(1, 2, 3);
    ///     let block = BlockId(0);
    ///     section.set_block(pos, block)?;
    ///     Ok(())
    /// }
    pub fn set_block(&mut self, pos: IVec3, block: BlockId) -> Result<(), WorldError> {
        let index = to_index(pos);
        self.block_states.set(index, block);
        Ok(())
    }

    /// Gets a block in the section at the given position.
    ///
    /// The position is relativized to the section (0-15) but you
    /// should still convert to relative coordinates before calling this function for readability.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the block to get, relative to the section.
    /// # Returns
    /// * Returns the BlockId at the given position. If no block is found, returns BlockId::default().
    /// # Examples
    /// ```rust
    /// use bevy_math::IVec3;
    /// use ferrumc_world::chunk_format::Section;
    /// use ferrumc_world::block_id::BlockId;
    /// use ferrumc_world::errors::WorldError;
    ///
    /// fn main() -> Result<(), WorldError> {
    ///     let mut section = Section::new(0);
    ///     let pos = IVec3::new(1, 2, 3);
    ///     let block = BlockId(0);
    ///     section.set_block(pos, block)?;
    ///     let retrieved_block = section.get_block(pos);
    ///     assert_eq!(block, retrieved_block);
    ///     Ok(())
    /// }
    pub fn get_block(&self, pos: IVec3) -> BlockId {
        let index = to_index(pos);
        match self.block_states.get(index) {
            Some(block) => *block,
            None => {
                trace!(
                    "Tried to get block but no block found at position: {:?}",
                    pos
                );
                BlockId::default()
            }
        }
    }

    /// Fills the entire section with the given block.
    ///
    /// # Arguments
    ///
    /// * `block` - The BlockId to fill the section with.
    /// # Examples
    /// ```rust
    /// use ferrumc_world::chunk_format::Section;
    /// use ferrumc_world::block_id::BlockId;
    /// use ferrumc_world::errors::WorldError;
    ///
    /// fn main() -> Result<(), WorldError> {
    ///     let mut section = Section::new(0);
    ///     let block = BlockId(0);
    ///     section.fill(block);
    ///     Ok(())
    /// }
    pub fn fill(&mut self, block: BlockId) {
        self.block_states = Palette::new(4096, block, 15);
    }

    /// Optimises the section's block state palette.
    ///
    /// This should be called after a series of set operations to ensure the palette is as compact as possible.
    /// # Examples
    /// ```rust
    /// use ferrumc_world::chunk_format::Section;
    /// use ferrumc_world::block_id::BlockId;
    /// use ferrumc_world::errors::WorldError;
    /// use bevy_math::IVec3;
    ///
    /// fn main() -> Result<(), WorldError> {
    ///     let mut section = Section::new(0);
    ///     let block = BlockId(0);
    ///     section.fill(block);
    ///     section.set_block(IVec3::new(1, 2, 3), BlockId(0))?;
    ///     section.optimise();
    ///     Ok(())
    /// }
    pub fn optimise(&mut self) {
        self.block_states.optimise()
    }

    /// Gets the count of a specific block in the section.
    ///
    /// # Arguments
    ///
    /// * `block` - The BlockId to count in the section.
    /// # Returns
    /// * Returns the count of the specified block in the section.
    /// # Examples
    /// ```rust
    /// use ferrumc_world::chunk_format::Section;
    /// use ferrumc_world::block_id::BlockId;
    /// use ferrumc_world::errors::WorldError;
    ///
    /// fn main() -> Result<(), WorldError> {
    ///     let mut section = Section::new(0);
    ///     let block = BlockId(0);
    ///     section.fill(block);
    ///     let count = section.get_count(&block);
    ///     assert_eq!(count, 4096);
    ///     Ok(())
    /// }
    pub fn get_count(&self, block: &BlockId) -> usize {
        self.block_states.get_count(block)
    }
}

#[cfg(test)]
mod tests {
    use super::Section;
    use crate::block_id::BlockId;
    use crate::vanilla_chunk_format::BlockData;
    use bevy_math::IVec3;

    #[test]
    fn test_set_and_get_block() {
        let mut section = Section::new(0);
        let pos = IVec3::new(1, 2, 3);
        let block = BlockId::from(BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        });
        section.set_block(pos, block).unwrap();
        assert_eq!(section.get_block(pos), block);
    }

    #[test]
    fn test_get_block_default() {
        let section = Section::new(0);
        let pos = IVec3::new(0, 0, 0);
        assert_eq!(section.get_block(pos), BlockId::default());
    }

    #[test]
    fn test_fill() {
        let mut section = Section::new(0);
        let block = BlockId::from(BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        });
        section.fill(block);
        let pos = IVec3::new(5, 5, 5);
        assert_eq!(section.get_block(pos), block);
    }

    #[test]
    fn test_optimise() {
        let mut section = Section::new(0);
        let block = BlockId::from(BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        });
        section.fill(block);
        section
            .set_block(
                IVec3::new(1, 1, 1),
                BlockId::from(BlockData {
                    name: "minecraft:dirt".to_string(),
                    properties: None,
                }),
            )
            .unwrap();
        section.optimise();
        // No assertion, just ensure no panic
    }

    #[test]
    fn test_get_count() {
        let mut section = Section::new(0);
        let stone = BlockId::from(BlockData {
            name: "minecraft:stone".to_string(),
            properties: None,
        });
        let dirt = BlockId::from(BlockData {
            name: "minecraft:dirt".to_string(),
            properties: None,
        });
        section.fill(stone);
        section.set_block(IVec3::new(1, 1, 1), dirt).unwrap();
        assert_eq!(section.get_count(&stone), 4095);
        assert_eq!(section.get_count(&dirt), 1);
    }
}
