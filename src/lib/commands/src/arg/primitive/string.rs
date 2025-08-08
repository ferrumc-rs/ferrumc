use std::io::Write;

use enum_ordinalize::Ordinalize;
use ferrumc_net_codec::{
    encode::{errors::NetEncodeError, NetEncode, NetEncodeOpts},
    net_types::var_int::VarInt,
};
use tokio::io::AsyncWrite;

use crate::{
    arg::{utils::parser_error, CommandArgument, ParserResult},
    ctx::CommandContext,
    wrapper,
};

use super::PrimitiveArgument;

#[derive(Clone, Debug, PartialEq, Ordinalize, Default)]
pub enum StringArgumentType {
    #[default]
    Word,
    Quotable,
    Greedy,
}

impl NetEncode for StringArgumentType {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt::new(self.ordinal() as i32).encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt::new(self.ordinal() as i32)
            .encode_async(writer, opts)
            .await
    }
}

wrapper! {
    /// A single-word string.
    struct SingleWord(String);

    /// A quotable string, accepting either a single-word string or a quoted multi-word string.
    struct QuotableString(String);

    /// A greedy string, consuming the rest of the command input.
    struct GreedyString(String);
}

impl CommandArgument for SingleWord {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let word = ctx.input.read_string();

        if word.is_empty() {
            return Err(parser_error("string must not be empty"));
        }

        Ok(SingleWord(word))
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::word()
    }
}

impl CommandArgument for QuotableString {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let input = &mut ctx.input;

        input.skip_whitespace(u32::MAX, false);

        // If it starts with a single or double quote, then parse a quotable string
        if input.peek() == Some('"') || input.peek() == Some('\'') {
            input.read(1); // Consume opening quote

            let mut result = String::new();
            let mut escaped = false;

            while input.has_remaining_input() {
                let current = input.peek();

                match current {
                    None => return Err(parser_error("unterminated quoted string")),
                    Some(c) => {
                        input.read(1);

                        if escaped {
                            match c {
                                '"' | '\\' => result.push(c),
                                _ => {
                                    result.push('\\');
                                    result.push(c);
                                }
                            }
                            escaped = false;
                        } else {
                            match c {
                                '"' | '\'' => return Ok(QuotableString(result)),
                                '\\' => escaped = true,
                                _ => result.push(c),
                            }
                        }
                    }
                }
            }

            Err(parser_error("unterminated quoted string"))
        } else {
            let word = input.read_string();

            if word.is_empty() {
                return Err(parser_error("string must not be empty"));
            }

            Ok(QuotableString(word))
        }
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::quotable()
    }
}

impl CommandArgument for GreedyString {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let input = &mut ctx.input;
        let mut result = String::new();

        while input.has_remaining_input() {
            let string = input.read_string();

            result.push_str(&string);
        }

        if result.is_empty() {
            return Err(parser_error("string cannot be empty"));
        }

        Ok(GreedyString(result))
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::greedy()
    }
}
