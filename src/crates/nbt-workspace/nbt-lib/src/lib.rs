#![feature(cursor_remaining)]

pub mod nbt_spec;
pub mod error;



#[cfg(feature = "derive")]
pub use nbt_derive::Serialize;

pub type NBTResult<T> = Result<T, error::NBTError>;
