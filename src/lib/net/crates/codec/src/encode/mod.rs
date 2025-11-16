use crate::encode::errors::NetEncodeError;
use std::io::Write;
use tokio::io::AsyncWrite;

pub mod errors;
mod primitives;

/// Sole purpose is for compression compatibility.
/// And possibly other stuff in the future.
#[derive(Debug, Clone, Copy)]
#[derive(Default)]
pub enum NetEncodeOpts {
    #[default]
    None,
    WithLength,
    SizePrefixed,
}


pub trait NetEncode {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError>;

    #[expect(async_fn_in_trait)]
    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError>;
}
