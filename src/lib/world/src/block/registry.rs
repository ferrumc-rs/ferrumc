use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::block::slabs::SlabBlock;
use crate::block::vtable::BlockBehaviorVTable;

lazy_static! {
    pub static ref BLOCK_BEHAVIOR_REGISTRY: HashMap<&'static str, &'static BlockBehaviorVTable> = register_blocks();
}

macro_rules! register_behavior {
    ($registry:expr, $name:ident, $($block:expr),* $(,)?) => {
        $(
            $registry.insert($block, $name::vtable());
        )*
    };
}

fn register_blocks() -> HashMap<&'static str, &'static BlockBehaviorVTable> {
    let mut m = HashMap::new();

    // Register block types here
    register_behavior!(m, SlabBlock, "minecraft:oak_slab", "minecraft:birch_slab");

    m
}