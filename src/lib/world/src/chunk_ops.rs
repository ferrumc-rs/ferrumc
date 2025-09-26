use crate::block_id::BlockId;
use crate::chunk_format::{Chunk, Heightmaps, Section};
use bevy_math::{IVec2, IVec3};

impl Chunk {
    /// Creates a new empty chunk at the given chunk coordinates and dimension.
    ///
    /// The chunk will contain sections from Y=-4 to Y=19 (16 sections).
    ///
    /// # Arguments
    ///
    /// * `pos` - The chunk coordinates (x, z) as IVec2.
    /// * `dimension` - The dimension of the chunk (e.g., "minecraft:overworld").
    /// # Examples
    /// ```rust
    /// use ferrumc_world::chunk_format::Chunk;
    /// use bevy_math::IVec2;
    ///
    /// let chunk = Chunk::new(IVec2::new(0, 0), "minecraft:overworld".to_string());
    pub fn new(pos: IVec2, dimension: String) -> Self {
        let mut sections: Vec<Section> = (-4..20).map(|y| Section::new(y as i8)).collect();
        for section in &mut sections {
            section.optimise();
        }
        Chunk {
            x: pos.x,
            z: pos.y,
            dimension,
            sections,
            heightmaps: Heightmaps::new(),
        }
    }
    /// Gets a block in the chunk at the given global position.
    ///
    /// The position is in global coordinates. The y coordinate is used to determine the section.
    ///
    /// # Arguments
    ///
    /// * `pos` - The global position of the block to get.
    /// # Returns
    /// * Returns the BlockId at the given position. If no block is found, returns BlockId::default().
    /// # Examples
    /// ```rust
    /// use bevy_math::{IVec3, IVec2};
    /// use ferrumc_world::chunk_format::Chunk;
    /// use ferrumc_world::block_id::BlockId;
    /// use ferrumc_world::errors::WorldError;
    ///
    /// fn main() -> Result<(), WorldError> {
    ///     let mut chunk = Chunk::new(IVec2::new(0, 0), "minecraft:overworld".to_string());
    ///     let pos = IVec3::new(1, 18, 3); // y=18 means section 1 (16-31)
    ///     let block = BlockId(1);
    ///     chunk.set_block(pos, block)?;
    ///     let retrieved_block = chunk.get_block(pos);
    ///     assert_eq!(block, retrieved_block);
    ///     Ok(())
    /// }
    pub fn get_block(&self, pos: IVec3) -> BlockId {
        let section_index = (pos.y >> 4) as usize;
        if section_index >= self.sections.len() {
            return BlockId::default();
        }
        let section = &self.sections[section_index];
        let local_pos = IVec3::new(pos.x & 0xF, pos.y & 0xF, pos.z & 0xF);
        section.get_block(local_pos)
    }
    /// Sets a block in the chunk at the given global position.
    ///
    /// The position is in global coordinates. The y coordinate is used to determine the section.
    ///
    /// # Arguments
    ///
    /// * `pos` - The global position of the block to set.
    /// * `block` - The BlockId to set at the given position.
    /// # Errors
    /// * Returns `WorldError::SectionOutOfBounds` if the section index is out of bounds.
    /// # Examples
    /// ```rust
    /// use bevy_math::IVec3;
    /// use ferrumc_world::chunk_format::Chunk;
    /// use ferrumc_world::block_id::BlockId;
    /// use ferrumc_world::errors::WorldError;    
    /// use bevy_math::IVec2;
    ///
    /// fn main() -> Result<(), WorldError> {
    ///     let mut chunk = Chunk::new(IVec2::new(0, 0), "minecraft:overworld".to_string());
    ///     let pos = IVec3::new(1, 18, 3); // y=18 means section 1 (16-31)
    ///     let block = BlockId(1);
    ///     chunk.set_block(pos, block)?;
    ///     let retrieved_block = chunk.get_block(pos);
    ///     assert_eq!(block, retrieved_block);
    ///     Ok(())
    /// }
    pub fn set_block(
        &mut self,
        pos: IVec3,
        block: BlockId,
    ) -> Result<(), crate::errors::WorldError> {
        let section_index = (pos.y >> 4) as usize;
        if section_index >= self.sections.len() {
            return Err(crate::errors::WorldError::SectionOutOfBounds(
                section_index as i32,
            ));
        }
        let section = &mut self.sections[section_index];
        let local_pos = IVec3::new(pos.x & 0xF, pos.y & 0xF, pos.z & 0xF);
        section.set_block(local_pos, block)
    }
    /// Fills the entire chunk with the given block.
    ///
    /// # Arguments
    ///
    /// * `block` - The BlockId to fill the chunk with.
    /// # Examples
    /// ```rust
    /// use ferrumc_world::chunk_format::Chunk;
    /// use ferrumc_world::block_id::BlockId;
    /// use ferrumc_world::errors::WorldError;
    /// use bevy_math::{IVec3, IVec2};
    ///
    /// fn main() -> Result<(), WorldError> {
    ///     let mut chunk = Chunk::new(IVec2::new(0, 0), "minecraft:overworld".to_string());
    ///     let block = BlockId(1);
    ///     chunk.fill(block);
    ///     for section in &chunk.sections {
    ///         for y in 0..16 {
    ///             for z in 0..16 {
    ///                 for x in 0..16 {
    ///                     let pos = IVec3::new(x, y, z);
    ///                     let retrieved_block = section.get_block(pos);
    ///                     assert_eq!(block, retrieved_block);
    ///                 }
    ///             }
    ///         }
    ///     }
    ///     Ok(())
    /// }
    pub fn fill(&mut self, block: BlockId) {
        self.sections = (-4..20).map(|y| Section::new(y as i8)).collect();
        for section in &mut self.sections {
            section.fill(block);
        }
    }

