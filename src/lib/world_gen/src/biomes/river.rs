//! River biome: a gravel-and-sand bed over dirt, laid on the floor of a carved river channel. The
//! water itself is supplied by the global below-sea-level flood (see
//! [`crate::WorldGenerator::generate_chunk`]); this decorator only lays the bed.
//!
//! A single decorator backs both `river` and `frozen_river`: the bed is identical, and only the
//! recorded registry biome ID differs (chosen from temperature by the selection layer). Frozen
//! rivers are not yet capped with ice — the surface water is left as-is.

use crate::BiomeGenerator;
use crate::errors::WorldGenError;
use crate::terrain_noise::NoiseGenerator;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

pub(crate) struct RiverBiome {
    /// Varies the bed thickness slightly across the channel so the floor is not a flat slab.
    bed_depth_noise: NoiseGenerator,
}

impl BiomeGenerator for RiverBiome {
    fn biome_id(&self) -> u8 {
        41 // minecraft:river — placeholder; frozen_river (24) is chosen by the selection layer.
    }

    fn _biome_name(&self) -> String {
        "river".to_string()
    }

    fn decorate(
        &self,
        chunk: &mut Chunk,
        x: u8,
        z: u8,
        surface_y: i16,
    ) -> Result<(), WorldGenError> {
        // A gravel cap (the classic river floor), then a couple of dirt layers over the base stone.
        let bed = ((self.bed_depth_noise.get(f32::from(x), f32::from(z)) * 2.0) + 2.0) as i16;
        for i in 0..=bed {
            chunk.set_block(ChunkBlockPos::new(x, surface_y - i, z), block!("gravel"));
        }
        for i in 1..=2 {
            chunk.set_block(
                ChunkBlockPos::new(x, surface_y - bed - i, z),
                block!("dirt"),
            );
        }
        Ok(())
    }

    fn new(seed: u64) -> Self {
        RiverBiome {
            bed_depth_noise: NoiseGenerator::new(seed.wrapping_add(77), 0.1, 4, None),
        }
    }
}
