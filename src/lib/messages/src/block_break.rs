use bevy_ecs::prelude::Message;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::BlockPos;

/// Message sent when a block is broken in the world
#[derive(Message)]
pub struct BlockBrokenEvent {
    pub position: BlockPos,
    pub old_id: BlockStateId,
    pub new_id: BlockStateId,
}
