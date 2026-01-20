#![allow(unsafe_code)]

pub mod de;
pub mod errors;
mod nbt;
pub mod ser;

pub type Result<T> = std::result::Result<T, NBTError>;

pub use de::borrow::{NbtTag, NbtTape, NbtTapeElement};
pub use de::converter::FromNbt;
pub use de::streaming::read_nbt_bytes;
pub use errors::NBTError;
pub use nbt::NBT;
pub use ser::{NBTSerializable, NBTSerializeOptions};

pub use tokio;
