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

        match token.parse::<i32>() {
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

    fn completions(
        &self,
        _ctx: Arc<CommandContext>,
        input: Arc<Mutex<CommandInput>>,
    ) -> Vec<String> {
        let input = input.lock().unwrap();

        let mut numbers = Vec::new();
        let token = input.peek_string();

        let input_num = if token == "-" {
            "-0".to_string()
        } else if token.is_empty() {
            "0".to_string()
        } else {
            token
        };

        if input_num.parse::<i32>().is_err() {
            return numbers;
        }

        for n in 0..=9 {
            let n = n.to_string();
            numbers.push(input_num.clone() + &n);
        }

        numbers
    }
}
