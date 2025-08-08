//! Command arguments.

use ferrumc_text::TextComponent;
use primitive::PrimitiveArgument;

use crate::ctx::CommandContext;

pub mod primitive;

pub type ParserResult<T> = Result<T, Box<TextComponent>>;

/// [`CommandArgument`] represents an argument that can be added to a command.
/// This is generally done by having a wrapper type around the inner value, with
/// (const) type arguments for options and implementing [`CommandArgument`] for
/// the wrapper type.
pub trait CommandArgument
where
    Self: Sized,
{
    /// Parses the argument from a command context and returns the value or a text error.
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self>;

    /// Returns the primitive argument type of this argument. This represents the
    /// vanilla parser sent to the client for client-side validation.
    fn primitive() -> PrimitiveArgument;

    /// Returns the completion strings sent to the client when typing something.
    /// This is called every time the client enters or removes a character.
    fn completions(_ctx: &mut CommandContext) -> Option<Vec<String>> {
        None
    }
}

impl<T> CommandArgument for Option<T>
where
    T: CommandArgument + Sized,
{
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        if ctx.input.has_remaining_input() {
            T::parse(ctx).map(|t| Some(t))
        } else {
            Ok(None)
        }
    }

    fn primitive() -> PrimitiveArgument {
        T::primitive()
    }
}

/// An instance of a command argument node consisting of a name, optionality and the
/// underlying [`PrimitiveArgument`] of this argument.
pub struct CommandArgumentNode {
    /// The name of the argument.
    pub name: String,

    /// Whether this argument is required or not.
    pub required: bool,

    /// The [`PrimitiveArgument`] of this argument node.
    pub primitive: PrimitiveArgument,
}

pub mod utils {
    //! Utilities related to argument parsing errors.

    use std::error::Error;

    use ferrumc_text::{NamedColor, TextComponent, TextComponentBuilder};

    use crate::errors::CommandError;

    /// Creates a [`CommandError::ParserError`] parser error from the given `message`.
    pub fn parser_error(message: &str) -> Box<TextComponent> {
        error(CommandError::ParserError(message.to_string()))
    }

    /// Creates a parser error from the given `err`.
    pub fn error(err: impl Error) -> Box<TextComponent> {
        Box::new(
            TextComponentBuilder::new(err.to_string())
                .color(NamedColor::Red)
                .build(),
        )
    }
}
