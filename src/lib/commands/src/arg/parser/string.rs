use std::sync::{Arc, Mutex};

use crate::{ctx::CommandContext, input::CommandInput, ParserResult};

use super::ArgumentParser;

pub struct SingleStringParser;

impl ArgumentParser for SingleStringParser {
    fn parse(&self, _ctx: Arc<&CommandContext>, input: Arc<Mutex<CommandInput>>) -> ParserResult {
        Ok(Box::new(input.lock().unwrap().read_string()))
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        SingleStringParser
    }
}

pub struct GreedyStringParser;

impl ArgumentParser for GreedyStringParser {
    fn parse(&self, _ctx: Arc<&CommandContext>, input: Arc<Mutex<CommandInput>>) -> ParserResult {
        let mut result = String::new();

        loop {
            let token = input.lock().unwrap().read_string_skip_whitespace(false);

            if token.is_empty() {
                break;
            }

            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&token);
        }

        Ok(Box::new(result))
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        GreedyStringParser
    }
}
