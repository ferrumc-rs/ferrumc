// I have no clue why it is saving i32 and i16. There is no precision. The actual player position is saved in f32.

use crate::decode::errors::NetDecodeError;
use crate::decode::{NetDecode, NetDecodeOpts};
use crate::encode::errors::NetEncodeError;
use crate::encode::{NetEncode, NetEncodeOpts};
use std::fmt::Display;
use std::io::{Read, Write};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

/// The definition of a "Position" in the Minecraft protocol.
#[derive(Clone, Debug, PartialEq)]
pub struct NetworkPosition {
    // Encoded as a 26 bit int
    pub x: i32,
    // Encoded as a 26 bit int
    pub z: i32,
    // Encoded as a 12 bit int
    pub y: i16,
}

impl Display for NetworkPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl NetworkPosition {
    pub fn new(x: i32, y: i16, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl NetEncode for NetworkPosition {
    fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        writer.write_all(self.as_u64().to_be_bytes().as_ref())?;
        Ok(())
    }
    async fn encode_async<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        writer
            .write_all(self.as_u64().to_be_bytes().as_ref())
            .await?;
        Ok(())
    }
}

impl NetDecode for NetworkPosition {
    fn decode<R: Read>(reader: &mut R, _: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(NetworkPosition::from_u64(u64::from_be_bytes(buf)))
    }
    async fn decode_async<R: tokio::io::AsyncRead + Unpin>(
        reader: &mut R,
        _: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf).await?;
        Ok(NetworkPosition::from_u64(u64::from_be_bytes(buf)))
    }
}

impl NetworkPosition {
    pub fn as_u64(&self) -> u64 {
        ((self.x as u64 & 0x3FFFFFF) << 38)
            | ((self.z as u64 & 0x3FFFFFF) << 12)
            | (self.y as u64 & 0xFFF)
    }

    pub fn from_u64(val: u64) -> Self {
        let mut x = (val >> 38) as i32;
        let mut y = (val << 52 >> 52) as i16;
        let mut z = (val << 26 >> 38) as i32;
        if x >= 1 << 25 {
            x -= 1 << 26
        }
        if y >= 1 << 11 {
            y -= 1 << 12
        }
        if z >= 1 << 25 {
            z -= 1 << 26
        }
        Self { x, y, z }
    }
}
