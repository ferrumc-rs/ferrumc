use crate::{
    ChunkAccess,
    blocktag::{BAMBOO_PLANTABLE_ON, DIRT, SMALL_DRIPLEAF_PLACEABLE},
    direction::Direction,
    pos::BlockPos,
};
use ferrumc_macros::block;
use ferrumc_world::block_id::BlockId;

pub fn can_survive(block: BlockId, level: &ChunkAccess, pos: BlockPos) -> bool {
    let below = level.get_block_state(pos + Direction::Down);
    match block {
        block!("small_dripleaf", { half: "upper", .. }) => match below {
            block!("small_dripleaf", { half: "upper", .. }) => false,
            block!("small_dripleaf", _) => true,
            _ => false,
        },
        block!("small_dripleaf", _) => {
            SMALL_DRIPLEAF_PLACEABLE.contains(&below)
                || matches!(level.get_block_state(pos), block!("water", { level: 0 }))
        }
        block!("bamboo", _) => BAMBOO_PLANTABLE_ON.contains(&below),
        _ => true,
    }
}

pub fn get_block_support_shape(block: BlockId, level: &ChunkAccess, pos: BlockPos) {}
pub fn get_block_collision_shape(block: BlockId, level: &ChunkAccess, pos: BlockPos) {}
