use primitive::PrimitiveArgument;

use crate::{ctx::CommandContext, ParserResult};

pub mod primitive;

pub trait CommandArgument
where
    Self: Sized,
{
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self>;

    fn primitive() -> PrimitiveArgument;

    fn completions(_ctx: &mut CommandContext) -> Option<Vec<String>> {
        None
    }
}

pub struct CommandArgumentInstance {
    pub name: String,
    pub required: bool,
    pub primitive: PrimitiveArgument,
}

pub mod utils {
    use std::error::Error;

    use ferrumc_text::{NamedColor, TextComponent, TextComponentBuilder};

    use crate::errors::CommandError;

    pub fn parser_error(message: &str) -> Box<TextComponent> {
        error(CommandError::ParserError(message.to_string()))
    }

    pub fn error(err: impl Error) -> Box<TextComponent> {
        Box::new(
            TextComponentBuilder::new(err.to_string())
                .color(NamedColor::Red)
                .build(),
        )
    }
}
