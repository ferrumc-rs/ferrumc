use std::io::Write;

pub mod errors;
mod primitives;

pub type NetEncodeResult<T> = Result<T, errors::NetEncodeError>;

/// Sole purpose is for compression compatibility.
/// And possibly other stuff in the future.
#[derive(Debug)]
pub enum NetEncodeOpts {
    None,
    WithLength,
    Compressed,
}

#[allow(async_fn_in_trait)]
pub trait NetEncode: Sized {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()>;
    async fn encode_async<W: tokio::io::AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()>;
}
