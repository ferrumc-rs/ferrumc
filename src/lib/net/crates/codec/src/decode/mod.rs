use crate::decode::errors::NetDecodeError;
use std::io::Read;
use tokio::io::AsyncRead;

pub mod errors;
mod primitives;

/// Sole purpose is for compression compatibility.
/// And possibly other stuff in the future.
#[derive(Debug)]
pub enum NetDecodeOpts {
    None,
    IsSizePrefixed,
}
pub trait NetDecode: Sized {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError>;

    #[expect(async_fn_in_trait)]
    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError>;
}
