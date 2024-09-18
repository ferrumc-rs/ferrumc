#![feature(portable_simd)]
#![allow(unsafe_code)]
#![allow(dead_code)]
use hashbrown as _;

pub mod de;
pub mod errors;
pub mod ser;
#[cfg(test)]
mod tests;

pub(crate) type Result<T> = std::result::Result<T, errors::NBTError>;
