use crate::{
    arg::{CommandArgument, ParserResult},
    CommandContext, Suggestion,
};

use super::PrimitiveArgument;
use ferrumc_data::items::Item;
use ferrumc_text::TextComponent;

// Implement the trait directly for the enum
impl CommandArgument for Item {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let str = ctx.input.read_string();

        let item = match Item::from_registry_key(&str) {
            Some(item) => item,
            None => match Item::from_registry_key(&format!("minecraft:{str}")) {
                Some(item) => item,
                None => {
                    return Err(Box::new(TextComponent::from(format!(
                        "Unknown item type: {str}"
                    ))))
                }
            },
        };

        Ok(item.clone())
    }

    fn primitive() -> PrimitiveArgument {
        // We're parsing a single word
        PrimitiveArgument::word()
    }

    fn suggest(ctx: &mut CommandContext) -> Vec<Suggestion> {
        ctx.input.read_string();
        Vec::new() // unsure currently how to suggest registry keys
    }
}
