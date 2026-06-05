//! Beach biome: a sand strip over sandstone along coastlines. A snowy variant (with the same floor)
//! is used where the climate is cold. Backs both `beach` and `snowy_beach`; the selection layer
//! ([`crate::WorldGenerator::get_biome`]) builds the variant with the appropriate biome ID.

use crate::BiomeGenerator;
use crate::errors::WorldGenError;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

pub(crate) struct BeachBiome {
    /// Registry biome ID — `3` (beach) or `45` (snowy_beach).
    id: u8,
}

impl BeachBiome {
    /// Builds a beach variant with the given registry biome ID.
    pub(crate) fn with_id(id: u8) -> Self {
        BeachBiome { id }
    }
}

impl BiomeGenerator for BeachBiome {
    fn biome_id(&self) -> u8 {
        self.id
    }

    fn _biome_name(&self) -> String {
        if self.id == 45 {
            "snowy_beach".to_string()
        } else {
            "beach".to_string()
        }
    }

    fn decorate(
        &self,
        chunk: &mut Chunk,
        x: u8,
        z: u8,
        surface_y: i16,
    ) -> Result<(), WorldGenError> {
        // A few blocks of sand over sandstone.
        for i in 0..=3 {
            chunk.set_block(ChunkBlockPos::new(x, surface_y - i, z), block!("sand"));
        }
        for i in 4..=8 {
            chunk.set_block(ChunkBlockPos::new(x, surface_y - i, z), block!("sandstone"));
        }
        Ok(())
    }

    fn new(_: u64) -> Self {
        BeachBiome::with_id(3)
    }
}
