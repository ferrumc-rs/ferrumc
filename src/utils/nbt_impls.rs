use simdnbt::borrow::{BaseNbt, NbtCompound, NbtList};

use crate::utils::error::Error;

pub struct ByteArray(pub Vec<u8>);

impl From<Vec<u8>> for ByteArray {
    fn from(value: Vec<u8>) -> Self {
        ByteArray(value)
    }
}

impl From<ByteArray> for Vec<u8> {
    fn from(value: ByteArray) -> Self {
        value.0
    }
}

impl From<&[u8]> for ByteArray {
    fn from(value: &[u8]) -> Self {
        ByteArray(value.to_vec())
    }
}

pub trait NBTDecodable {
    fn decode_from_base(nbt: BaseNbt, name: &str) -> Result<Self, Error>
    where
        Self: Sized;

    fn decode_from_compound(nbt: NbtCompound, name: &str) -> Result<Self, Error>
    where
        Self: Sized;

    fn decode_from_list(nbt: NbtList) -> Result<Vec<Self>, Error>
    where
        Self: Sized;
}

impl NBTDecodable for i8 {
    fn decode_from_base(nbt: BaseNbt, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find i8 named {} in base nbt", name).to_string(),
            ));
        }
        match nbt.byte(name) {
            Some(value) => Ok(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode i8 named {} from base nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_compound(nbt: NbtCompound, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find i8 named {} in compound nbt", name).to_string(),
            ));
        }
        match nbt.byte(name) {
            Some(value) => Ok(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode i8 named {} from compound nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_list(nbt: NbtList) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        match nbt.bytes() {
            Some(value) => Ok(value.into()),
            None => Err(Error::GenericNbtError(
                "Could not decode Vec<i8>  from list nbt"
                    .to_string()
                    .to_string(),
            )),
        }
    }
}

impl NBTDecodable for i16 {
    fn decode_from_base(nbt: BaseNbt, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find i16 named {} in base nbt", name).to_string(),
            ));
        }
        match nbt.short(name) {
            Some(value) => Ok(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode i16 named {} from base nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_compound(nbt: NbtCompound, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find i16 named {} in compound nbt", name).to_string(),
            ));
        }
        match nbt.short(name) {
            Some(value) => Ok(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode i16 named {} from compound nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_list(nbt: NbtList) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        match nbt.shorts() {
            Some(value) => Ok(value.into()),
            None => Err(Error::GenericNbtError(
                "Could not decode Vec<i16> from list nbt"
                    .to_string()
                    .to_string(),
            )),
        }
    }
}

impl NBTDecodable for i32 {
    fn decode_from_base(nbt: BaseNbt, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find i32 named {} in base nbt", name).to_string(),
            ));
        }
        match nbt.int(name) {
            Some(value) => Ok(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode i32 named {} from base nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_compound(nbt: NbtCompound, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find i32 named {} in compound nbt", name).to_string(),
            ));
        }
        match nbt.int(name) {
            Some(value) => Ok(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode i32 named {} from compound nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_list(nbt: NbtList) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        match nbt.ints() {
            Some(value) => Ok(value.into()),
            None => Err(Error::GenericNbtError(
                "Could not decode Vec<i32> from list nbt"
                    .to_string()
                    .to_string(),
            )),
        }
    }
}

impl NBTDecodable for i64 {
    fn decode_from_base(nbt: BaseNbt, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find i64 named {} in base nbt", name).to_string(),
            ));
        }
        match nbt.long(name) {
            Some(value) => Ok(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode i64 named {} from base nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_compound(nbt: NbtCompound, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find i64 named {} in compound nbt", name).to_string(),
            ));
        }
        match nbt.long(name) {
            Some(value) => Ok(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode i64 named {} from compound nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_list(nbt: NbtList) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        match nbt.longs() {
            Some(value) => Ok(value.into()),
            None => Err(Error::GenericNbtError(
                "Could not decode Vec<i64> from list nbt"
                    .to_string()
                    .to_string(),
            )),
        }
    }
}

impl NBTDecodable for f32 {
    fn decode_from_base(nbt: BaseNbt, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find f32 named {} in base nbt", name).to_string(),
            ));
        }
        match nbt.float(name) {
            Some(value) => Ok(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode f32 named {} from base nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_compound(nbt: NbtCompound, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find f32 named {} in compound nbt", name).to_string(),
            ));
        }
        match nbt.float(name) {
            Some(value) => Ok(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode f32 named {} from compound nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_list(nbt: NbtList) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        match nbt.floats() {
            Some(value) => Ok(value.into()),
            None => Err(Error::GenericNbtError(
                "Could not decode Vec<f32> from list nbt"
                    .to_string()
                    .to_string(),
            )),
        }
    }
}

