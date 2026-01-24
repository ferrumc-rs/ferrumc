use ferrumc_macros::{block, match_block};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::{ChunkBlockPos, ChunkPos};

pub(crate) fn generate_caves(
    chunk: &mut Chunk,
    chunk_pos: ChunkPos,
    generator: &super::NoiseGenerator,
) {
    for x in 0..16u8 {
        for y in -60..100 {
            for z in 0..16u8 {
                let current_block = chunk.get_block(ChunkBlockPos::new(x, y, z));
                if match_block!("air", current_block)
                    || match_block!("cave_air", current_block)
                    || match_block!("water", current_block)
                {
                    continue; // Skip air blocks
                }
                let world_x = chunk_pos.x() * 16 + i32::from(x);
                let world_y = i32::from(y);
                let world_z = chunk_pos.z() * 16 + i32::from(z);

                let cave_noise = generator.get_cave_noise(
                    f64::from(world_x) / 2.0,
                    f64::from(world_y) / 2.0,
                    f64::from(world_z) / 2.0,
                );

                if cave_noise > 0.6 {
                    // Carve out a cave, unless the block or block above is water
                    if match_block!("water", current_block)
                        || match_block!("water", chunk.get_block(ChunkBlockPos::new(x, y + 1, z)))
                    {
                        continue;
                    }
                    chunk.set_block(ChunkBlockPos::new(x, y, z), block!("air"));
                }
            }
        }
    }
}
