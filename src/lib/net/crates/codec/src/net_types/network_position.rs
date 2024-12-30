// I have no clue why it is saving i32 and i16. There is no precision. The actual player position is saved in f32.

use crate::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use crate::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use std::fmt::Display;
use std::io::{Read, Write};
use tokio::io::AsyncWrite;

/// The definition of a "Position" in the Minecraft protocol.
#[derive(Clone, Debug)]
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
    fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> NetEncodeResult<()> {
        writer.write_all(self.as_u64().to_be_bytes().as_ref())?;
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        use tokio::io::AsyncWriteExt;
        writer
            .write_all(self.as_u64().to_be_bytes().as_ref())
            .await?;
        Ok(())
    }
}

impl NetDecode for NetworkPosition {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let value = u64::decode(reader, opts)?;
        Ok(Self {
            x: (value >> 38) as i32,
            y: (value << 54 >> 52) as i16,
            z: (value << 26 >> 38) as i32,
        })
    }
}

impl NetworkPosition {
    pub fn as_u64(&self) -> u64 {
        ((self.x as u64 & 0x3FFFFFF) << 38)
            | ((self.z as u64 & 0x3FFFFFF) << 12)
            | (self.y as u64 & 0xFFF)
    }
}
