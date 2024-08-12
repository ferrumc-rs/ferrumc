#![feature(cursor_remaining)]

pub mod nbt_spec;
pub mod error;

pub use nbt_spec::deserializer::nbt_tag_reader::read_tag;
pub use nbt_spec::deserializer::{NBTDeserialize, NBTDeserializeBytes};


#[cfg(feature = "derive")]
pub use nbt_derive::Serialize;
#[cfg(feature = "derive")]
pub use nbt_derive::Deserialize;

pub type NBTResult<T> = Result<T, error::NBTError>;
