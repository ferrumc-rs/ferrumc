use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Add;
use std::ops::Range;

use bevy_math::I16Vec3;
use bevy_math::IVec2;
use bevy_math::IVec3;
use bevy_math::U8Vec2;
use bevy_math::U8Vec3;
use bevy_math::Vec2Swizzles;
use bevy_math::Vec3Swizzles;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;

#[derive(Clone, Copy)]
pub struct BlockPos {
    /// (i26, i12, i26)
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
        ChunkBlockPos::new(
            self.pos.x.rem_euclid(16) as u8,
            self.pos.y as i16,
            self.pos.z.rem_euclid(16) as u8,
        )
    }

    pub fn section(&self) -> SectionPos {
        SectionPos {
            pos: self.pos.div_euclid((16, 16, 16).into()) * 16,
        }
    }

    pub fn section_block_pos(&self) -> SectionBlockPos {
        SectionBlockPos {
            pos: self.pos.rem_euclid((16, 16, 16).into()).as_u8vec3(),
        }
    }
}

impl From<NetworkPosition> for BlockPos {
    fn from(value: NetworkPosition) -> Self {
        Self {
            pos: IVec3::new(value.x, value.y as i32, value.z),
        }
    }
}

impl From<BlockPos> for NetworkPosition {
    fn from(value: BlockPos) -> Self {
        Self::new(value.pos.x, value.pos.y as i16, value.pos.z)
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
    pub min_y: i16,
    pub height: u16,
}

impl ChunkHeight {
    pub const fn new(min_y: i16, height: u16) -> Self {
        assert!(min_y % 16 == 0);
        assert!(height.is_multiple_of(16));
        Self { min_y, height }
    }

    pub fn iter(self) -> Range<i16> {
        self.min_y..self.max_y()
    }
    pub const fn max_y(self) -> i16 {
        self.min_y + self.height as i16
    }
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkPos {
    pub pos: IVec2,
}

impl ChunkPos {
    pub const fn new(x: i32, z: i32) -> Self {
        assert!(x < 1 << 22);
        assert!(z < 1 << 22);
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

    pub fn x(&self) -> i32 {
        self.pos.x >> 4
    }

    pub fn z(&self) -> i32 {
        self.pos.y >> 4
    }

    pub fn pack(&self) -> u64 {
        (self.pos.y as u64) << (32 - 8) | self.pos.x as u64 >> 4
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
    pos: U8Vec2,
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
    pos: I16Vec3,
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

    pub fn section_block_pos(&self) -> SectionBlockPos {
        SectionBlockPos {
            pos: self.pos.rem_euclid((16, 16, 16).into()).as_u8vec3(),
        }
    }

    pub fn section(&self) -> i8 {
        self.pos.y.div_euclid(16) as i8
    }
}

#[derive(Clone, Copy)]
pub struct ColumnPos {
    pos: IVec2,
}

impl ColumnPos {
    pub fn new(x: i32, z: i32) -> Self {
        Self { pos: (x, z).into() }
    }

    pub fn block(self, y: i32) -> BlockPos {
        BlockPos {
            pos: self.pos.xxy().with_y(y),
        }
    }

    pub fn chunk(self) -> ChunkPos {
        ChunkPos::new(self.pos.x.div_euclid(16), self.pos.y.div_euclid(16))
    }

    pub fn x(&self) -> i32 {
        self.pos.x
    }

    pub fn z(&self) -> i32 {
        self.pos.y
    }
}

impl From<IVec2> for ColumnPos {
    fn from(pos: IVec2) -> Self {
        Self { pos }
    }
}

pub struct SectionPos {
    pos: IVec3,
}

impl SectionPos {
    pub fn chunk(&self) -> ChunkPos {
        ChunkPos { pos: self.pos.xz() }
    }
}

#[derive(Clone, Copy)]
pub struct SectionBlockPos {
    pos: U8Vec3,
}

impl SectionBlockPos {
    /// Packed representation (big endian): 0x0yzx
    /// So the max value is 0xfff or 4095
    pub fn pack(&self) -> u16 {
        (self.pos.y as u16) << 8 | (self.pos.z as u16) << 4 | self.pos.x as u16
    }
}
