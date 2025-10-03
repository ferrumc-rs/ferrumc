use crate::{
    ChunkAccess,
    blocktag::{BAMBOO_PLANTABLE_ON, DIRT, SMALL_DRIPLEAF_PLACEABLE},
    direction::Direction,
    pos::BlockPos,
};
use ferrumc_world::vanilla_chunk_format::BlockData;

fn has(block_data: BlockData, key: &str, value: &str) -> bool {
    block_data
        .properties
        .is_some_and(|p| p.get(key).is_some_and(|s| s == value))
}
fn is(block_data: BlockData, name: &str, key: &str, value: &str) -> bool {
    block_data.name == name
        && block_data
            .properties
            .is_some_and(|p| p.get(key).is_some_and(|s| s == value))
}

pub fn can_survive(block: BlockData, level: &ChunkAccess, pos: BlockPos) -> bool {
    let below = level.get_block_state(pos + Direction::Down);
    match block.name.as_str() {
        "minecraft:small_dripleaf_block" => {
            if has(block, "half", "upper") {
                below.name == "minecraft:small_dripleaf_block" && !has(below, "half", "upper")
            } else {
                SMALL_DRIPLEAF_PLACEABLE.contains(&below.to_block_id())
                    || is(level.get_block_state(pos), "minecraft:water", "level", "0")
                        && (below.name == "minecraft:farmland"
                            || DIRT.contains(&below.to_block_id()))
            }
        }
        "minecraft:bamboo" => BAMBOO_PLANTABLE_ON.contains(&below.to_block_id()),
        _ => true,
    }
}

pub fn get_block_support_shape(block_data: BlockData, level: &ChunkAccess, pos: BlockPos) {}
pub fn get_block_collision_shape(block_data: BlockData, level: &ChunkAccess, pos: BlockPos) {}
