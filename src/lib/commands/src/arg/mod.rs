use parser::ArgumentParser;

pub mod parser;

pub struct CommandArgument {
    pub name: String,
    pub required: bool,
    pub parser: Box<dyn ArgumentParser>,
}

impl CommandArgument {
    pub fn new(name: String, required: bool, parser: Box<dyn ArgumentParser>) -> Self {
        CommandArgument {
            name,
            required,
            parser,
        }
    }
}
