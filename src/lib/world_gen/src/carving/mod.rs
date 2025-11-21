use crate::WorldGenerator;

pub mod erosion;
pub mod initial_height;

impl WorldGenerator {
    pub fn carve_chunk(
        &self,
        chunk: &mut ferrumc_world::chunk_format::Chunk,
    ) -> Result<(), crate::errors::WorldGenError> {
        self.apply_initial_height(chunk)?;
        self.apply_erosion(chunk)?;
        Ok(())
    }
}
