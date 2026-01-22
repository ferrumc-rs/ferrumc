use ferrumc_core::block::BlockFace;
use crate::block::behavior::{BlockBehavior, PlacementContext};
use crate::block::state::SlabType;
use crate::define_block_behavior;
use crate::pos::BlockPos;

define_block_behavior!(SlabBlock, {slab_type, type}: SlabType, {waterlogged, waterlogged}: bool);

impl BlockBehavior for SlabBlock<'_> {
    fn get_placement_state(&mut self, context: PlacementContext, _pos: BlockPos) {
        if !context.face.is_y_axis() {
            self.slab_type = if context.face_position.y >= 0.5 {
                SlabType::Top
            } else {
                SlabType::Bottom
            };
        } else {
            match context.face {
                BlockFace::Top => self.slab_type = SlabType::Bottom,
                BlockFace::Bottom => self.slab_type = SlabType::Top,
                _ => unreachable!()
            }
        }
    }
}