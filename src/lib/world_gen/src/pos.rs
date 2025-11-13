use std::ops::Range;

use bevy_math::IVec2;
use bevy_math::IVec3;
use bevy_math::Vec2Swizzles;
use itertools::Itertools;

pub type BlockPos = IVec3;

#[derive(Clone, Copy)]
pub struct ChunkHeight {
    pub min_y: i32,
    pub height: u32,
}

impl ChunkHeight {
    pub fn iter(self) -> Range<i32> {
        self.min_y..self.max_y()
    }
    pub const fn max_y(self) -> i32 {
        self.min_y + self.height as i32
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChunkPos {
    pub pos: IVec2,
}

impl From<IVec2> for ChunkPos {
    fn from(pos: IVec2) -> Self {
        Self {
            pos: pos.div_euclid((16, 16).into()) * 16,
        }
    }
}

impl ChunkPos {
    pub fn column_pos(&self, x: u32, z: u32) -> ColumnPos {
        (self.pos + IVec2::new(x as i32, z as i32)).into()
    }
    pub fn iter_columns(self) -> impl Iterator<Item = ColumnPos> {
        (self.pos.x..self.pos.x + 16)
            .cartesian_product(self.pos.y..self.pos.y + 16)
            .map(IVec2::from)
            .map(ColumnPos::from)
    }
    pub fn block(&self, x: u32, y: i32, z: u32) -> BlockPos {
        self.column_pos(x, z).block(y)
    }
}

#[derive(Clone, Copy)]
pub struct ColumnPos {
    pub pos: IVec2,
}

impl ColumnPos {
    pub fn new(x: i32, z: i32) -> Self {
        Self { pos: (x, z).into() }
    }

    pub fn block(self, y: i32) -> BlockPos {
        self.pos.xxy().with_y(y)
    }

    pub fn chunk(self) -> ChunkPos {
        self.pos.into()
    }
}

impl From<IVec2> for ColumnPos {
    fn from(pos: IVec2) -> Self {
        Self { pos }
    }
}
