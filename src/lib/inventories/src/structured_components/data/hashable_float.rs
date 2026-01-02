use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ordered_float::NotNan;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq)]
pub struct HashableF32(pub NotNan<f32>);

impl NetEncode for HashableF32 {
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

impl NetDecode for HashableF32 {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let value = f32::decode(reader, opts)?;

        let result = NotNan::new(value).map_err(|e| NetDecodeError::ExternalError(e.into()))?;

        Ok(HashableF32(result))
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let value = f32::decode_async(reader, opts).await?;

        let result = NotNan::new(value).map_err(|e| NetDecodeError::ExternalError(e.into()))?;

        Ok(HashableF32(result))
    }
}

impl From<HashableF32> for f32 {
    fn from(wrapper: HashableF32) -> Self {
        wrapper.0.into_inner()
    }
}

impl From<&HashableF32> for f32 {
    fn from(wrapper: &HashableF32) -> Self {
        wrapper.0.into_inner()
    }
}

impl From<NotNan<f32>> for HashableF32 {
    fn from(val: NotNan<f32>) -> Self {
        Self(val)
    }
}

impl TryFrom<f32> for HashableF32 {
    type Error = ordered_float::FloatIsNan;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Self(NotNan::new(value)?))
    }
}

impl std::ops::Deref for HashableF32 {
    type Target = NotNan<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
