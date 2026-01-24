#![feature(min_specialization)]

use bevy_math::DVec2;
use ferrumc_block_properties::SlabType;
use ferrumc_core::block::BlockFace;
use ferrumc_macros::match_block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::BlockPos;
use ferrumc_world::World;
use std::fmt::Debug;
use ferrumc_blocks_generated::SlabBlock;

mod vtable;

pub use vtable::*;

pub const BLOCK_MAPPINGS: &[StateBehaviorTable] = include!(concat!(env!("OUT_DIR"), "/mappings.rs"));

pub struct PlacementContext {
    pub face: BlockFace,
    pub cursor: DVec2,
}

pub trait BlockBehavior: TryInto<u32, Error = ()> + TryFrom<u32, Error = ()> + Clone + Debug {
    fn get_placement_state(&mut self, context: PlacementContext, world: &World, pos: BlockPos);
    fn test(&self);
}

impl<T> BlockBehavior for T
where
    T: TryInto<u32, Error = ()> + TryFrom<u32, Error = ()> + Clone + Debug,
{
    #[inline(always)]
    default fn get_placement_state(&mut self, _context: PlacementContext, _world: &World, _pos: BlockPos) {

    }

    #[inline(always)]
    default fn test(&self) {

    }
}

impl BlockBehavior for SlabBlock {
    #[inline(always)]
    fn get_placement_state(&mut self, context: PlacementContext, world: &World, pos: BlockPos) {
        let block = world.get_block_and_fetch(pos, "overworld").unwrap_or(BlockStateId::new(0));

        self.waterlogged = match_block!("water", block);
        self.ty = match context.face {
            BlockFace::Top => SlabType::Bottom,
            BlockFace::Bottom => SlabType::Top,
            _ => if context.cursor.y > 0.5 {
                SlabType::Top
            } else {
                SlabType::Bottom
            }
        }
    }

    #[inline(always)]
    fn test(&self) {
        panic!("hello")
    }
}

#[cfg(test)]
mod tests {
    use crate::BLOCK_MAPPINGS;

    #[test]
    fn test() {
        let a = BLOCK_MAPPINGS[12051].test();
    }
}