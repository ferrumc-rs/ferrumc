use crate::structured_components::generated::StructuredComponent;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Read;
use tokio::io::AsyncRead;
use futures::future::{LocalBoxFuture, FutureExt};

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct ExactDataComponentMatcher {
    pub component_type: VarInt,
    pub value: Box<StructuredComponent>,
}

fn decode_structured_boxed<'a, R: AsyncRead + Unpin + 'a>(
    reader: &'a mut R,
    opts: &'a NetDecodeOpts,
) -> LocalBoxFuture<'a, Result<StructuredComponent, NetDecodeError>> {
    async move {
        StructuredComponent::decode_async(reader, opts).await
    }.boxed_local()
}

impl NetDecode for ExactDataComponentMatcher {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let value = Box::new(StructuredComponent::decode(reader, opts)?);
        let component_type = StructuredComponent::to_id(&value)?;

        Ok(ExactDataComponentMatcher {
            component_type,
            value,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let value = Box::new(decode_structured_boxed(reader, opts).await?);
        let component_type = StructuredComponent::to_id(&value)?;

        Ok(ExactDataComponentMatcher {
            component_type,
            value,
        })
    }
}