use crate::errors::WorldError;
use crate::vanilla_chunk_format::BlockData;
use crate::World;

impl World {
    pub async fn get_block(
        &self,
        x: i32,
        y: i32,
        z: i32,
        dimension: &str,
    ) -> Result<BlockData, WorldError> {
        let chunk_x = x / 16;
        let chunk_z = z / 16;
        let chunk = self.load_chunk(chunk_x, chunk_z, dimension).await?;
        let section = chunk
            .sections
            .iter()
            .find(|section| section.y == (y / 16) as i8)
            .ok_or(WorldError::SectionOutOfBounds(y / 16))?;
        let bits_per_block = section.block_states.bits_per_block as usize;
        let data = &section.block_states.data;
        // for some reason the y is off by one block
        let index = ((y) % 16) * 256 + (z % 16) * 16 + (x % 16);
        let i64_index = (index * bits_per_block as i32) as usize / 64;
        let packed_u64 = data.get(i64_index).ok_or(WorldError::ChunkNotFound)?;
        let offset = (index as usize * bits_per_block) % 64;
        let id = ferrumc_general_purpose::data_packing::u32::read_nbit_u32(
            packed_u64,
            bits_per_block as u8,
            offset as u32,
        )?;
        let palette_id = section
            .block_states
            .palette
            .get(id as usize)
            .ok_or(WorldError::ChunkNotFound)?;
        Ok(crate::chunk_format::ID2BLOCK
            .get(&palette_id.val)
            .unwrap_or(&BlockData::default())
            .clone())
    }
}
