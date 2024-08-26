#![feature(portable_simd)]
#![feature(extend_one)]

pub use error::NBTError;
#[cfg(feature = "derive")]
pub use nbt_derive::NBTDeserialize;
#[cfg(feature = "derive")]
pub use nbt_derive::NBTSerialize;
pub use nbt_spec::deserializer::{NBTDeserialize, NBTDeserializeBytes};
pub use nbt_spec::deserializer::nbt_tag_reader::{NBTTag, read_tag};
pub use nbt_spec::serializer::NBTSerialize;

pub mod error;
pub mod nbt_spec;

pub type NBTResult<T> = Result<T, NBTError>;
