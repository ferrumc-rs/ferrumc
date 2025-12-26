/// This struct holds the position of a chunk within the world.
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

/// This struct holds a local position of a block within a chunk
pub struct ChunkBlockPos {
    pub x: u8,
    pub z: u8,
    pub y: i16,
}

/// This struct holds a local position of a block within a chunk section
pub struct SectionBlockPos {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

/// This struct holds the position of a block within the world.
pub struct BlockPos {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}