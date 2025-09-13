use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use simd_json::base::ValueAsScalar;
use simd_json::prelude::ValueAsObject;
use simd_json::prelude::ValueObjectAccess;
use std::fmt::Display;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemID(pub VarInt);

impl Display for ItemID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemID({})", self.0.0)
    }
}

impl ItemID {
    /// Creates an `ItemID` from a name, e.g. "minecraft:stone" or "stone".
    /// Is somewhat expensive, as it queries the registry.
    pub fn from_name(name: &str) -> Option<Self> {
        let name = if !name.starts_with("minecraft:") {
            format!("minecraft:{}", name)
        } else {
            name.to_string()
        };
        ferrumc_registry::lookup(format!("minecraft:item/entries/{}/protocol_id", name).as_str())
            .and_then(|scalar| scalar.as_i32())
            .map(|id| Self(VarInt::new(id)))
    }

    /// Converts the `ItemID` to a name, e.g. "minecraft:stone" or "stone".
    /// This is extremely expensive, as it iterates over the entire items registry.
    pub fn to_name(&self) -> Option<String> {
        ferrumc_registry::lookup("minecraft:item/entries").map(|entries| {
            entries.as_object().and_then(|obj| {
                obj.iter().find_map(|(key, value)| {
                    if value.get("protocol_id")?.as_i32() == Some(self.0.0) {
                        Some(key.to_string())
                    } else {
                        None
                    }
                })
            })
        })?
    }
}
impl ItemID {
    pub fn new(id: i32) -> Self {
        Self(VarInt::from(id))
    }

    pub fn as_u32(&self) -> u32 {
        self.0.0 as u32
    }
}

impl NetDecode for ItemID {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode(reader, opts)?;
        Ok(Self(id))
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let id = VarInt::decode_async(reader, opts).await?;
        Ok(Self(id))
    }
}

impl NetEncode for ItemID {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.0.encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.0.encode_async(writer, opts).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_id_from_name() {
        let item_id = ItemID::from_name("minecraft:stone");
        assert!(item_id.is_some());
        assert_eq!(item_id.unwrap().0.0, 1); // Assuming stone has ID 1
    }

    #[test]
    fn test_item_id_to_name() {
        let item_id = ItemID::new(1);
        let name = item_id.to_name();
        assert!(name.is_some());
        assert_eq!(name.unwrap(), "minecraft:stone");
    }

    #[test]
    fn test_item_id_round_trip() {
        let item_id = ItemID::new(42);
        let name = item_id.to_name().unwrap();
        let decoded = ItemID::from_name(&name).unwrap();
        assert_eq!(item_id, decoded);
    }

    #[test]
    fn test_invalid_item_name() {
        let item_name = ItemID::from_name("minecraft:nonexistent_item");
        assert!(item_name.is_none());
    }

    #[test]
    fn test_invalid_item_id() {
        let item_id = ItemID::new(i32::MAX);
        assert_eq!(item_id.0.0, i32::MAX);
        let name = item_id.to_name();
        assert!(name.is_none());
    }
}
