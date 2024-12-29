use std::sync::{Arc, Mutex};

use crate::{ctx::CommandContext, input::CommandInput, ParserResult};

use super::{
    utils::error,
    vanilla::{
        int::IntParserFlags, MinecraftArgument, MinecraftArgumentProperties, MinecraftArgumentType,
    },
    ArgumentParser,
};

pub struct IntParser;

impl ArgumentParser for IntParser {
    fn parse(&self, _ctx: Arc<CommandContext>, input: Arc<Mutex<CommandInput>>) -> ParserResult {
        let token = input.lock().unwrap().read_string();

        match token.parse::<u32>() {
            Ok(int) => Ok(Box::new(int)),
            Err(err) => Err(error(err)),
        }
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        IntParser
    }

    fn vanilla(&self) -> MinecraftArgument {
        MinecraftArgument {
            argument_type: MinecraftArgumentType::Int,
            props: MinecraftArgumentProperties::Int(IntParserFlags::default()),
        }
    }
}
