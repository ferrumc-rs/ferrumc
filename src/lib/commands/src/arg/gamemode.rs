use crate::{
    arg::{utils::parser_error, CommandArgument, ParserResult},
    CommandContext, Suggestion,
};

use super::PrimitiveArgument;
use ferrumc_core::player::gamemode::GameMode;

// Implement the trait directly for the enum
impl CommandArgument for GameMode {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let str = ctx.input.read_string();

        let value = match &*str.to_lowercase() {
            "survival" | "0" => GameMode::Survival,
            "creative" | "1" => GameMode::Creative,
            "adventure" | "2" => GameMode::Adventure,
            "spectator" | "3" => GameMode::Spectator,
            _ => return Err(parser_error(&format!("invalid gamemode: {str}"))),
        };

        Ok(value)
    }

    fn primitive() -> PrimitiveArgument {
        // We're parsing a single word
        PrimitiveArgument::word()
    }

    fn suggest(ctx: &mut CommandContext) -> Vec<Suggestion> {
        let input = ctx.input.read_string();

        ["survival", "creative", "adventure", "spectator"]
            .into_iter()
            .filter(move |s| s.starts_with(&input))
            .map(Suggestion::of)
            .collect()
    }
}
