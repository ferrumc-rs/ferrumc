use crate::{
    DensityFunction,
    biome::Biome,
    common::aquifer::FluidType,
    pos::{BlockPos, ChunkHeight, ChunkPos, ColumnPos},
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
        y: bevy_math::IVec3,
    ) -> Option<BlockData> {
        todo!()
    }
}

pub struct PreliminarySurface {
    pub chunk_height: ChunkHeight,
    noise_size_vertical: usize,
    initial_density_without_jaggedness: fn(BlockPos) -> f64,
}

impl PreliminarySurface {
    pub(crate) fn at(&self, chunk: ChunkPos) -> i32 {
        let column = chunk.column_pos(0, 0);
        self.chunk_height
            .iter()
            .rev()
            .step_by(self.noise_size_vertical)
            .find(|y| (self.initial_density_without_jaggedness)(column.block(*y)) > 0.390625)
            .unwrap_or(i32::MAX) //TODO: should this panic?
    }
}
pub(crate) struct Surface {
    pub(crate) preliminary_surface: PreliminarySurface,
    default_block: BlockId,
    final_density: DensityFunction,
    pub(crate) rules: SurfaceRule,
}

impl Surface {
    pub(crate) fn find_surface(
        &self,
        pos: ColumnPos,
        aquifer: impl Fn(BlockPos, f64) -> Option<FluidType>,
    ) -> (i32, Option<i32>) {
        let mut stone_level = self.preliminary_surface.chunk_height.min_y - 1;
        let mut fluid_level = None;
        for y in self.preliminary_surface.chunk_height.iter() {
            let substance = aquifer(pos.block(y), self.final_density.compute(pos.block(y)));
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
        (self.preliminary_surface.chunk_height.min_y..=stone_level)
            .rev()
            .map(|y| {
                let substance = aquifer(pos.block(y), self.final_density.compute(pos.block(y)));
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
            .chain(
                (stone_level + 1..self.preliminary_surface.chunk_height.max_y())
                    .map(|_| Default::default()),
            )
            .collect()
    }
}
