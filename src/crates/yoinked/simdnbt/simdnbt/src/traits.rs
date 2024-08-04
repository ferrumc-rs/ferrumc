use std::{collections::HashMap, fmt::Display, hash::Hash, str::FromStr};
use crate::DeserializeError;

pub trait Deserialize: Sized {
    fn from_nbt(nbt: &crate::borrow::BaseNbt) -> Result<Self, DeserializeError> {
        Self::from_compound(nbt.as_compound())
    }

    fn from_compound(compound: crate::borrow::NbtCompound) -> Result<Self, DeserializeError>;
}

pub trait Serialize: Sized {
    fn to_nbt(self) -> crate::owned::BaseNbt {
        crate::owned::BaseNbt::new("", self.to_compound())
    }

    fn to_compound(self) -> crate::owned::NbtCompound;
}

pub trait FromNbtTag: Sized {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self>;
    fn from_optional_nbt_tag(
        tag: Option<crate::borrow::NbtTag>,
    ) -> Result<Option<Self>, DeserializeError> {
        match tag {
            Some(tag) => Ok(Self::from_nbt_tag(tag)),
            None => Err(DeserializeError::MissingField),
        }
    }
}

pub trait ToNbtTag: Sized {
    fn to_nbt_tag(self) -> crate::owned::NbtTag;
    fn to_optional_nbt_tag(self) -> Option<crate::owned::NbtTag> {
        Some(self.to_nbt_tag())
    }
}

impl<K: Display + FromStr + Eq + Hash, V: FromNbtTag> Deserialize for HashMap<K, V> {
    fn from_compound(compound: crate::borrow::NbtCompound) -> Result<Self, DeserializeError> {
        let mut hashmap = HashMap::with_capacity(compound.approx_len());

        for (k, v) in compound.iter() {
            let k_str = k.to_str();
            let k_parsed = k_str
                .parse()
                .map_err(|_| DeserializeError::MismatchedFieldType("key".to_owned()))?;

            let v_parsed = V::from_nbt_tag(v).ok_or_else(|| {
                DeserializeError::MismatchedFieldType(format!("value for key {k_str}"))
            })?;

            hashmap.insert(k_parsed, v_parsed);
        }

        Ok(hashmap)
    }
}
impl<K: Display + FromStr + Eq + Hash, V: ToNbtTag> Serialize for HashMap<K, V> {
    fn to_compound(self) -> crate::owned::NbtCompound {
        let mut compound = crate::owned::NbtCompound::new();

        for (k, v) in self {
            compound.insert(k.to_string(), v.to_nbt_tag());
        }

        compound
    }
}

impl Deserialize for crate::owned::NbtCompound {
    fn from_compound(compound: crate::borrow::NbtCompound) -> Result<Self, DeserializeError> {
        Ok(compound.to_owned())
    }
}
impl Serialize for crate::owned::NbtCompound {
    fn to_compound(self) -> crate::owned::NbtCompound {
        self
    }
}

impl<T: Deserialize> FromNbtTag for T {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.compound().and_then(|c| Self::from_compound(c).ok())
    }
}

impl<T: Serialize> ToNbtTag for T {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::Compound(self.to_compound())
    }
}

impl FromNbtTag for crate::owned::NbtTag {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        Some(tag.to_owned())
    }
}
impl ToNbtTag for crate::owned::NbtTag {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        self
    }
}

// standard nbt types
impl FromNbtTag for i8 {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.byte()
    }
}
impl ToNbtTag for i8 {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::Byte(self)
    }
}

impl FromNbtTag for i16 {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.short()
    }
}
impl ToNbtTag for i16 {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::Short(self)
    }
}

impl FromNbtTag for i32 {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.int()
    }
}
impl ToNbtTag for i32 {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::Int(self)
    }
}

impl FromNbtTag for i64 {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.long()
    }
}
impl ToNbtTag for i64 {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::Long(self)
    }
}

impl FromNbtTag for f32 {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.float()
    }
}
impl ToNbtTag for f32 {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::Float(self)
    }
}

