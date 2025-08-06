use std::io::Write;

use ferrumc_net_codec::encode::{errors::NetEncodeError, NetEncode, NetEncodeOpts};
use tokio::io::AsyncWrite;

use crate::{
    arg::{utils::error, CommandArgument},
    ctx::CommandContext,
    wrapper, ParserResult,
};

use super::PrimitiveArgument;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct FloatArgumentFlags {
    pub min: Option<f32>,
    pub max: Option<f32>,
}

impl NetEncode for FloatArgumentFlags {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
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
    ) -> Result<(), NetEncodeError> {
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

wrapper! {
    /// A 32-bit floating point number. This can sadly not be bound in size since
    /// floats are not valid const type parameters. The workaround for this is to
    /// check it manually.
    struct Float(f32);
}

impl CommandArgument for Float {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let token = ctx.input.read_string();
        let float = match token.parse::<f32>() {
            Ok(int) => int,
            Err(err) => return Err(error(err)),
        };

        Ok(Float(float))
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::float(None, None)
    }
}
