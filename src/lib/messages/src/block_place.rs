use bevy_ecs::message::Message;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::BlockPos;

/// Message sent when a block is placed in the world.
#[derive(Message)]
pub struct BlockPlaceEvent {
    pub position: BlockPos,
    pub old_id: BlockStateId,
    pub new_id: BlockStateId,
}