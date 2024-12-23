use std::error::Error;

use ferrumc_text::{NamedColor, TextComponent, TextComponentBuilder};

pub(crate) fn error(err: impl Error) -> TextComponent {
    TextComponentBuilder::new(err.to_string())
        .color(NamedColor::Red)
        .build()
}
