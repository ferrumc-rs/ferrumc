use std::sync::{Arc, Mutex};

use crate::{ctx::CommandContext, input::CommandInput, ParserResult};

use super::{utils::error, ArgumentParser};

pub struct IntParser;

impl ArgumentParser for IntParser {
    type Output = u32;
    
    fn parse(&self, _ctx: Arc<&CommandContext>, input: Arc<Mutex<CommandInput>>) -> ParserResult<u32> {
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
}