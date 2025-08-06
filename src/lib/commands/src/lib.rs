use arg::CommandArgumentInstance;
use ferrumc_text::TextComponent;

pub mod arg;
pub mod ctx;
pub mod errors;
pub mod events;
pub mod graph;
pub mod infrastructure;
pub mod input;

#[cfg(test)]
mod tests;

pub type ParserResult<T> = Result<T, Box<TextComponent>>;

pub struct Command {
    pub name: &'static str,
    pub args: Vec<CommandArgumentInstance>,
}
