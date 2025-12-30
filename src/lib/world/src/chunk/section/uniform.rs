use crate::chunk::BlockStateId;
use deepsize::DeepSizeOf;

#[derive(Clone, DeepSizeOf)]
pub struct UniformSection(BlockStateId);

impl UniformSection {
    pub fn air() -> Self {
        Self(0)
    }

    pub fn new_with(id: BlockStateId) -> Self {
        Self(id)
    }

    #[inline]
    pub fn get_block(&self) -> BlockStateId {
        self.0
    }

    pub fn fill(&mut self, id: BlockStateId) {
        self.0 = id;
    }
}
