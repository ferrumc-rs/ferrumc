use std::sync::{Arc, Mutex};

use crate::{ctx::CommandContext, input::CommandInput, ParserResult};

pub mod int;
pub mod string;
pub mod utils;
pub mod vanilla;

pub trait ArgumentParser: Send + Sync {
    fn parse(&self, ctx: Arc<CommandContext>, input: Arc<Mutex<CommandInput>>) -> ParserResult;
    fn completions(&self, ctx: Arc<CommandContext>, input: Arc<Mutex<CommandInput>>)
        -> Vec<String>;

    fn new() -> Self
    where
        Self: Sized;
    fn vanilla(&self) -> vanilla::MinecraftArgument;
}
