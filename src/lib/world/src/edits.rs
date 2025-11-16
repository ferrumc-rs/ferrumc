use crate::block_state_id::BlockStateId;
use crate::chunk_format::{BlockStates, Chunk, PaletteType, Paletted, Section};
use crate::errors::WorldError;
use crate::World;
use bevy_math::IVec3;
use ferrumc_macros::block;
use std::sync::Arc;
use tracing::debug;
impl World {
    /// Retrieves the [`BlockStateId`] at the specified coordinates in the given dimension.
    pub fn get_block_and_fetch(
        &self,
        pos: IVec3,
        dimension: &str,
    ) -> Result<BlockStateId, WorldError> {
        let chunk_x = pos.x >> 4;
        let chunk_z = pos.z >> 4;
        let chunk = self.load_chunk(chunk_x, chunk_z, dimension)?;
        Ok(*chunk.get_block(pos)?)
    }

    /// Sets the [`BlockStateId`] at the specified coordinates in the given dimension.
    pub fn set_block_and_fetch(
        &self,
        pos: IVec3,
        dimension: &str,
        block_state_id: BlockStateId,
    ) -> Result<(), WorldError> {
        // Get chunk
        let chunk_x = pos.x >> 4;
        let chunk_z = pos.z >> 4;
        let mut chunk = self.load_chunk_owned(chunk_x, chunk_z, dimension)?;

        debug!("Chunk: {}, {}", chunk_x, chunk_z);

        chunk.set_block(pos, block_state_id)?;

        // Save chunk
        self.save_chunk(Arc::new(chunk))?;
        Ok(())
    }
}

impl Chunk {
    /// Sets the block at the specified coordinates to the sepcifiend block.
    pub fn set_block(
        &mut self,
        pos: IVec3,
        block_state_id: BlockStateId,
    ) -> Result<(), WorldError> {
        self.find_section_mut(pos.y)?.set_block(pos, block_state_id)
    }
    pub fn find_section(&self, y: i32) -> Result<&Section, WorldError> {
        self.sections
            .iter()
            .find(|section| section.y == (y >> 4) as i8)
            .ok_or(WorldError::SectionOutOfBounds(y >> 4))
    }

    pub fn find_section_mut(&mut self, y: i32) -> Result<&mut Section, WorldError> {
        self.sections
            .iter_mut()
            .find(|section| section.y == (y >> 4) as i8)
            .ok_or(WorldError::SectionOutOfBounds(y >> 4))
    }

    /// Gets the block at the specified coordinates.
    pub fn get_block(&self, pos: IVec3) -> Result<&BlockStateId, WorldError> {
        Ok(self.find_section(pos.y)?.get_block(pos))
    }

    /// Fills the [`Section`] at the specified index with the specified [`BlockStateId`].
    /// If the section is out of bounds, an error is returned.
    pub fn set_section(&mut self, y: i32, block_state_id: BlockStateId) -> Result<(), WorldError> {
        self.find_section_mut(y)?.fill(block_state_id);
        Ok(())
    }

    /// Fills the chunk with the specified block.
    pub fn fill(&mut self, block_state_id: BlockStateId) {
        for section in &mut self.sections {
            section.fill(block_state_id);
        }
    }
}

impl Section {
    pub fn index(pos: IVec3) -> usize {
        (pos & 15).dot((1, 256, 16).into()) as usize
    }

    /// Set block by index from `Self::index`
    pub fn set_block_by_index(
        &mut self,
        index: usize,
        block_state_id: BlockStateId,
    ) -> Result<(), WorldError> {
        self.block_states.set_block_by_index(index, block_state_id);
        Ok(())
    }

    /// Set block by it's position in the world.
    pub fn set_block(
        &mut self,
        pos: IVec3,
        block_state_id: BlockStateId,
    ) -> Result<(), WorldError> {
        self.set_block_by_index(Self::index(pos), block_state_id)
    }

    /// Get block by index from `Self::index`
    pub fn get_block_by_index(&self, index: usize) -> &BlockStateId {
        self.block_states.get_block_by_index(index)
    }

    /// Get block by it's position in the world.
    pub fn get_block(&self, pos: IVec3) -> &BlockStateId {
        self.block_states.get_block_by_index(Self::index(pos))
    }

    /// Fills the section with the specified block.
    pub fn fill(&mut self, block_state_id: BlockStateId) {
        self.block_states = BlockStates::from_single(block_state_id);
    }

    /// UNIMPLEMENTED
    pub fn optimise(&mut self) {
        self.block_states = self.block_states.iter().collect();
    }
}

