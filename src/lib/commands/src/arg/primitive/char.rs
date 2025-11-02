use crate::{
    arg::{utils::parser_error, CommandArgument, ParserResult},
    CommandContext, Suggestion,
};

use super::PrimitiveArgument;

impl CommandArgument for char {
    /// Parses a string to be only one char from the client to the server.
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let str = ctx.input.read_string();

        if str.len() > 1 || str.is_empty() {
            return Err(parser_error("expected single character"));
        }

        Ok(str.chars().nth(0).unwrap())
    }

    /// Returns the Primitive Argument character.
    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::word()
    }

    /// Suggests a possible char.
    fn suggest(ctx: &mut CommandContext) -> Vec<Suggestion> {
        ctx.input.read_string();

        ('a'..'Z').map(|c| Suggestion::of(c.to_string())).collect()
    }
}
