use crate::{
    biome::Biome,
    common::aquifer::FluidType,
    pos::{BlockPos, ChunkBlockPos, ChunkHeight, ColumnPos},
};
use ferrumc_macros::block;
use ferrumc_world::{block_state_id::BlockStateId, chunk_format::Chunk};

pub struct Surface {
    default_block: BlockStateId,
    pub chunk_height: ChunkHeight,
}

impl Surface {
    pub fn new(default_block: BlockStateId, chunk_height: ChunkHeight) -> Self {
        Self {
            default_block,
            chunk_height,
        }
    }

    pub fn find_surface(&self, chunk: &Chunk, pos: ColumnPos) -> (i32, Option<i32>) {
        let mut stone_level = self.chunk_height.min_y - 1;
        let mut fluid_level = None;
        for y in self.chunk_height.iter().rev() {
            let chunk_block_pos: ChunkBlockPos = pos.block(y).into();
            let substance = chunk
                .get_block(
                    chunk_block_pos.pos.x.into(),
                    chunk_block_pos.pos.y.into(),
                    chunk_block_pos.pos.z.into(),
                )
                .unwrap();
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
        stone_level: i32,
        mut fluid_level: Option<i32>,
        pos: ColumnPos,
        biome: Biome,
        rules: impl Fn(Biome, i32, i32, Option<i32>, BlockPos) -> Option<BlockStateId>,
        aquifer: impl Fn(BlockPos, f64) -> Option<FluidType>,
    ) -> Vec<BlockStateId> {
        let mut depth = 0;
        (self.chunk_height.min_y..=stone_level)
            .rev()
            .map(|y| {
                let substance = aquifer(
                    pos.block(y),
                    0.0, /* self.final_density.compute(pos.block(y)) TODO */
                );
                if let Some(sub) = substance {
                    if sub != FluidType::Air && fluid_level.is_none() {
                        fluid_level = Some(y);
                    }
                    return sub.into();
                }
                depth += 1;
                let depth_from_stone = y - stone_level + 1;

                rules(biome, depth, depth_from_stone, fluid_level, pos.block(y))
                    .unwrap_or(self.default_block)
            })
            .rev()
            .chain((stone_level + 1..self.chunk_height.max_y()).map(|_| Default::default()))
            .collect()
    }
}
