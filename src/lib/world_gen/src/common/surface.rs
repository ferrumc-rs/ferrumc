use crate::biome::Biome;
use ferrumc_macros::block;
use ferrumc_world::edit_batch::EditBatch;
use ferrumc_world::pos::{BlockPos, ChunkHeight, ColumnPos};
use ferrumc_world::{block_state_id::BlockStateId, chunk_format::Chunk};
use itertools::Itertools;

pub struct Surface {
    pub chunk_height: ChunkHeight,
}

impl Surface {
    pub fn new(chunk_height: ChunkHeight) -> Self {
        Self { chunk_height }
    }

    pub fn find_surface(&self, chunk: &Chunk, pos: ColumnPos) -> (i16, Option<i16>) {
        let mut stone_level = self.chunk_height.min_y - 1;
        let mut fluid_level = None;
        for y in self.chunk_height.iter().rev() {
            let chunk_block_pos = pos.block(y.into()).chunk_block_pos();
            let substance = chunk.get_block(chunk_block_pos).unwrap();
            if substance == block!("stone") || substance == block!("deepslate", {axis: "y"}) {
                stone_level = y;
                break;
            }
            if substance != block!("air") /*aka lava and water*/ && fluid_level.is_none() {
                fluid_level = Some(y);
            }
        }
        (stone_level, fluid_level)
    }

    pub fn make_column(
        &self,
        chunk: &mut Chunk,
        stone_level: i16,
        mut fluid_level: Option<i16>,
        pos: ColumnPos,
        biome: Biome,
        rules: impl Fn(&Chunk, Biome, i16, i16, Option<i16>, BlockPos) -> Option<BlockStateId>,
    ) {
        let mut stone_under = stone_level;
        let mut depth_above = 0;

        for y in (self.chunk_height.min_y..=stone_level).rev() {
            match chunk
                .get_block(pos.block(y.into()).chunk_block_pos())
                .unwrap()
            {
                block!("air") => {
                    depth_above = 0;
                    fluid_level = None;
                }
                block!("water", {level:0}) | block!("lava", {level:0}) if fluid_level.is_none() => {
                    fluid_level = Some(y + 1);
                }
                _ => {
                    if stone_under >= y
                        && let Some(new_stone_level) =
                            (self.chunk_height.min_y..y).rev().find_or_last(|i| {
                                let block = chunk
                                    .get_block(pos.block((*i).into()).chunk_block_pos())
                                    .unwrap();
                                block != block!("stone")
                                    && block != block!("deepslate", {axis: "y"})
                            })
                    {
                        stone_under = new_stone_level;
                    }
                    depth_above += 1;
                    let depth_from_stone = y - stone_under + 1;

                    if let Some(block) = rules(
                        chunk,
                        biome,
                        depth_above,
                        depth_from_stone,
                        fluid_level,
                        pos.block(y.into()),
                    ) {
                        chunk.set_block(pos.block(y.into()).chunk_block_pos(), block);
                    }
                }
            }
        }
    }
}