impl NBTDecodable for f64 {
    fn decode_from_base(nbt: BaseNbt, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find f64 named {} in base nbt", name).to_string(),
            ));
        }
        match nbt.double(name) {
            Some(value) => Ok(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode f64 named {} from base nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_compound(nbt: NbtCompound, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find f64 named {} in compound nbt", name).to_string(),
            ));
        }
        match nbt.double(name) {
            Some(value) => Ok(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode f64 named {} from compound nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_list(nbt: NbtList) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        match nbt.doubles() {
            Some(value) => Ok(value.into()),
            None => Err(Error::GenericNbtError(
                "Could not decode Vec<f64> from list nbt"
                    .to_string()
                    .to_string(),
            )),
        }
    }
}

impl NBTDecodable for ByteArray {
    fn decode_from_base(nbt: BaseNbt, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find Vec<u8> named {} in base nbt", name).to_string(),
            ));
        }
        match nbt.byte_array(name) {
            Some(value) => Ok(value.to_vec().into()),
            None => Err(Error::GenericNbtError(
                format!("Could not decode Vec<u8> named {} from base nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_compound(nbt: NbtCompound, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find Vec<u8> named {} in compound nbt", name).to_string(),
            ));
        }
        match nbt.byte_array(name) {
            Some(value) => Ok(value.into()),
            None => Err(Error::GenericNbtError(
                format!("Could not decode Vec<u8> named {} from compound nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_list(nbt: NbtList) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        match nbt.byte_arrays() {
            Some(value) => Ok(value.to_vec().iter().map(|x| x.to_vec().into()).collect()),
            None => Err(Error::GenericNbtError(
                "Could not decode Vec<Vec<u8>> from list nbt".to_string(),
            )),
        }
    }
}

impl NBTDecodable for String {
    fn decode_from_base(nbt: BaseNbt, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find String named {} in base nbt", name).to_string(),
            ));
        }
        match nbt.string(name) {
            Some(value) => Ok(value.into()),
            None => Err(Error::GenericNbtError(
                format!("Could not decode String named {} from base nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_compound(nbt: NbtCompound, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find String named {} in compound nbt", name).to_string(),
            ));
        }
        match nbt.string(name) {
            Some(value) => Ok(value.into()),
            None => Err(Error::GenericNbtError(
                format!("Could not decode String named {} from compound nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_list(nbt: NbtList) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        match nbt.strings() {
            Some(value) => Ok(value.to_vec().iter().map(|x| x.to_string()).collect()),
            None => Err(Error::GenericNbtError(
                "Could not decode Vec<String> from list nbt".to_string(),
            )),
        }
    }
}

impl<T: NBTDecodable> NBTDecodable for Vec<T> {
    fn decode_from_base(nbt: BaseNbt, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find Vec<T> named {} in base nbt", name).to_string(),
            ));
        }
        match nbt.list(name) {
            Some(value) => T::decode_from_list(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode Vec<T> named {} from base nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_compound(nbt: NbtCompound, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Err(Error::GenericNbtError(
                format!("Could not find Vec<T> named {} in compound nbt", name).to_string(),
            ));
        }
        match nbt.list(name) {
            Some(value) => T::decode_from_list(value),
            None => Err(Error::GenericNbtError(
                format!("Could not decode Vec<T> named {} from compound nbt", name).to_string(),
            )),
        }
    }

    fn decode_from_list(nbt: NbtList) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        match nbt.lists() {
            Some(value) => {
                let mut vec = Vec::new();
                for element in value {
                    element.lists().map(|x| {
                        for y in x {
                            vec.push(T::decode_from_list(y).unwrap())
                        }
                    });
                }
                Ok(vec)
            }
            None => Err(Error::GenericNbtError(
                "Could not decode Vec<Vec<T>> from list nbt".to_string(),
            )),
        }
    }
}

impl<T: NBTDecodable> NBTDecodable for Option<T> {
    fn decode_from_base(nbt: BaseNbt, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Ok(None);
        }
        match T::decode_from_base(nbt, name) {
            Ok(value) => Ok(Some(value)),
            Err(e) => Err(e),
        }
    }

    fn decode_from_compound(nbt: NbtCompound, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !nbt.contains(name) {
            return Ok(None);
        }
        match T::decode_from_compound(nbt, name) {
            Ok(value) => Ok(Some(value)),
            Err(e) => Err(e),
        }
    }

    fn decode_from_list(nbt: NbtList) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        match T::decode_from_list(nbt) {
            Ok(value) => Ok(value.into_iter().map(|x| Some(x)).collect()),
            Err(e) => Err(e),
        }
    }
}

impl<T: NBTDecodable> NBTDecodable for T {
    fn decode_from_base(nbt: BaseNbt, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        todo!()
    }

    fn decode_from_compound(nbt: NbtCompound, name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        todo!()
    }

    fn decode_from_list(nbt: NbtList) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        todo!()
    }
}
