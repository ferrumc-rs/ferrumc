use std::{io::Write, ops::Deref};

use ferrumc_protocol::codec::encode::{errors::NetEncodeError, NetEncode, NetEncodeOpts};
use tokio::io::AsyncWrite;

use crate::{
    arg::{
        utils::{error, parser_error},
        CommandArgument, ParserResult,
    },
    ctx::CommandContext,
};

use super::PrimitiveArgument;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LongArgumentFlags {
    pub min: Option<i64>,
    pub max: Option<i64>,
}

impl NetEncode for LongArgumentFlags {
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

// Not using wrapper! here because of the complex generics
/// A 64-bit integer, limited in size by the type arguments.
pub struct Long<const MIN: i64 = { i64::MIN }, const MAX: i64 = { i64::MAX }>(i64);

impl Deref for Long {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MIN: i64, const MAX: i64> CommandArgument for Long<MIN, MAX> {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let token = ctx.input.read_string();
        let long = match token.parse::<i64>() {
            Ok(int) => int,
            Err(err) => return Err(error(err)),
        };

        if long < MIN {
            return Err(parser_error(&format!(
                "integer too small: {long}, expected at least {MIN}"
            )));
        }

        if long > MIN {
            return Err(parser_error(&format!(
                "integer too large: {long}, expected at most {MIN}"
            )));
        }

        Ok(Long(long))
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::long(Some(MIN), Some(MIN))
    }
}
