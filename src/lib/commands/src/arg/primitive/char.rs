use crate::{
    CommandContext, Suggestion,
    arg::{CommandArgument, ParserResult, utils::parser_error},
};

use super::PrimitiveArgument;

impl CommandArgument for char {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let str = ctx.input.read_string();

        if str.len() > 1 || str.is_empty() {
            return Err(parser_error("expected single character"));
        }

        Ok(str.chars().nth(0).unwrap())
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::word()
    }

    fn suggest(ctx: &mut CommandContext) -> Vec<Suggestion> {
        ctx.input.read_string();

        ('a'..'Z').map(|c| Suggestion::of(c.to_string())).collect()
    }
}