impl FromNbtTag for f64 {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.double()
    }
}
impl ToNbtTag for f64 {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::Double(self)
    }
}

impl FromNbtTag for String {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.string().map(|s| s.to_string())
    }
}
impl ToNbtTag for String {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::String(self.into())
    }
}

impl ToNbtTag for &str {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::String(self.into())
    }
}

// unsigned integers
impl FromNbtTag for u8 {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.byte().map(|b| b as u8)
    }
}
impl ToNbtTag for u8 {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::Byte(self as i8)
    }
}

impl FromNbtTag for u16 {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.short().map(|s| s as u16)
    }
}
impl ToNbtTag for u16 {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::Short(self as i16)
    }
}

impl FromNbtTag for u32 {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.int().map(|i| i as u32)
    }
}
impl ToNbtTag for u32 {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::Int(self as i32)
    }
}

impl FromNbtTag for u64 {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.long().map(|l| l as u64)
    }
}
impl ToNbtTag for u64 {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::Long(self as i64)
    }
}

// lists
impl FromNbtTag for Vec<String> {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.list().and_then(|l| {
            l.strings()
                .map(|s| s.iter().map(|s| s.to_string()).collect())
        })
    }
}

impl ToNbtTag for Vec<String> {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::List(crate::owned::NbtList::String(
            self.into_iter().map(|s| s.into()).collect(),
        ))
    }
}

// slightly less standard types
impl<T: FromNbtTag> FromNbtTag for Option<T> {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        Some(T::from_nbt_tag(tag))
    }
    fn from_optional_nbt_tag(
        tag: Option<crate::borrow::NbtTag>,
    ) -> Result<Option<Self>, DeserializeError> {
        match tag {
            Some(tag) => Ok(Some(T::from_nbt_tag(tag))),
            None => Ok(Some(None)),
        }
    }
}
impl<T: ToNbtTag> ToNbtTag for Option<T> {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        panic!("Called to_nbt_tag on Option<T>. Use to_optional_nbt_tag instead.")
    }
    fn to_optional_nbt_tag(self) -> Option<crate::owned::NbtTag> {
        self.map(|t| t.to_nbt_tag())
    }
}

impl<T: Deserialize> FromNbtTag for Vec<Option<T>> {
    /// A list of compounds where `None` is an empty compound
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        let list = tag.list()?;
        let list = list.compounds()?;
        let mut vec = Vec::with_capacity(list.approx_len());
        for tag in list {
            if tag.is_empty() {
                vec.push(None);
            } else {
                vec.push(Some(T::from_compound(tag).ok()?));
            }
        }

        Some(vec)
    }
}
impl<T: Serialize> ToNbtTag for Vec<Option<T>> {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::List(crate::owned::NbtList::Compound(
            self.into_iter()
                .map(|t| match t {
                    Some(t) => t.to_compound(),
                    None => crate::owned::NbtCompound::new(),
                })
                .collect(),
        ))
    }
}

impl<T: Deserialize> FromNbtTag for Vec<T> {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        let list = tag.list()?;
        let list = list.compounds()?;
        let mut vec = Vec::with_capacity(list.approx_len());
        for tag in list {
            vec.push(T::from_compound(tag).ok()?);
        }

        Some(vec)
    }
}

impl<T: Serialize> ToNbtTag for Vec<T> {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::List(crate::owned::NbtList::Compound(
            self.into_iter().map(|t| t.to_compound()).collect(),
        ))
    }
}



impl FromNbtTag for bool {
    fn from_nbt_tag(tag: crate::borrow::NbtTag) -> Option<Self> {
        tag.byte().map(|b| b != 0)
    }
}
impl ToNbtTag for bool {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::Byte(if self { 1 } else { 0 })
    }
}

impl ToNbtTag for crate::owned::NbtList {
    fn to_nbt_tag(self) -> crate::owned::NbtTag {
        crate::owned::NbtTag::List(self)
    }
}
