use std::fmt::Debug;
use crate::NetResult;
use std::io::Cursor;
use tokio::io::{AsyncRead, AsyncReadExt};
use ferrumc_net_codec::net_types::var_int::VarInt;

pub mod handshake;
pub mod login_start;
pub mod status_request;
pub mod ping;



pub struct PacketSkeleton {
    pub length: usize,
    pub id: u8,
    pub data: Cursor<Vec<u8>>,
}

impl Debug for PacketSkeleton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PacketSkeleton")
            .field("length", &self.length)
            .field("id", &self.id)
            .finish()
    }
}
impl PacketSkeleton {
    pub async fn new<R: AsyncRead + Unpin>(reader: &mut R) -> NetResult<Self> {
        let length = VarInt::read_async(reader).await?.val as usize;
        let mut buf = {
            let mut buf = vec![0; length];
            reader.read_exact(&mut buf).await?;

            Cursor::new(buf)
        };

        let id = VarInt::read(&mut buf)?;

        Ok(Self {
            length,
            id: id.val as u8,
            data: buf,
        })
    }
}