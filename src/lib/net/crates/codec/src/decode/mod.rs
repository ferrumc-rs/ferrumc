use std::io::Read;

pub mod errors;
mod primitives;

pub type NetDecodeResult<T> = Result<T, errors::NetDecodeError>;

/// Sole purpose is for compression compatibility.
/// And possibly other stuff in the future.
#[derive(Debug)]
pub enum NetDecodeOpts {
    None,
    IsSizePrefixed,
}
pub trait NetDecode: Sized {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self>;
}
