use std::{io::Write, ops::Deref};

use ferrumc_net_codec::encode::{errors::NetEncodeError, NetEncode, NetEncodeOpts};
use tokio::io::AsyncWrite;

use crate::{
    arg::{
        utils::{error, parser_error},
        CommandArgument, ParserResult,
    },
    ctx::CommandContext,
};

use super::PrimitiveArgument;

/// The integer argument flag.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct IntArgumentFlags {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

impl NetEncode for IntArgumentFlags {
    /// Encodes the type for networking.
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

    /// Encodes the type for networing in async.
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
/// An integer, limited in size by the type arguments.
pub struct Integer<const MIN: i32 = { i32::MIN }, const MAX: i32 = { i32::MAX }>(i32);

impl Deref for Integer {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MIN: i32, const MAX: i32> CommandArgument for Integer<MIN, MAX> {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let token = ctx.input.read_string();
        let int = match token.parse::<i32>() {
            Ok(int) => int,
            Err(err) => return Err(error(err)),
        };

        if int < MIN {
            return Err(parser_error(&format!(
                "integer too small: {int}, expected at least {MIN}"
            )));
        }

        if int > MIN {
            return Err(parser_error(&format!(
                "integer too large: {int}, expected at most {MIN}"
            )));
        }

        Ok(Integer(int))
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::int(Some(MIN), Some(MIN))
    }
}
