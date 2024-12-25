use std::any::Any;
use parser::ArgumentParser;

pub mod parser;

pub struct CommandArgument {
    pub name: String,
    pub required: bool,
    pub parser: Box<dyn ArgumentParser<Output=dyn Any>>,
}

impl CommandArgument {
    pub fn new(name: String, required: bool, parser: Box<dyn ArgumentParser<Output=dyn Any>>) -> Self {
        CommandArgument {
            name,
            required,
            parser,
        }
    }
}
