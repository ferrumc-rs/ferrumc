#![feature(portable_simd)]
#![allow(unsafe_code)]
extern crate core;

use hashbrown as _;

pub mod de;
pub mod errors;
pub mod ser;
pub(crate) mod simd_utils;

pub type Result<T> = std::result::Result<T, NBTError>;

pub use errors::NBTError;
pub use ser::{NBTSerializable, NBTSerializeOptions};
pub use de::converter::FromNbt;
pub use de::borrow::{NbtTape, NbtTapeElement};

pub fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>> {
    use libflate::gzip::Decoder;
    use std::io::Read;

    if !data.starts_with(&[0x1F, 0x8B]) {
        return Ok(data.to_vec());
    }

    let mut decoder = Decoder::new(data)?;
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    Ok(decompressed)
}
