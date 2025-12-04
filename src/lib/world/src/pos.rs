use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Add;
use std::ops::Range;

use bevy_math::I16Vec3;
use bevy_math::IVec2;
use bevy_math::IVec3;
use bevy_math::U8Vec2;
use bevy_math::Vec2Swizzles;
use bevy_math::Vec3Swizzles;

#[derive(Clone, Copy)]
pub struct BlockPos {
    pub pos: IVec3,
}

impl BlockPos {
    pub fn of(x: i32, y: i32, z: i32) -> Self {
        Self {
            pos: IVec3::new(x, y, z),
        }
    }

    pub fn column(&self) -> ColumnPos {
        ColumnPos { pos: self.pos.xz() }
    }

    pub fn chunk(&self) -> ChunkPos {
        self.column().chunk()
    }

    pub fn chunk_block_pos(self) -> ChunkBlockPos {
        ChunkBlockPos::from(self)
    }
}

impl Display for BlockPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.pos.fmt(f)
    }
}

impl Add<(i32, i32, i32)> for BlockPos {
    type Output = BlockPos;

    fn add(self, rhs: (i32, i32, i32)) -> Self::Output {
        Self {
            pos: self.pos + IVec3::from(rhs),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ChunkHeight {
    pub min_y: i32,
    pub height: u32,
}

impl ChunkHeight {
    pub const fn new(min_y: i32, height: u32) -> Self {
        Self { min_y, height }
    }

    pub fn iter(self) -> Range<i32> {
        self.min_y..self.max_y()
    }
    pub const fn max_y(self) -> i32 {
        self.min_y + self.height as i32
    }
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkPos {
    pub pos: IVec2,
}

impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            pos: IVec2::new(x * 16, z * 16),
        }
    }

    pub fn center(&self) -> ColumnPos {
        self.column_pos((8, 8).into())
    }

    pub fn origin(&self) -> ColumnPos {
        self.column_pos((0, 0).into())
    }

    pub fn column_pos(&self, pos: ChunkColumnPos) -> ColumnPos {
        (self.pos + pos.pos.as_ivec2()).into()
    }

    pub fn chunk_block(&self, pos: ChunkBlockPos) -> BlockPos {
        self.column_pos(pos.pos.xz().as_u8vec2().into())
            .block(i32::from(pos.pos.y))
    }

    pub fn block_offset(&self, x: i32, y: i32, z: i32) -> BlockPos {
        ColumnPos::from(self.pos + IVec2::new(x, z)).block(y)
    }
    pub fn column_offset(&self, x: i32, z: i32) -> ColumnPos {
        ColumnPos::from(self.pos + IVec2::new(x, z))
    }
}

impl Display for ChunkPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        IVec2::fmt(&(self.pos >> 4), f)
    }
}

impl Add<(i32, i32)> for ChunkPos {
    type Output = ChunkPos;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        let pos = self.pos + IVec2::from(rhs) * 16;
        Self::Output { pos }
    }
}

pub struct ChunkColumnPos {
    pub pos: U8Vec2,
}

impl ChunkColumnPos {
    pub const fn new(x: u8, z: u8) -> Self {
        assert!(x < 16);
        assert!(z < 16);
        Self {
            pos: U8Vec2::new(x, z),
        }
    }
}

impl From<ColumnPos> for ChunkColumnPos {
    fn from(pos: ColumnPos) -> Self {
        Self {
            pos: pos.pos.rem_euclid((16, 16).into()).as_u8vec2(),
        }
    }
}

impl From<U8Vec2> for ChunkColumnPos {
    fn from(pos: U8Vec2) -> Self {
        assert!(pos.x < 16);
        assert!(pos.y < 16);
        Self { pos }
    }
}

impl From<(u8, u8)> for ChunkColumnPos {
    fn from(pos: (u8, u8)) -> Self {
        assert!(pos.0 < 16);
        assert!(pos.1 < 16);
        Self { pos: pos.into() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkBlockPos {
    pub pos: I16Vec3,
}

impl From<BlockPos> for ChunkBlockPos {
    fn from(pos: BlockPos) -> Self {
        Self::new(
            pos.pos.x.rem_euclid(16) as u8,
            pos.pos.y as i16,
            pos.pos.z.rem_euclid(16) as u8,
        )
    }
}

impl From<(u8, i16, u8)> for ChunkBlockPos {
    fn from(pos: (u8, i16, u8)) -> Self {
        Self::new(pos.0, pos.1, pos.2)
    }
}

impl ChunkBlockPos {
    pub const fn new(x: u8, y: i16, z: u8) -> Self {
        assert!(x < 16);
        assert!(z < 16);
        Self {
            pos: I16Vec3::new(x as i16, y, z as i16),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ColumnPos {
    pub pos: IVec2,
}

impl ColumnPos {
    #[deprecated]
    pub fn new(x: i32, z: i32) -> Self {
        Self { pos: (x, z).into() }
    }

    pub fn block(self, y: i32) -> BlockPos {
        BlockPos {
            pos: self.pos.xxy().with_y(y),
        }
    }

    pub fn chunk(self) -> ChunkPos {
        ChunkPos::new(self.pos.x, self.pos.y)
    }
}

impl From<IVec2> for ColumnPos {
    fn from(pos: IVec2) -> Self {
        Self { pos }
    }
}
