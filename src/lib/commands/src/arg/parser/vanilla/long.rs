use std::io::Write;

use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use tokio::io::AsyncWrite;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LongParserFlags {
    pub min: Option<i64>,
    pub max: Option<i64>,
}

impl NetEncode for LongParserFlags {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        let mut flags = 0u8;
        if self.min.is_some() {
            flags |= 0x01;
        }
        if self.max.is_some() {
            flags |= 0x02;
        }
        flags.encode(writer, opts)?;
        self.min.encode(writer, opts)?;
        self.max.encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        let mut flags = 0u8;
        if self.min.is_some() {
            flags |= 0x01;
        }
        if self.max.is_some() {
            flags |= 0x02;
        }
        flags.encode_async(writer, opts).await?;
        self.min.encode_async(writer, opts).await?;
        self.max.encode_async(writer, opts).await
    }
}
