use crate::{BlockBehavior, PlacementContext};
use ferrumc_world::pos::BlockPos;
use ferrumc_world::World;

pub struct StateBehaviorTable {
    block: &'static BlockBehaviorTable,
    id: u32,
}

impl StateBehaviorTable {
    pub const fn spin_off(block: &'static BlockBehaviorTable, id: u32) -> Self {
        Self { block, id }
    }

    #[inline(always)]
    pub fn get_placement_state(
        &self,
        context: PlacementContext,
        world: &World,
        pos: BlockPos,
    ) -> u32 {
        (self.block.get_placement_state)(self.id, context, world, pos)
    }

    #[inline(always)]
    pub fn update(&self, world: &World, pos: BlockPos) -> u32 {
        (self.block.update)(self.id, world, pos)
    }

    #[inline(always)]
    pub fn test(&self) {
        (self.block.test)(self.id)
    }
}

pub struct BlockBehaviorTable {
    get_placement_state:
        fn(id: u32, context: PlacementContext, world: &World, pos: BlockPos) -> u32,
    update: fn(id: u32, world: &World, pos: BlockPos) -> u32,
    test: fn(id: u32),
}

impl BlockBehaviorTable {
    pub const fn from<T: BlockBehavior>() -> BlockBehaviorTable {
        Self {
            get_placement_state: Self::placement_state_for::<T>,
            update: Self::update_for::<T>,
            test: Self::test_for::<T>,
        }
    }

    #[inline(always)]
    fn convert_id<T: BlockBehavior>(id: u32, func: impl FnOnce(&mut T)) -> u32 {
        let mut data = T::try_from(id).unwrap_or_else(|_| panic!("Unknown block state id: {id}"));
        func(&mut data);
        data.try_into()
            .unwrap_or_else(|_| panic!("Unknown block state"))
    }

    fn placement_state_for<T: BlockBehavior>(
        id: u32,
        context: PlacementContext,
        world: &World,
        pos: BlockPos,
    ) -> u32 {
        Self::convert_id::<T>(id, |data| data.get_placement_state(context, world, pos))
    }

    fn update_for<T: BlockBehavior>(id: u32, world: &World, pos: BlockPos) -> u32 {
        Self::convert_id::<T>(id, |data| data.update(world, pos))
    }

    fn test_for<T: BlockBehavior>(id: u32) {
        Self::convert_id::<T>(id, |data| data.test());
    }
}
