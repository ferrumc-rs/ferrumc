#![doc = include_str!("../README.md")]
#![feature(portable_simd)]
#![feature(array_chunks)]
#![allow(internal_features)]
#![feature(core_intrinsics)]

#[cfg(not(target_pointer_width = "64"))]
compile_error!("simdnbt only supports 64-bit platforms");

pub mod borrow;
mod common;
mod error;
mod mutf8;
pub mod owned;
pub mod raw_list;
mod reader;
pub mod swap_endianness;
mod traits;
mod impls;

pub use error::{DeserializeError, Error};
pub use mutf8::Mutf8Str;
pub use traits::{Deserialize, FromNbtTag, Serialize, ToNbtTag};

pub use simdnbt_derive::*;
