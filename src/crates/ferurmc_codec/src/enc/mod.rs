use tokio::io::AsyncWrite;

use crate::prelude::*;

mod non_primitives;
mod primitives;

pub trait NetEncode {
    #[allow(async_fn_in_trait)]
    async fn net_encode<W>(&self, writer: &mut W, encode_option: &EncodeOption) -> Result<()>
    where
        W: AsyncWrite + Unpin;
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum EncodeOption {
    /// Default encoding option, includes size depending on the type
    #[default]
    Default,
    // /// Always include the size in packet encoding
    // AlwaysIncludeSize,
    /// Always omit the size from packet encoding when available
    AlwaysOmitSize,
}
