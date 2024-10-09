use std::io::Write;

pub mod errors;
mod primitives;

pub type NetEncodeResult<T> = Result<T, errors::NetEncodeError>;

/// Sole purpose is for compression compatibility.
/// And possibly other stuff in the future.
pub enum NetEncodeOpts {
    None,
    WithLength,
}

pub trait NetEncode: Sized {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()>;
}
