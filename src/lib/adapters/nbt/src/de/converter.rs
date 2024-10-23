use std::collections::HashMap;
use crate::de::borrow::{NbtTape, NbtTapeElement};
use crate::{NBTError, Result};

pub trait FromNbt<'a>: Sized {
    fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self>;
}

mod primitives {
    use super::*;
    use crate::de::borrow::NbtDeserializable;

    macro_rules! impl_for_primitives {
        ($($ty:ty) | *, $variant:ident) => {
            $(
            impl FromNbt<'_> for $ty {
                fn from_nbt(_tapes: &NbtTape, element: &NbtTapeElement) -> Result<Self> {
                    match element {
                        NbtTapeElement::$variant(val) => Ok(*val as $ty),
                        _ => Err(NBTError::TypeMismatch { expected: stringify!($variant), found: element.nbt_type() }),
                    }
                }
            })*
        };
    }

    impl_for_primitives!(i8 | u8, Byte);
    impl_for_primitives!(i16 | u16, Short);
    impl_for_primitives!(i32 | u32, Int);
    impl_for_primitives!(i64 | u64, Long);
    impl_for_primitives!(f32, Float);
    impl_for_primitives!(f64, Double);

    impl FromNbt<'_> for bool {
        fn from_nbt(_tapes: &NbtTape, element: &NbtTapeElement) -> Result<Self> {
            match element {
                NbtTapeElement::Byte(val) => Ok(*val != 0),
                _ => Err(NBTError::TypeMismatch {
                    expected: "Byte",
                    found: element.nbt_type(),
                }),
            }
        }
    }

    impl FromNbt<'_> for String {
        fn from_nbt(_tapes: &NbtTape, element: &NbtTapeElement) -> Result<Self> {
            match element {
                NbtTapeElement::String(val) => Ok(val.to_string()),
                _ => Err(NBTError::TypeMismatch {
                    expected: "String",
                    found: element.nbt_type(),
                }),
            }
        }
    }

    impl<'a> FromNbt<'a> for &'a str {
        fn from_nbt(_tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
            match element {
                NbtTapeElement::String(val) => Ok(val),
                _ => Err(NBTError::TypeMismatch {
                    expected: "String",
                    found: element.nbt_type(),
                }),
            }
        }
    }

    impl<'a, T: FromNbt<'a>> FromNbt<'a> for Vec<T> {
        fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
            match tapes.unpack_list::<T>(element) {
                Some(vec) => Ok(vec),
                None => Err(NBTError::TypeMismatch {
                    expected: "List",
                    found: element.nbt_type(),
                }),
            }
        }
    }

    impl<'a, T: NbtDeserializable<'a>> FromNbt<'a> for &'a [T] {
        fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
            match tapes.unpack_list_sliced::<T>(element) {
                Some(slice) => Ok(slice),
                None => Err(NBTError::TypeMismatch {
                    expected: "List Slice (T != array type)",
                    found: element.nbt_type(),
                }),
            }
        }
    }

    // optional
    impl<'a, T: FromNbt<'a>> FromNbt<'a> for Option<T> {
        fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
            // handle optionals yourself lol (jk they're handled by the derive macro :p)
            Ok(Some(T::from_nbt(tapes, element)?))
        }
    }
}

impl<'a, V: FromNbt<'a>> FromNbt<'a> for HashMap<&'a str, V> {
    fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
        let compound = element.as_compound().ok_or(NBTError::TypeMismatch {
            expected: "Compound",
            found: element.nbt_type(),
        })?;
        // Compound: &Vec<(&str, NbtTapeElement)>, therefore we can just iterate over it and turn it into a hashmap.
        compound
            .iter()
            .map(|(key, val)| Ok((*key, V::from_nbt(tapes, val)?)))
            .collect()
    }
}


#[cfg(test)]
mod test_hashmap{
    use std::collections::HashMap;
    use crate::{FromNbt, NBTSerializable, NBTSerializeOptions};

    #[test]
    fn test_both_ways() {
        let some_hashmap = maplit::hashmap! {
            "key1" => 1,
            "key2" => 2,
            "key3" => 3,
        };

        let data = {
            let mut buf = Vec::new();
            some_hashmap.serialize(&mut buf, &NBTSerializeOptions::WithHeader("root"));
            buf
        };

        let mut tapes = crate::de::borrow::NbtTape::new(&data);
        tapes.parse();
        let root = tapes.root.as_ref().map(|(_, b)| b).unwrap();
        let hashmap = HashMap::<&str, i32>::from_nbt(&tapes, root).unwrap();

        assert_eq!(some_hashmap, hashmap);
    }
}