    /// Fills a specific section of the chunk with the given block.
    ///
    /// The section is specified by its Y coordinate, which ranges from -4 to 19.
    ///
    /// # Arguments
    ///
    /// * `section_y` - The Y coordinate of the section to fill (-4 to 19).
    /// * `block` - The BlockId to fill the section with.
    /// # Errors
    /// * Returns `WorldError::SectionOutOfBounds` if the section index is out of bounds.
    /// # Examples
    /// ```rust
    /// use ferrumc_world::chunk_format::Chunk;
    /// use ferrumc_world::block_id::BlockId;
    /// use ferrumc_world::errors::WorldError;
    /// use bevy_math::{IVec3, IVec2};
    ///
    /// fn main() -> Result<(), WorldError> {
    ///     let mut chunk = Chunk::new(IVec2::new(0, 0), "minecraft:overworld".to_string());
    ///     let block = BlockId(1);
    ///     chunk.fill_section(0, block)?; // Fill section at Y=0 (0-15)
    ///     let section = &chunk.sections[4]; // Section index 4 corresponds to Y=0
    ///     for y in 0..16 {
    ///         for z in 0..16 {
    ///             for x in 0..16 {
    ///                 let pos = IVec3::new(x, y, z);
    ///                 let retrieved_block = section.get_block(pos);
    ///                 assert_eq!(block, retrieved_block);
    ///             }
    ///         }
    ///     }
    ///     Ok(())
    /// }
    pub fn fill_section(
        &mut self,
        section_y: i8,
        block: BlockId,
    ) -> Result<(), crate::errors::WorldError> {
        let section_index = (section_y + 4) as usize;
        if section_index >= self.sections.len() {
            return Err(crate::errors::WorldError::SectionOutOfBounds(
                section_index as i32,
            ));
        }
        let section = &mut self.sections[section_index];
        section.fill(block);
        Ok(())
    }

    /// Optimises the chunk's sections by reducing the bits per entry where possible.
    ///
    /// This function iterates through each section in the chunk and calls the `optimise` method
    /// on the section's block states to reduce memory usage.
    /// # Examples
    /// ```rust
    /// use ferrumc_world::chunk_format::Chunk;
    /// use ferrumc_world::block_id::BlockId;
    /// use ferrumc_world::errors::WorldError;
    /// use bevy_math::IVec2;
    ///
    /// fn main() -> Result<(), WorldError> {
    ///     let mut chunk = Chunk::new(IVec2::new(0, 0), "minecraft:overworld".to_string());
    ///     let block = BlockId(1);
    ///     chunk.fill(block);
    ///     chunk.optimise();
    ///     Ok(())
    /// }
    pub fn optimise(&mut self) {
        for section in &mut self.sections {
            section.block_states.optimise();
        }
    }
}
