#![feature(portable_simd)]
#![allow(unsafe_code)]
#![allow(dead_code)]
extern crate core;

use hashbrown as _;

pub mod de;
pub mod errors;
pub mod ser;
#[cfg(test)]
mod tests;

pub(crate) type Result<T> = std::result::Result<T, errors::NBTError>;

pub use errors::NBTError;
pub use ser::{NBTSerializable, NBTSerializeOptions};


pub fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>> {
    use std::io::Read;
    use libflate::gzip::Decoder;

    let mut decoder = Decoder::new(data)?;
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    Ok(decompressed)
}