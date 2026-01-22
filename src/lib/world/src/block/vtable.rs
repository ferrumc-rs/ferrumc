use tracing::error;
use crate::block::behavior::{BlockBehavior, PlacementContext};
use crate::block::{BlockType, BlockUpdates};
use crate::pos::BlockPos;
use crate::vanilla_chunk_format::BlockData;
use crate::World;

pub struct BlockBehaviorVTable {
    pub(super) get_placement_state: fn(
        data: &mut BlockData,
        context: PlacementContext,
        pos: BlockPos,
    ),
    pub(super) random_tick: fn(
        data: &mut BlockData,
        world: &World,
        updates: &mut BlockUpdates,
        pos: BlockPos,
    ),
}

impl BlockBehaviorVTable {
    #[inline]
    pub fn get_placement_state(&self, data: &mut BlockData, context: PlacementContext, pos: BlockPos) {
        (self.get_placement_state)(data, context, pos)
    }

    #[inline]
    pub fn random_tick(&self, data: &mut BlockData, world: &World, updates: &mut BlockUpdates, pos: BlockPos) {
        (self.random_tick)(data, world, updates, pos)
    }
}

#[inline(always)]
pub(super) fn random_tick_adapter<'data, T: BlockBehavior + BlockType + TryFrom<&'data mut BlockData, Error=String> + 'data>(
    data: &'data mut BlockData,
    world: &World,
    updates: &mut BlockUpdates,
    pos: BlockPos,
) {
    match T::try_from(data) {
        Ok(mut behavior) => {
            behavior.random_tick(world, updates, pos);
            behavior.apply_changes();
        },
        Err(msg) => error!("Failed to convert BlockData into block behavior: {msg}"),
    }
}

#[inline(always)]
pub(super) fn get_placement_state_adapter<'data, T: BlockBehavior + BlockType + TryFrom<&'data mut BlockData, Error=String> + 'data>(
    data: &'data mut BlockData,
    context: PlacementContext,
    pos: BlockPos,
) {
    match T::try_from(data) {
        Ok(mut behavior) => {
            behavior.get_placement_state(context, pos);
            behavior.apply_changes();
        },
        Err(msg) => error!("Failed to convert BlockData into block behavior: {msg}"),
    }
}