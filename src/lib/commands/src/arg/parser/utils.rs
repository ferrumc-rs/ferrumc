use std::error::Error;

use ferrumc_text::{NamedColor, TextComponent, TextComponentBuilder};

use crate::errors::CommandError;

pub fn parser_error(message: &'static str) -> TextComponent {
    error(CommandError::ParserError(message.to_string()))
}

pub fn error(err: impl Error) -> TextComponent {
    TextComponentBuilder::new(err.to_string())
        .color(NamedColor::Red)
        .build()
}
