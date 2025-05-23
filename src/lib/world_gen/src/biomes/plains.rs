use crate::errors::WorldGenError;
use crate::BiomeGenerator;
use ferrumc_world::chunk_format::Chunk;

pub(crate) struct PlainsBiome;

impl BiomeGenerator for PlainsBiome {
    fn _biome_id(&self) -> u8 {
        0
    }

    fn _biome_name(&self) -> String {
        "plains".to_string()
    }

    fn decorate(&self, chunk: &mut Chunk) -> Result<(), WorldGenError> {
        Ok(())
    }
}