impl BlockStates {
    /// Get block by index from `Section::index`
    pub fn get_block_by_index(&self, index: usize) -> &BlockStateId {
        assert!((0..4096).contains(&index));
        match &self.block_data {
            PaletteType::Empty => &block!("air"),
            PaletteType::Paleted(paletted) => match paletted.as_ref() {
                Paletted::U4 { palette, data, .. } => {
                    let palette_index = if index % 2 == 1 {
                        data[index / 2] >> 4
                    } else {
                        data[index / 2] & 0xf
                    } as usize;
                    assert!((0..16).contains(&palette_index));
                    &palette[palette_index]
                }
                Paletted::U8 { palette, data, .. } => {
                    let palette_index = data[index] as usize;
                    assert!((0..256).contains(&palette_index));
                    &palette[palette_index]
                }
                Paletted::Direct { data } => &data[index],
            },
        }
    }
    // TODO: find what is the bug causing client disconect
    fn promote_to_next_size(&mut self) {
        debug!("size promotion");
        match &self.block_data {
            PaletteType::Empty => self.block_data = PaletteType::empty_u4(),
            PaletteType::Paleted(paletted) => match paletted.as_ref() {
                Paletted::U4 {
                    palette,
                    data,
                    last,
                } => {
                    let mut new_palette = [BlockStateId(0); 256];
                    for (index, &block) in palette.iter().enumerate() {
                        new_palette[index] = block;
                    }
                    let mut new_data = [0u8; 4096];
                    for (index, data) in data.iter().enumerate() {
                        new_data[index * 2] = data % 16;
                        new_data[index * 2 + 1] = data / 16;
                    }
                    self.block_data = PaletteType::Paleted(Box::new(Paletted::U8 {
                        palette: new_palette,
                        data: Box::new(new_data),
                        last: *last,
                    }));
                }
                Paletted::U8 { palette, data, .. } => {
                    let mut blocks = [BlockStateId(0); 4096];
                    for (index, &data) in data.iter().enumerate() {
                        blocks[index] = palette[data as usize];
                    }
                    self.block_data = PaletteType::Paleted(Box::new(Paletted::Direct {
                        data: Box::new(blocks),
                    }));
                }
                Paletted::Direct { .. } => {}
            },
        }
    }

    fn is_palette_full(&self) -> bool {
        match &self.block_data {
            PaletteType::Empty => true,
            PaletteType::Paleted(palette) => match &**palette {
                Paletted::U4 { last, .. } => last == &15,
                Paletted::U8 { last, .. } => last == &255,
                Paletted::Direct { .. } => false,
            },
        }
    }
    /// Set block by index from `Section::index`. This method must uphold invariants of the section.
    pub fn set_block_by_index(&mut self, index: usize, block_state_id: BlockStateId) {
        assert!((0..4096).contains(&index));
        let old_block_id = *self.get_block_by_index(index);
        if old_block_id == block_state_id {
            return;
        }
        // TODO: could we remove old_block_id form the block_counts here?
        let old_block_count = *self
            .block_counts
            .entry(old_block_id)
            .and_modify(|x| *x -= 1)
            .or_insert(0);

        let block_count = *self
            .block_counts
            .entry(block_state_id)
            .and_modify(|x| *x += 1)
            .or_insert(1)
            - 1;

        if !block_state_id.is_non_air() && old_block_id.is_non_air() {
            self.non_air_blocks -= 1;
        }
        if block_state_id.is_non_air() && !old_block_id.is_non_air() {
            self.non_air_blocks += 1;
        }

        // if palette is full promote to the next size
        if self.is_palette_full() {
            self.promote_to_next_size();
        }

        // now the size is correct, set the block
        match &mut self.block_data {
            PaletteType::Empty => unreachable!(),
            PaletteType::Paleted(paletted) => match paletted.as_mut() {
                Paletted::U4 {
                    palette,
                    last,
                    data,
                } => {
                    let set_palette = |data: &mut [u8; 2048], palette_index: u8| {
                        if index % 2 == 0 {
                            data[index / 2] = (data[index / 2] & 0xf0) | (palette_index % 16);
                        } else {
                            data[index / 2] =
                                (data[index / 2] & 0x0f) | ((palette_index % 16) * 16);
                        }
                    };
                    let get_palette = |data: &mut [u8; 2048], index: usize| {
                        if index % 2 == 0 {
                            data[index / 2] % 16
                        } else {
                            data[index / 2] / 16
                        }
                    };
                    // evict single block, this won't reduce last size if `block_state_id` is air
                    // to not change all the values
                    if old_block_count == 0 {
                        let palette_index = get_palette(data, index);
                        palette[palette_index as usize] = block_state_id;
                        return;
                    }
                    // find the block in the palette
                    if block_count != 0 {
                        let (palette_index, _) = palette
                            .iter()
                            .take(*last as usize)
                            .enumerate()
                            .find(|(_, &block)| block == block_state_id)
                            .expect("block_state_id count is not zero");
                        set_palette(data, palette_index as u8);
                        return;
                    }
                    // take another spot in the palette
                    palette[*last as usize] = block_state_id;
                    set_palette(data, *last);
                    *last += 1;
                }
                Paletted::U8 {
                    palette,
                    last,
                    data,
                } => {
                    // evict single block, this won't reduce last size if `block_state_id` is air
                    // to not change all the values
                    if old_block_count == 0 {
                        let palette_index = data[index];
                        palette[palette_index as usize] = block_state_id;
                        return;
                    }
                    // find the block in the palette
                    if block_count != 0 {
                        let (palette_index, _) = palette
                            .iter()
                            .take(*last as usize)
                            .enumerate()
                            .find(|(_, &block)| block == block_state_id)
                            .expect("block_state_id count is not zero");
                        data[index] = palette_index as u8;
                        return;
                    }
                    // take another spot in the palette
                    palette[*last as usize] = block_state_id;
                    data[index] = *last;
                    *last += 1;
                }
                Paletted::Direct { data } => data[index] = block_state_id,
            },
        }
    }
}
