use crate::{
    arg::{utils::parser_error, CommandArgument, ParserResult},
    CommandContext, Suggestion,
};

use super::PrimitiveArgument;

impl CommandArgument for bool {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let str = ctx.input.read_string();

        let value = match &*str.to_lowercase() {
            "true" | "yes" | "on" | "y" => true,
            "false" | "no" | "off" | "n" => false,
            _ => return Err(parser_error(&format!("invalid variant: {str}"))),
        };

        Ok(value)
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::bool()
    }

    fn suggest(ctx: &mut CommandContext) -> Vec<Suggestion> {
        ctx.input.read_string();

        vec![Suggestion::of("true"), Suggestion::of("false")]
    }
}
