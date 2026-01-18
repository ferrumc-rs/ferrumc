use bevy_ecs::prelude::Message;
use ferrumc_world::pos::BlockPos;

/// Message sent when a block is broken in the world
#[derive(Message)]
pub struct BlockBrokenEvent {
    pub position: BlockPos,
}
