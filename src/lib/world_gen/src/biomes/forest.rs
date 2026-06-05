//! Forest biome: grass over dirt, densely covered in a mix of oak and birch trees.

use crate::BiomeGenerator;
use crate::biomes::tree_placement::TreePlacer;
use crate::biomes::trees::{Tree, TreeKind};
use crate::errors::WorldGenError;
use crate::terrain_noise::NoiseGenerator;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

/// Minimum surface height for a tree (kept above the waterline as in the other wooded biomes).
const TREE_MIN_SURFACE_Y: i16 = 64;

pub(crate) struct ForestBiome {
    seed: u64,
    dirt_depth_noise: NoiseGenerator,
    trees: TreePlacer,
}

impl ForestBiome {
    /// Picks oak vs birch for the tree at a column, deterministically from the seed and position so
    /// the choice is stable across the chunks that resolve this tree. Roughly a third are birch.
    fn tree_kind(&self, gx: i32, gz: i32) -> TreeKind {
        let mut h = self.seed ^ 0x9e37_79b9_7f4a_7c15;
        h ^= (gx as u64).wrapping_mul(0xff51_afd7_ed55_8ccd);
        h ^= (gz as u64).wrapping_mul(0xc4ce_b9fe_1a85_ec53);
        h ^= h >> 29;
        if h.is_multiple_of(3) {
            TreeKind::Birch
        } else {
            TreeKind::Oak
        }
    }
}

impl BiomeGenerator for ForestBiome {
    fn biome_id(&self) -> u8 {
        21 // minecraft:forest
    }

    fn _biome_name(&self) -> String {
        "forest".to_string()
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
            block!("grass_block", { snowy: false }),
        );

        let dirt_depth = (self.dirt_depth_noise.get(f32::from(x), f32::from(z)) * 5.0) + 3.0;
        for i in 1..=dirt_depth as i16 {
            chunk.set_block(ChunkBlockPos::new(x, surface_y - i, z), block!("dirt"));
        }

        Ok(())
    }

    fn tree_at(&self, global_x: i32, global_z: i32, surface_y: i16) -> Option<Tree> {
        if !self.trees.should_place_tree(global_x, global_z, surface_y) {
            return None;
        }
        Some(Tree {
            kind: self.tree_kind(global_x, global_z),
            surface_y,
            trunk_height: self.trees.trunk_height(global_x, global_z, 5, 3),
        })
    }

    fn new(seed: u64) -> Self {
        ForestBiome {
            seed,
            dirt_depth_noise: NoiseGenerator::new(seed, 0.1, 4, None),
            // Dense canopy: tight spacing and a low density threshold so most of the biome is wooded
            // with only small clearings.
            trees: TreePlacer::new(seed.wrapping_add(13), 0.02, 3, 0.30, TREE_MIN_SURFACE_Y),
        }
    }
}
