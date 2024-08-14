#![feature(cursor_remaining)]
#![feature(portable_simd)]

pub mod nbt_spec;
pub mod error;

pub use nbt_spec::deserializer::nbt_tag_reader::{NBTTag, read_tag};
pub use nbt_spec::deserializer::{NBTDeserialize, NBTDeserializeBytes};
pub use nbt_spec::serializer::{NBTSerialize};


pub use error::NBTError;


#[cfg(feature = "derive")]
pub use nbt_derive::Serialize;

#[cfg(feature = "derive")]
pub use nbt_derive::Deserialize;
pub type NBTResult<T> = Result<T, NBTError>;
