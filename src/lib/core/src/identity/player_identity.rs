use ferrumc_macros::{NetEncode, NetDecode};
use ferrumc_net_codec::{
    decode::{NetDecode, NetDecodeResult, NetDecodeOpts},
    encode::{NetEncode, NetEncodeResult, NetEncodeOpts},
    net_types::length_prefixed_vec::LengthPrefixedVec
};
use std::io::{Write, Read};
use tokio::io::AsyncWrite;

#[derive(Eq, PartialEq, Clone, Debug, NetEncode, NetDecode)]
/// The PlayerIdentity holds information about a player.
///
/// Fields:
/// `uuid`: The uuid of the PlayerIdentity.
/// `username`: The username of the PlayerIdentity.
/// `properties`: The properties of the PlayerIdentity for example textures.
///
/// ```ignore
/// PlayerIdentity {
///     uuid: Uuid::new_v4().as_u128(),
///     username: String::from("Name"),
///     properties: vec![IdentityProperty {
///         name: String::from("textures"),
///         value: String::from("ewogICJ0aW1lc3RhbXAiIDog..."),
///         is_signed: false,
///         signature: None,
///     }],
/// }
/// ```
///
pub struct PlayerIdentity {
    /// The uuid of this Identity
    pub uuid: u128,
    /// The username of this Identity
    pub username: String,
    /// The properties of this Identity
    pub properties: LengthPrefixedVec<IdentityProperty>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature(pub Option<String>);

impl NetEncode for Signature {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        (self.0.is_some()).encode(writer, opts)?;
        (self.0).encode(writer, opts)?;
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        (self.0.is_some()).encode_async(writer, opts).await?;
        (self.0).encode_async(writer, opts).await?;
        Ok(())
    }
}

impl NetDecode for Signature {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        if bool::decode(reader, opts)? {
            Ok(Signature(Some(<String>::decode(reader, opts)?)))
        } else {
            Ok(Signature(None))
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, NetEncode, NetDecode)]
/// A property of a PlayerIdentity.
///
/// Fields:
/// `name`: The name of the Property.
/// `value`: The value of the Property.
/// `signature`: The signature of the Property
///
pub struct IdentityProperty {
    /// The name of this Property.
    pub name: String,
    /// The value of this Property.
    pub value: String,
    /// The signature of this Property.
    pub signature: Signature,
}

impl PlayerIdentity {
    /// Create a new PlayerIdentity from uuid and username.
    pub fn new(username: String, uuid: u128) -> Self {
        Self {
            username,
            uuid,
            properties: LengthPrefixedVec::new(vec![])
        }
    }
}
