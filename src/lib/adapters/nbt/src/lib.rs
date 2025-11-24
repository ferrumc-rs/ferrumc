#![allow(unsafe_code)]

pub mod de;
pub mod errors;
pub mod ser;
mod nbt;

pub type Result<T> = std::result::Result<T, NBTError>;

pub use de::borrow::{NbtTape, NbtTapeElement};
pub use de::converter::FromNbt;
pub use errors::NBTError;
pub use ser::{NBTSerializable, NBTSerializeOptions};
pub use nbt::NBT;

pub use tokio;
