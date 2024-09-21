use super::{NBTSerializable, NBTSerializeOptions};

macro_rules! impl_ser_primitives {
    ($($($ty:ty) | * > $id:expr),*) => {
        $($(
            impl NBTSerializable for $ty {
                fn serialize(&self, buf: &mut Vec<u8>, options: & NBTSerializeOptions<'_> ) {
                    write_header::<Self>(buf, options);
                    buf.extend_from_slice(&self.to_be_bytes());
                }

                fn id() -> u8 {
                    $id
                }
            }
        )?)*
    };
}

const TAG_BYTE: u8 = 1;
const TAG_SHORT: u8 = 2;
const TAG_INT: u8 = 3;
const TAG_LONG: u8 = 4;
const TAG_FLOAT: u8 = 5;
const TAG_DOUBLE: u8 = 6;
const TAG_BYTE_ARRAY: u8 = 7;
const TAG_STRING: u8 = 8;
const TAG_LIST: u8 = 9;
const TAG_COMPOUND: u8 = 10;
const TAG_INT_ARRAY: u8 = 11;
const TAG_LONG_ARRAY: u8 = 12;

impl_ser_primitives!(
    i8 | u8 > TAG_BYTE,
    i16 | u16 > TAG_SHORT,
    i32 | u32 > TAG_INT,
    i64 | u64 > TAG_LONG,
    f32 > TAG_FLOAT,
    f64 > TAG_DOUBLE
);

impl NBTSerializable for bool {
    fn serialize(&self, buf: &mut Vec<u8>, options: &NBTSerializeOptions<'_>) {
        write_header::<Self>(buf, options);
        buf.push(if *self { 1 } else { 0 });
    }

    fn id() -> u8 {
        TAG_BYTE
    }
}

impl NBTSerializable for String {
    fn serialize(&self, buf: &mut Vec<u8>, options: &NBTSerializeOptions<'_>) {
        self.as_str().serialize(buf, options);
    }

    fn id() -> u8 {
        TAG_STRING
    }
}

impl NBTSerializable for &str {
    fn serialize(&self, buf: &mut Vec<u8>, options: &NBTSerializeOptions<'_>) {
        write_header::<Self>(buf, options);
        let bytes = self.as_bytes();
        (bytes.len() as u16).serialize(buf, &NBTSerializeOptions::None);
        buf.extend_from_slice(bytes);
    }

    fn id() -> u8 {
        TAG_STRING
    }
}

impl<T: NBTSerializable +std::fmt::Debug> NBTSerializable for Vec<T> {
    fn serialize(&self, buf: &mut Vec<u8>, options: &NBTSerializeOptions<'_>) {
        write_header::<Self>(buf, options);

        let is_special = [TAG_BYTE_ARRAY, TAG_INT_ARRAY, TAG_LONG_ARRAY].contains(&Self::id());
        
        if !is_special {
            buf.push(T::id());
        }

        (self.len() as i32).serialize(buf, &NBTSerializeOptions::None);

        if is_special {
            match Self::id() {
                TAG_BYTE_ARRAY => {
                    let bytes = unsafe {
                        std::slice::from_raw_parts(self.as_ptr() as *const u8, self.len())
                    };
                    buf.extend_from_slice(bytes);
                }
                TAG_INT_ARRAY => {
                    let bytes = unsafe {crate::simd_utils::u32_slice_to_u8_be(
                        std::slice::from_raw_parts(self.as_ptr() as *const u32, self.len())
                    )};
                    buf.extend_from_slice(bytes.as_slice());
                }
                TAG_LONG_ARRAY => {
                    let bytes = unsafe {crate::simd_utils::u64_slice_to_u8_be(
                        std::slice::from_raw_parts(self.as_ptr() as *const u64, self.len())
                    )};
                    buf.extend_from_slice(&bytes);
                }
                _ => unreachable!(),
            }
        } else {
            for item in self {
                item.serialize(buf, options);
            }
        }
    }

    #[inline]
    fn id() -> u8 {
        match T::id() {
            TAG_BYTE => TAG_BYTE_ARRAY,
            TAG_INT => TAG_INT_ARRAY,
            TAG_LONG => TAG_LONG_ARRAY,
            _ => TAG_LIST,
        }
    }
}

impl<T: NBTSerializable> NBTSerializable for Option<T> {
    fn serialize(&self, buf: &mut Vec<u8>, options: &NBTSerializeOptions<'_>) {
        match self {
            Some(value) => {
                value.serialize(buf, options);
            }
            None => {}
        }
    }

    fn id() -> u8 {
        T::id()
    }
}

mod hashmaps {
    use super::*;
    use crate::ser::NBTSerializeOptions;
    impl<T: NBTSerializable> NBTSerializable for std::collections::HashMap<String, T> {
        //! Equivalent to a COMPOUND tag in NBT.
        fn serialize(&self, buf: &mut Vec<u8>, options: &NBTSerializeOptions<'_>) {
            write_header::<Self>(buf, options);

            for (key, value) in self {
                // tag type ; name length; name
                T::id().serialize(buf, &NBTSerializeOptions::None);
                key.serialize(buf, &NBTSerializeOptions::None);
                value.serialize(buf, &NBTSerializeOptions::None);
            }

            // compounds need an ending tag too
            if let NBTSerializeOptions::WithHeader(_) = options {
                // end tag
                0u8.serialize(buf, &NBTSerializeOptions::None);
            }
        }

        fn id() -> u8 {
            TAG_COMPOUND
        }
    }
}
fn write_header<T: NBTSerializable>(buf: &mut Vec<u8>, opts: &NBTSerializeOptions<'_>) {
    // tag type ; name length; name
    let NBTSerializeOptions::WithHeader(name) = opts else {
        return;
    };

    T::id().serialize(buf, &NBTSerializeOptions::None);
    name.serialize(buf, &NBTSerializeOptions::None);
}
