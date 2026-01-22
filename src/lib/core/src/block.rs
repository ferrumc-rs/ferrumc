use std::io::Read;
use bevy_math::IVec3;
use tokio::io::AsyncRead;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, Clone)]
pub enum BlockFace {
    Top,
    Bottom,
    North,
    South,
    East,
    West,
}

impl BlockFace {
    pub fn is_x_axis(&self) -> bool {
        matches!(self, BlockFace::East | BlockFace::West)
    }

    pub fn is_y_axis(&self) -> bool {
        matches!(self, BlockFace::Top | BlockFace::Bottom)
    }

    pub fn is_z_axis(&self) -> bool {
        matches!(self, BlockFace::North | BlockFace::South)
    }

    /// Returns the translation vector that will get the block that touches this face.
    pub fn translation_vec(&self) -> IVec3 {
        match self {
            BlockFace::Top => IVec3::new(0, 1, 0),
            BlockFace::Bottom => IVec3::new(0, -1, 0),
            BlockFace::North => IVec3::new(0, 0, -1),
            BlockFace::South => IVec3::new(0, 0, 1),
            BlockFace::East => IVec3::new(1, 0, 0),
            BlockFace::West => IVec3::new(-1, 0, 0),
        }
    }
}

impl NetDecode for BlockFace {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let VarInt(data) = VarInt::decode(reader, opts)?;

        match data {
            0 => Ok(BlockFace::Bottom),
            1 => Ok(BlockFace::Top),
            2 => Ok(BlockFace::North),
            3 => Ok(BlockFace::South),
            4 => Ok(BlockFace::West),
            5 => Ok(BlockFace::East),
            _ => Err(NetDecodeError::InvalidEnumVariant),
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let VarInt(data) = VarInt::decode_async(reader, opts).await?;

        match data {
            0 => Ok(BlockFace::Bottom),
            1 => Ok(BlockFace::Top),
            2 => Ok(BlockFace::North),
            3 => Ok(BlockFace::South),
            4 => Ok(BlockFace::West),
            5 => Ok(BlockFace::East),
            _ => Err(NetDecodeError::InvalidEnumVariant),
        }
    }
}