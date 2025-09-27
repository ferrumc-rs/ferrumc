use crate::{
    biome::Biome,
    common::aquifer::FluidType,
    pos::{BlockPos, ChunkHeight, ColumnPos},
};
use ferrumc_world::{block_id::BlockId, vanilla_chunk_format::BlockData};

pub struct SurfaceRule {} //TODO
impl SurfaceRule {
    pub fn try_apply(
        &self,
        biome: Biome,
        depth: i32,
        depth_from_stone: i32,
        fluid_level: Option<i32>,
        y: BlockPos,
    ) -> Option<BlockData> {
        todo!()
    }
}
pub(crate) struct Surface {
    default_block: BlockId,
    pub chunk_height: ChunkHeight,
    pub(crate) rules: SurfaceRule,
}

impl Surface {
    pub(crate) fn new(
        default_block: BlockId,
        chunk_height: ChunkHeight,
        rules: SurfaceRule,
    ) -> Self {
        Self {
            default_block,
            chunk_height,
            rules,
        }
    }

    pub(crate) fn find_surface(
        &self,
        pos: ColumnPos,
        aquifer: impl Fn(BlockPos, f64) -> Option<FluidType>,
    ) -> (i32, Option<i32>) {
        let mut stone_level = self.chunk_height.min_y - 1;
        let mut fluid_level = None;
        for y in self.chunk_height.iter() {
            let substance = aquifer(
                pos.block(y),
                0.0, /* self.final_density.compute(pos.block(y)) TODO */
            );
            if substance.is_none() {
                stone_level = y;
                break;
            }
            if substance.is_some_and(|s| s != FluidType::Air) && fluid_level.is_none() {
                fluid_level = Some(y);
            }
        }
        (stone_level, fluid_level)
    }

    pub(crate) fn make_column(
        &self,
        stone_level: i32,
        mut fluid_level: Option<i32>,
        pos: ColumnPos,
        biome: Biome,
        aquifer: impl Fn(BlockPos, f64) -> Option<FluidType>,
    ) -> Vec<BlockData> {
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

                self.rules
                    .try_apply(biome, depth, depth_from_stone, fluid_level, pos.block(y))
                    .unwrap_or(self.default_block.to_block_data().unwrap())
            })
            .rev()
            .chain((stone_level + 1..self.chunk_height.max_y()).map(|_| Default::default()))
            .collect()
    }
}
