use std::sync::{Arc, Mutex};

use crate::{ctx::CommandContext, input::CommandInput, ParserResult};

pub mod int;
pub mod string;
pub mod utils;

pub trait ArgumentParser: Send + Sync {
    fn parse(&self, context: Arc<CommandContext>, input: Arc<Mutex<CommandInput>>) -> ParserResult;
    fn new() -> Self
    where
        Self: Sized;
}
