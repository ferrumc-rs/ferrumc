use bevy_math::DVec3;
use ferrumc_core::block::BlockFace;
use crate::block::BlockUpdates;
use crate::pos::BlockPos;
use crate::World;

pub struct PlacementContext {
    pub face: BlockFace,
    pub face_position: DVec3,
}

pub trait BlockBehavior {
    fn get_placement_state(&mut self, _context: PlacementContext, _pos: BlockPos) {}
    fn random_tick(&mut self, _world: &World, _updates: &mut BlockUpdates, _pos: BlockPos) {}
}