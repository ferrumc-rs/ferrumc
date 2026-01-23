use std::str::FromStr;

mod simple;
mod double_block_half;
mod note_block_instrument;

pub use simple::*;
pub use double_block_half::DoubleBlockHalf;
pub use note_block_instrument::NoteBlockInstrument;

/// Marker trait for types that can be used as block state property values
pub trait BlockStateProperty: FromStr + ToString {}

impl BlockStateProperty for i32 {}
impl BlockStateProperty for bool {}

/// Helper macro to implement enum property types
#[macro_export]
macro_rules! enum_property {
    ($name:ident, $($variant:ident => $variant_str:expr),* $(,)?) => {
        pub enum $name {
            $($variant),*
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($variant_str => Ok($name::$variant)),*,
                    s => Err(format!("Unknown property for {}: {}", stringify!($name), s)),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $($name::$variant => write!(f, $variant_str)),*
                }
            }
        }

        impl $crate::BlockStateProperty for $name {}
    };
}