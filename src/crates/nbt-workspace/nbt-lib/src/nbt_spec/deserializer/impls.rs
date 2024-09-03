use crate::error::NBTError;
use crate::nbt_spec::deserializer::nbt_tag_reader::NBTTag;
use crate::nbt_spec::deserializer::NBTDeserialize;
use crate::NBTResult;
use ferrumc_codec::network_types::varint::VarInt;
use std::collections::BTreeMap;

impl NBTDeserialize for i8 {
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Byte(val) => Ok(val),
            _ => Err(NBTError::InvalidType("i8", nbt.my_type())),
        }
    }
}

impl NBTDeserialize for i16 {
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Short(val) => Ok(val),
            _ => Err(NBTError::InvalidType("i16", nbt.my_type())),
        }
    }
}

impl NBTDeserialize for i32 {
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Int(val) => Ok(val),
            _ => Err(NBTError::InvalidType("i32", nbt.my_type())),
        }
    }
}

impl NBTDeserialize for i64 {
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Long(val) => Ok(val),
            _ => Err(NBTError::InvalidType("i64", nbt.my_type())),
        }
    }
}

impl NBTDeserialize for f32 {
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Float(val) => Ok(val),
            _ => Err(NBTError::InvalidType("f32", nbt.my_type())),
        }
    }
}

impl NBTDeserialize for f64 {
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Double(val) => Ok(val),
            _ => Err(NBTError::InvalidType("f64", nbt.my_type())),
        }
    }
}

impl NBTDeserialize for String {
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::String(val) => Ok(val),
            _ => Err(NBTError::InvalidType("String", nbt.my_type())),
        }
    }
}

impl<T: NBTDeserialize> NBTDeserialize for Vec<T> {
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::List(list) => list.into_iter().map(|tag| T::read_from(tag)).collect(),
            NBTTag::ByteArray(list) => list
                .into_iter()
                .map(|val| T::read_from(NBTTag::Byte(val)))
                .collect(),
            NBTTag::IntArray(list) => list
                .into_iter()
                .map(|val| T::read_from(NBTTag::Int(val)))
                .collect(),
            NBTTag::LongArray(list) => list
                .into_iter()
                .map(|val| T::read_from(NBTTag::Long(val)))
                .collect(),
            _ => Err(NBTError::InvalidType("Vec", nbt.my_type())),
        }
    }
}

impl NBTDeserialize for bool {
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Byte(val) => Ok(val != 0),
            _ => Err(NBTError::InvalidType("bool", nbt.my_type())),
        }
    }
}

impl<T: NBTDeserialize> NBTDeserialize for Option<T> {
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        // We're directly deserializing, since the derive macro handles the None case.
        T::read_from(nbt).map(Some)
    }
}

impl<K: NBTDeserialize, V: NBTDeserialize> NBTDeserialize for std::collections::HashMap<K, V>
where
    K: std::hash::Hash + std::cmp::Eq,
{
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Compound(map) => map
                .into_iter()
                .map(|(k, v)| Ok((K::read_from(NBTTag::String(k))?, V::read_from(v)?)))
                .collect(),
            _ => Err(NBTError::InvalidType("HashMap", nbt.my_type())),
        }
    }
}

impl<K: NBTDeserialize + std::cmp::Ord, V: NBTDeserialize> NBTDeserialize for BTreeMap<K, V>
where
    K: std::hash::Hash + std::cmp::Eq,
{
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Compound(map) => map
                .into_iter()
                .map(|(k, v)| Ok((K::read_from(NBTTag::String(k))?, V::read_from(v)?)))
                .collect(),
            _ => Err(NBTError::InvalidType("HashMap", nbt.my_type())),
        }
    }
}

impl NBTDeserialize for VarInt {
    #[inline]
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Int(val) => Ok(VarInt::from(val)),
            _ => Err(NBTError::InvalidType("VarInt", nbt.my_type())),
        }
    }
}
