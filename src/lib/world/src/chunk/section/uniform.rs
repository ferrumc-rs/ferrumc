use crate::chunk::BlockStateId;

pub struct UniformSection(BlockStateId);

impl UniformSection {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get_block(&self) -> BlockStateId {
        self.0
    }

    pub fn fill(&mut self, id: BlockStateId) {
        self.0 = id;
    }
}