use crate::error::NBTError;
use crate::nbt_spec::deserializer::nbt_tag_reader::NBTTag;
use crate::nbt_spec::deserializer::NBTDeserialize;
use crate::NBTResult;

impl NBTDeserialize for i8 {
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Byte(val) => Ok(val),
            _ => Err(NBTError::InvalidType("i8", nbt.my_type()))
        }
    }
}

impl NBTDeserialize for i16 {
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Short(val) => Ok(val),
            _ => Err(NBTError::InvalidType("i16", nbt.my_type()))
        }
    }
}

impl NBTDeserialize for i32 {
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Int(val) => Ok(val),
            _ => Err(NBTError::InvalidType("i32", nbt.my_type()))
        }
    }
}

impl NBTDeserialize for i64 {
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Long(val) => Ok(val),
            _ => Err(NBTError::InvalidType("i64", nbt.my_type()))
        }
    }
}

impl NBTDeserialize for f32 {
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Float(val) => Ok(val),
            _ => Err(NBTError::InvalidType("f32", nbt.my_type()))
        }
    }
}

impl NBTDeserialize for f64 {
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Double(val) => Ok(val),
            _ => Err(NBTError::InvalidType("f64", nbt.my_type()))
        }
    }
}

impl NBTDeserialize for String {
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::String(val) => Ok(val),
            _ => Err(NBTError::InvalidType("String", nbt.my_type()))
        }
    }
}

impl<T: NBTDeserialize> NBTDeserialize for Vec<T> {
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::List(list) => list.into_iter().map(T::read_from).collect(),
            _ => Err(NBTError::InvalidType("Vec", nbt.my_type()))
        }
    }
}

impl NBTDeserialize for bool {
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        match nbt {
            NBTTag::Byte(val) => Ok(val != 0),
            _ => Err(NBTError::InvalidType("bool", nbt.my_type()))
        }
    }
}

impl<T: NBTDeserialize> NBTDeserialize for Option<T> {
    fn read_from(nbt: NBTTag) -> NBTResult<Self> {
        // We're directly deserializing, since the derive macro handles the None case.
        T::read_from(nbt).map(Some)
    }
}