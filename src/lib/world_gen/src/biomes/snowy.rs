//! Cold biomes: snowy grass with a thin snow layer on top, over dirt. Backs both `snowy_plains`
//! (treeless) and `snowy_taiga` (scattered spruce); the selection layer
//! ([`crate::WorldGenerator::get_biome`]) builds the appropriate variant.

use crate::BiomeGenerator;
use crate::biomes::tree_placement::TreePlacer;
use crate::biomes::trees::{Tree, TreeKind};
use crate::errors::WorldGenError;
use crate::terrain_noise::NoiseGenerator;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

/// Minimum surface height for a tree (above the waterline).
const TREE_MIN_SURFACE_Y: i16 = 64;

pub(crate) struct SnowyBiome {
    dirt_depth_noise: NoiseGenerator,
    /// Registry biome ID — `46` (snowy_plains) or `48` (snowy_taiga).
    id: u8,
    /// Spruce-tree placer for the taiga variant; `None` for treeless snowy plains.
    trees: Option<TreePlacer>,
}

impl SnowyBiome {
    /// Builds the treeless snowy plains variant (ID 46).
    pub(crate) fn plains(seed: u64) -> Self {
        SnowyBiome {
            dirt_depth_noise: NoiseGenerator::new(seed, 0.1, 4, None),
            id: 46,
            trees: None,
        }
    }

    /// Builds the snowy taiga variant (ID 48) with scattered spruce trees.
    pub(crate) fn taiga(seed: u64) -> Self {
        SnowyBiome {
            dirt_depth_noise: NoiseGenerator::new(seed, 0.1, 4, None),
            id: 48,
            trees: Some(TreePlacer::new(
                seed.wrapping_add(29),
                0.018,
                3,
                0.45,
                TREE_MIN_SURFACE_Y,
            )),
        }
    }
}

impl BiomeGenerator for SnowyBiome {
    fn biome_id(&self) -> u8 {
        self.id
    }

    fn _biome_name(&self) -> String {
        if self.id == 48 {
            "snowy_taiga".to_string()
        } else {
            "snowy_plains".to_string()
        }
    }

    fn decorate(
        &self,
        chunk: &mut Chunk,
        x: u8,
        z: u8,
        surface_y: i16,
    ) -> Result<(), WorldGenError> {
        chunk.set_block(
            ChunkBlockPos::new(x, surface_y, z),
            block!("grass_block", { snowy: true }),
        );

        let dirt_depth = (self.dirt_depth_noise.get(f32::from(x), f32::from(z)) * 5.0) + 3.0;
        for i in 1..=dirt_depth as i16 {
            chunk.set_block(ChunkBlockPos::new(x, surface_y - i, z), block!("dirt"));
        }

        // A single layer of snow on top of the grass.
        chunk.set_block(
            ChunkBlockPos::new(x, surface_y + 1, z),
            block!("snow", { layers: 1 }),
        );

        Ok(())
    }

    fn tree_at(&self, global_x: i32, global_z: i32, surface_y: i16) -> Option<Tree> {
        let placer = self.trees.as_ref()?;
        if !placer.should_place_tree(global_x, global_z, surface_y) {
            return None;
        }
        Some(Tree {
            kind: TreeKind::Spruce,
            surface_y,
            trunk_height: placer.trunk_height(global_x, global_z, 6, 3),
        })
    }

    fn new(seed: u64) -> Self {
        SnowyBiome::plains(seed)
    }
}
