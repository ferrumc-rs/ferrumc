use crate::World;
use bevy_math::{IVec2, IVec3};
use std::sync::Arc;

impl World {
    pub fn get_block_and_fetch(
        &self,
        chunk: IVec2,
        block: IVec3,
        dimension: &str,
    ) -> Option<crate::block_id::BlockId> {
        if let Ok(chunk) = self.load_chunk(chunk, dimension) {
            Some(chunk.get_block(block))
        } else {
            None
        }
    }

    pub fn set_block_and_save(
        &mut self,
        chunk: IVec2,
        block: IVec3,
        dimension: &str,
        block_id: crate::block_id::BlockId,
    ) -> Result<(), crate::errors::WorldError> {
        let mut chunk = self.load_chunk_owned(chunk, dimension)?;
        chunk.set_block(block, block_id)?;
        self.save_chunk(Arc::new(chunk))?;
        Ok(())
    }
}
