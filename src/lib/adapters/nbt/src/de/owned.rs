use crate::errors::NBTError;
use crate::Result;
use crate::{NbtToken, NbtTokenView};
use std::collections::HashMap;

/// Trait for converting NbtToken into owned types.
pub trait FromNbtToken<'a>: Sized

{
    /// Converts a `NbtTokenView` into an owned type `Self`.
    fn from_token(token_view: NbtTokenView<'a>) -> Result<Self>;
}

/// Implementations for primitive types.

macro_rules! impl_from_nbt_token_primitive {
    ($t:ty, $variant:ident) => {
        impl<'a> FromNbtToken<'a> for $t

        {
            fn from_token(token_view: NbtTokenView<'a>) -> Result<Self> {
                match token_view.value().ok_or(NBTError::TypeMismatch {
                    expected: stringify!($variant),
                    found: token_view.token_type(),
                })? {
                    NbtToken::$variant(v) => Ok(*v),
                    _ => Err(NBTError::TypeMismatch {
                        expected: stringify!($variant),
                        found: token_view.token_type(),
                    }),
                }
            }
        }
    };
}

impl_from_nbt_token_primitive!(i8, Byte);
impl_from_nbt_token_primitive!(i16, Short);
impl_from_nbt_token_primitive!(i32, Int);
impl_from_nbt_token_primitive!(i64, Long);
impl_from_nbt_token_primitive!(f32, Float);
impl_from_nbt_token_primitive!(f64, Double);





mod slices {
    use super::*;

    /// Macro to implement `FromNbtToken` for borrowed primitive slices.
    /*    macro_rules! impl_from_nbt_token_primitive_borrowed_slice {
            ($t:ty, $variant:ident) => {
                impl<'a> FromNbtToken<'a> for &'a [$t] {
                    #[inline]
                    fn from_token(token_view'a : &NbtTokenView<'a>) -> Result<Self> {
                        match token_view.token() {
                            NbtToken::$variant(slice) => {
                                // SAFETY: The slice in NbtToken is guaranteed to be valid and of type [$t].
                                Ok(unsafe { &*(slice as *const [_] as *const [$t]) })
                            },
                            _ => Err(NBTError::TypeMismatch {
                                expected: stringify!($variant),
                                found: token_view.token_type(),
                            }),
                        }
                    }
                }
            };
        }

        // Implement `FromNbtToken` for borrowed primitive slices.
        impl_from_nbt_token_primitive_borrowed_slice!(u8, ByteArray);
        impl_from_nbt_token_primitive_borrowed_slice!(i32, IntArray);
        impl_from_nbt_token_primitive_borrowed_slice!(i64, LongArray);*/

    impl<'a> FromNbtToken<'a> for &'a [u8]

    {
        fn from_token(token_view: NbtTokenView<'a>) -> Result<Self> {
            match token_view.value().ok_or(NBTError::TypeMismatch { expected: "ByteArray", found: token_view.token_type() })? {
                NbtToken::ByteArray(array) => {
                    let slice = array.as_ref();
                    Ok(slice)
                }
                _ => return Err(NBTError::TypeMismatch { expected: "ByteArray", found: token_view.token_type() }),
            }
        }
    }
}


/// Implementation for String.
impl<'a> FromNbtToken<'a> for String {
    fn from_token(token_view: NbtTokenView<'a>) -> Result<Self> {
        match token_view.value().ok_or(NBTError::TypeMismatch { expected: "String", found: token_view.token_type() })? {
            NbtToken::String(s) => Ok((*s).to_string()),
            _ => Err(NBTError::TypeMismatch {
                expected: "String",
                found: token_view.token_type(),
            }),
        }
    }
}
impl<'a> FromNbtToken<'a> for &'a str {
    fn from_token(token_view: NbtTokenView<'a>) -> Result<Self> {
        match token_view.value().ok_or(NBTError::TypeMismatch { expected: "String", found: token_view.token_type() })? {
            NbtToken::String(s) => Ok(s),
            _ => Err(NBTError::TypeMismatch {
                expected: "String",
                found: token_view.token_type(),
            }),
        }
    }
}

/// Implementation for Vec<T>, where T implements FromNbtToken.
impl<'a, T> FromNbtToken<'a> for Vec<T>
where
    T: FromNbtToken<'a>,
{
    fn from_token(token_view: NbtTokenView<'a>) -> Result<Self> {
        if let Some(list_view) = token_view.as_list() {
            let mut vec = Vec::with_capacity(list_view.len());
            for element in list_view.iter() {
                vec.push(T::from_token(element)?);
            }
            Ok(vec)
        } else {
            Err(NBTError::TypeMismatch {
                expected: "List",
                found: token_view.token_type(),
            })
        }
    }
}


/// Implementation for `HashMap<String, T>`, where `T` implements `FromNbtToken`.
impl<'a, T> FromNbtToken<'a> for HashMap<String, T>
where
    T: FromNbtToken<'a>,
{
    fn from_token(token_view: NbtTokenView<'a>) -> Result<Self> {
        if let Some(compound_view) = token_view.as_compound() {
            let mut map = HashMap::with_capacity(compound_view.children.len());
            for (name, child_view) in compound_view.iter() {
                let value = T::from_token(child_view)?;
                map.insert(name.to_owned(), value);
            }
            Ok(map)
        } else {
            Err(NBTError::TypeMismatch {
                expected: "Compound",
                found: token_view.token_type(),
            })
        }
    }
}

/// Extension methods for `NbtTokenView` to assist in type identification.
impl<'a> NbtTokenView<'a> {
    /// Returns the type of the current token as a string.
    pub fn token_type(&self) -> &'static str {
        match self.token() {
            NbtToken::TagStart { tag_type, .. } => match *tag_type {
                0 => "TagEnd",
                1 => "Byte",
                2 => "Short",
                3 => "Int",
                4 => "Long",
                5 => "Float",
                6 => "Double",
                7 => "ByteArray",
                8 => "String",
                9 => "List",
                10 => "Compound",
                11 => "IntArray",
                12 => "LongArray",
                _ => "Unknown",
            },
            NbtToken::TagEnd => "TagEnd",
            NbtToken::Byte(_) => "Byte",
            NbtToken::Short(_) => "Short",
            NbtToken::Int(_) => "Int",
            NbtToken::Long(_) => "Long",
            NbtToken::Float(_) => "Float",
            NbtToken::Double(_) => "Double",
            NbtToken::ByteArray(_) => "ByteArray",
            NbtToken::String(_) => "String",
            NbtToken::ListStart { .. } => "List",
            NbtToken::ListEnd => "ListEnd",
            NbtToken::IntArray(_) => "IntArray",
            NbtToken::LongArray(_) => "LongArray",
        }
    }
}
