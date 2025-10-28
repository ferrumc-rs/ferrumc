use crate::{
    ChunkAccess,
    blocktag::{BAMBOO_PLANTABLE_ON, DIRT, SMALL_DRIPLEAF_PLACEABLE},
    direction::Direction,
    pos::BlockPos,
};
use ferrumc_macros::match_block;
use ferrumc_world::block_id::BlockId;

fn has(block: BlockId, key: &str, value: &str) -> bool {
    todo!()
}
fn is(block: BlockId, name: &str, key: &str, value: &str) -> bool {
    todo!()
}

pub fn can_survive(block: BlockId, level: &ChunkAccess, pos: BlockPos) -> bool {
    let below = level.get_block_state(pos + Direction::Down);
    match block {
        _ if match_block!("small_dripleaf", block) => {
            if has(block, "half", "upper") {
                match_block!("small_dripleaf", below) && !has(below, "half", "upper")
            } else {
                SMALL_DRIPLEAF_PLACEABLE.contains(&below)
                    || is(level.get_block_state(pos), "minecraft:water", "level", "0")
                        && (match_block!("farmland", below) || DIRT.contains(&below))
            }
        }
        _ if match_block!("bamboo", block) => BAMBOO_PLANTABLE_ON.contains(&below),
        _ => true,
    }
}

pub fn get_block_support_shape(block: BlockId, level: &ChunkAccess, pos: BlockPos) {}
pub fn get_block_collision_shape(block: BlockId, level: &ChunkAccess, pos: BlockPos) {}
