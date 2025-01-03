use std::io::Write;

use enum_ordinalize::Ordinalize;
use ferrumc_net_codec::{
    encode::{NetEncode, NetEncodeOpts, NetEncodeResult},
    net_types::var_int::VarInt,
};
use tokio::io::AsyncWrite;

#[derive(Clone, Debug, PartialEq, Ordinalize, Default)]
pub enum StringParsingBehavior {
    #[default]
    SingleWord,
    Quotable,
    Greedy,
}

impl NetEncode for StringParsingBehavior {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        VarInt::new(self.ordinal() as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        VarInt::new(self.ordinal() as i32)
            .encode_async(writer, opts)
            .await
    }
}
