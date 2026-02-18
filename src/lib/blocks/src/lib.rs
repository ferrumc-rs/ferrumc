#![feature(min_specialization)]

use bevy_math::DVec2;
use ferrumc_block_properties::SlabType;
use ferrumc_blocks_generated::{SlabBlock, SnowyBlock};
use ferrumc_core::block::BlockFace;
use ferrumc_macros::match_block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::BlockPos;
use ferrumc_world::World;

mod behavior_trait;

#[allow(unused_imports)] // Used in the include!
use crate::behavior_trait::BlockBehaviorTable;

pub use crate::behavior_trait::{BlockBehavior, StateBehaviorTable};

pub const BLOCK_MAPPINGS: &[StateBehaviorTable] =
    include!(concat!(env!("OUT_DIR"), "/mappings.rs"));

pub struct PlacementContext {
    pub face: BlockFace,
    pub cursor: DVec2,
}

impl BlockBehavior for SlabBlock {
    #[inline(always)]
    fn get_placement_state(&mut self, context: PlacementContext, world: &World, pos: BlockPos) {
        let block = world
            .get_block_and_fetch(pos, "overworld")
            .unwrap_or(BlockStateId::new(0));

        self.waterlogged = match_block!("water", block);
        self.ty = match context.face {
            BlockFace::Top => SlabType::Bottom,
            BlockFace::Bottom => SlabType::Top,
            _ => {
                if context.cursor.y > 0.5 {
                    SlabType::Top
                } else {
                    SlabType::Bottom
                }
            }
        }
    }

    #[inline(always)]
    fn test(&self) {
        panic!("hello")
    }
}

fn has_snow_above(world: &World, pos: BlockPos) -> bool {
    world
        .get_block_and_fetch(pos + (0, 1, 0), "overworld")
        .is_ok_and(|id| match_block!("snow", id))
}

impl BlockBehavior for SnowyBlock {
    fn get_placement_state(&mut self, _context: PlacementContext, world: &World, pos: BlockPos) {
        self.snowy = has_snow_above(world, pos);
    }

    fn update(&mut self, world: &World, pos: BlockPos) {
        self.snowy = has_snow_above(world, pos);
    }
}

#[cfg(test)]
mod tests {
    use crate::BLOCK_MAPPINGS;

    #[test]
    #[ignore]
    fn test() {
        BLOCK_MAPPINGS[12051].test();
    }
}
