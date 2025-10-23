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
/// Represents the type of string argument a command can accept.
#[derive(Clone, Debug, PartialEq, Ordinalize, Default)]
pub enum StringArgumentType {
    /// A single-word string argument.
    #[default]
    Word,
    /// A string argument that can be quoted to include spaces.
    Quotable,
    /// A string argument that consumes the rest of the input.
    Greedy,
}

wrapper! {
    /// A single-word string.
    ///
    /// Accepts a single word without spaces.
    struct SingleWord(String);

    /// A quotable string.
    ///
    /// Accepts either a single word or a quoted string that may contain spaces.
    struct QuotableString(String);

    /// A greedy string.
    ///
    /// Consumes the remainder of the command input as a single string.
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
