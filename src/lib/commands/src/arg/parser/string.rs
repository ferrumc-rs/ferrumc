use std::sync::{Arc, Mutex};

use crate::{ctx::CommandContext, input::CommandInput, ParserResult};

use super::{
    utils::parser_error,
    vanilla::{
        string::StringParsingBehavior, MinecraftArgument, MinecraftArgumentProperties,
        MinecraftArgumentType,
    },
    ArgumentParser,
};

pub struct SingleStringParser;

impl ArgumentParser for SingleStringParser {
    fn parse(&self, _ctx: Arc<CommandContext>, input: Arc<Mutex<CommandInput>>) -> ParserResult {
        let mut input = input.lock().unwrap();
        if input.peek_string().is_empty() {
            return Err(parser_error("input cannot be empty"));
        }

        Ok(Box::new(input.read_string()))
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        SingleStringParser
    }

    fn vanilla(&self) -> MinecraftArgument {
        MinecraftArgument {
            argument_type: MinecraftArgumentType::String,
            props: MinecraftArgumentProperties::String(StringParsingBehavior::default()),
        }
    }

    fn completions(
        &self,
        _ctx: Arc<CommandContext>,
        _input: Arc<Mutex<CommandInput>>,
    ) -> Vec<String> {
        vec![]
    }
}

pub struct GreedyStringParser;

impl ArgumentParser for GreedyStringParser {
    fn parse(&self, _ctx: Arc<CommandContext>, input: Arc<Mutex<CommandInput>>) -> ParserResult {
        let mut input = input.lock().unwrap();
        let mut result = String::new();

        if input.peek_string().is_empty() {
            return Err(parser_error("input cannot be empty"));
        }

        loop {
            let token = input.read_string_skip_whitespace(false);

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

    fn vanilla(&self) -> MinecraftArgument {
        MinecraftArgument {
            argument_type: MinecraftArgumentType::String,
            props: MinecraftArgumentProperties::String(StringParsingBehavior::Greedy),
        }
    }

    fn completions(
        &self,
        _ctx: Arc<CommandContext>,
        _input: Arc<Mutex<CommandInput>>,
    ) -> Vec<String> {
        vec![]
    }
}

pub struct QuotedStringParser;

impl ArgumentParser for QuotedStringParser {
    fn parse(&self, _ctx: Arc<CommandContext>, input: Arc<Mutex<CommandInput>>) -> ParserResult {
        let mut input = input.lock().unwrap();
        input.skip_whitespace(u32::MAX, false);

        // If it starts with a quote, use quoted string parsing
        if input.peek() == Some('"') {
            input.read(1); // consume the opening quote

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
        } else {
            // If no quotes, parse as single word
            if input.peek_string().is_empty() {
                return Err(parser_error("input cannot be empty"));
            }

            Ok(Box::new(input.read_string()))
        }
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        QuotedStringParser
    }

    fn vanilla(&self) -> MinecraftArgument {
        MinecraftArgument {
            argument_type: MinecraftArgumentType::String,
            props: MinecraftArgumentProperties::String(StringParsingBehavior::Quotable),
        }
    }

    fn completions(
        &self,
        _ctx: Arc<CommandContext>,
        _input: Arc<Mutex<CommandInput>>,
    ) -> Vec<String> {
        vec![]
    }
}
