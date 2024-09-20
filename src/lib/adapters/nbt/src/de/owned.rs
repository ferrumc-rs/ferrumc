use crate::NbtTokenView;
use crate::errors::NBTError;
use crate::{NbtToken, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum OwnedNbtValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<OwnedNbtValue>),
    Compound(std::collections::HashMap<String, OwnedNbtValue>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
    None,
}

impl<'a, 'b> NbtTokenView<'a, 'b> {
    pub fn into_owned(self) -> Result<OwnedNbtValue> {
        let compound = self.as_compound();
        let list = self.as_list();
        let value = self.value();

        if compound.is_none() && list.is_none() && value.is_none() {
            return Ok(OwnedNbtValue::None);
        }

        if compound.is_some() {
            let owned_compound = to_compound(self)?;
            return Ok(OwnedNbtValue::Compound(owned_compound));
        }

        if list.is_some() {
            let owned_list = to_list(self)?;
            return Ok(OwnedNbtValue::List(owned_list));
        }

        match self.value().ok_or(NBTError::MissingNbtValue)? {
            NbtToken::Byte(v) => Ok(OwnedNbtValue::Byte(*v)),
            NbtToken::Short(v) => Ok(OwnedNbtValue::Short(*v)),
            NbtToken::Int(v) => Ok(OwnedNbtValue::Int(*v)),
            NbtToken::Long(v) => Ok(OwnedNbtValue::Long(*v)),
            NbtToken::Float(v) => Ok(OwnedNbtValue::Float(*v)),
            NbtToken::Double(v) => Ok(OwnedNbtValue::Double(*v)),
            NbtToken::ByteArray(v) => Ok(OwnedNbtValue::ByteArray(v.to_vec())),
            NbtToken::String(v) => Ok(OwnedNbtValue::String(v.to_string())),
            NbtToken::IntArray(v) => Ok(OwnedNbtValue::IntArray(v.to_vec())),
            NbtToken::LongArray(v) => Ok(OwnedNbtValue::LongArray(v.to_vec())),
            NbtToken::ListStart { .. } => {
                let owned_list = to_list(self)?;
                Ok(OwnedNbtValue::List(owned_list))
            }
            NbtToken::TagStart { tag_type: 10, .. } => {
                let owned_compound = to_compound(self)?;
                Ok(OwnedNbtValue::Compound(owned_compound))
            }
            _ => Ok(OwnedNbtValue::None),
        }
    }
}

fn to_compound(view: NbtTokenView) -> Result<std::collections::HashMap<String, OwnedNbtValue>> {
    let compound = view.as_compound().ok_or(NBTError::InvalidToken)?;
    let owned_compound: Result<std::collections::HashMap<String, OwnedNbtValue>> = compound.iter()
        .map(|(key, value)| Ok((key.to_string(), value.into_owned()?)))
        .collect();
    Ok(owned_compound?)
}

fn to_list(view: NbtTokenView) -> Result<Vec<OwnedNbtValue>> {
    let list = view.as_list().ok_or(NBTError::InvalidToken)?;
    let mut vec = Vec::with_capacity(list.len());
    list.iter().for_each(|item| {
        vec.push(item.into_owned().expect("Failed to convert list item to owned value"));
    });
    Ok(vec)
}