use std::sync::{Arc, Mutex};

use crate::{ctx::CommandContext, input::CommandInput, ParserResult};

use super::{utils::parser_error, ArgumentParser};

pub struct SingleStringParser;

impl ArgumentParser for SingleStringParser {
    type Output = String;
    
    fn parse(&self, _ctx: Arc<&CommandContext>, input: Arc<Mutex<CommandInput>>) -> ParserResult<String> {
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
    type Output = String;
    
    fn parse(&self, _ctx: Arc<&CommandContext>, input: Arc<Mutex<CommandInput>>) -> ParserResult<String> {
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
pub struct QuotedStringParser;

impl ArgumentParser for QuotedStringParser {
    type Output = String;

    fn parse(&self, _ctx: Arc<&CommandContext>, input: Arc<Mutex<CommandInput>>) -> ParserResult<String> {
        let mut input = input.lock().unwrap();

        input.skip_whitespace(u32::MAX, false);

        if input.peek() != Some('"') {
            return Err(parser_error("expected opening quote"));
        }

        input.read(1);

        let mut result = String::new();
        let mut escaped = false;

        while input.has_remaining_input() {
            let current = input.peek();

            match current {
                None => return Err(parser_error("unterminated quoted string")),
                Some(c) => {
                    input.read(1);

                    if escaped {
                        match c {
                            '"' | '\\' => result.push(c),
                            'n' => result.push('\n'),
                            'r' => result.push('\r'),
                            't' => result.push('\t'),
                            _ => {
                                result.push('\\');
                                result.push(c);
                            }
                        }
                        escaped = false;
                    } else {
                        match c {
                            '"' => return Ok(Box::new(result)),
                            '\\' => escaped = true,
                            _ => result.push(c),
                        }
                    }
                }
            }
        }

        Err(parser_error("unterminated quoted string"))
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        QuotedStringParser
    }
}
