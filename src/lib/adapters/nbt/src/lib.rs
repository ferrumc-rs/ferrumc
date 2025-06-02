#![allow(unsafe_code)]
pub mod de;
pub mod errors;
pub mod ser;

pub type Result<T> = std::result::Result<T, NBTError>;

pub use de::borrow::{NbtTape, NbtTapeElement};
pub use de::converter::FromNbt;
pub use errors::NBTError;
pub use ser::{NBTSerializable, NBTSerializeOptions};

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
