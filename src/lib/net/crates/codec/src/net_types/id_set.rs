use crate::decode::errors::NetDecodeError;
use crate::decode::{NetDecode, NetDecodeOpts};
use crate::encode::errors::NetEncodeError;
use crate::encode::{NetEncode, NetEncodeOpts};
use crate::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

/// An ID Set is a data structure used to represent a set of IDs.
/// It can be either a tag name (referring to a registry tag) or an explicit list of IDs.
pub struct IDSet {
    /// 0 for tag name mode, otherwise represents (length + 1) for ID list mode
    pub id_type: VarInt,
    /// Present only when id_type is 0 - refers to a registry tag
    pub tag_name: Option<String>,
    /// Present only when id_type is > 0 - explicit list of registry IDs
    pub ids: Option<Vec<VarInt>>,
}

impl NetEncode for IDSet {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.id_type.encode(writer, opts)?;
        if self.id_type.0 == 0 {
            if let Some(ref tag_name) = self.tag_name {
                tag_name.encode(writer, opts)?;
            }
        } else if let Some(ref ids) = self.ids {
            for id in ids {
                id.encode(writer, opts)?;
            }
        }
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.id_type.encode_async(writer, opts).await?;
        if self.id_type.0 == 0 {
            if let Some(ref tag_name) = self.tag_name {
                tag_name.encode_async(writer, opts).await?;
            }
        } else if let Some(ref ids) = self.ids {
            for id in ids {
                id.encode_async(writer, opts).await?;
            }
        }
        Ok(())
    }
}

impl NetDecode for IDSet {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id_type = VarInt::decode(reader, opts)?;
        if id_type.0 == 0 {
            let tag_name = String::decode(reader, opts)?;
            Ok(IDSet {
                id_type,
                tag_name: Some(tag_name),
                ids: None,
            })
        } else {
            let count = (id_type.0 - 1) as usize;
            let mut ids = Vec::with_capacity(count);
            for _ in 0..count {
                ids.push(VarInt::decode(reader, opts)?);
            }
            Ok(IDSet {
                id_type,
                tag_name: None,
                ids: Some(ids),
            })
        }
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let id_type = VarInt::decode_async(reader, opts).await?;
        if id_type.0 == 0 {
            let tag_name = String::decode_async(reader, opts).await?;
            Ok(IDSet {
                id_type,
                tag_name: Some(tag_name),
                ids: None,
            })
        } else {
            let count = (id_type.0 - 1) as usize;
            let mut ids = Vec::with_capacity(count);
            for _ in 0..count {
                ids.push(VarInt::decode_async(reader, opts).await?);
            }
            Ok(IDSet {
                id_type,
                tag_name: None,
                ids: Some(ids),
            })
        }
    }
}
