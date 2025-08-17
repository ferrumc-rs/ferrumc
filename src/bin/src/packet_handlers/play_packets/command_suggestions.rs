use std::sync::Arc;

use bevy_ecs::prelude::*;
use ferrumc_commands::{events::CommandSuggestionsEvent, Command, CommandContext, CommandInput, Sender, ROOT_COMMAND};
use ferrumc_net::{CommandSuggestionRequestReceiver, connection::StreamWriter};

fn find_command(input: String) -> Option<Arc<Command>> {
    let mut input = input;
    if input.starts_with("/") {
        input.remove(0);
    }
    
    if let Some(command) = ferrumc_commands::infrastructure::get_command_by_name(&input) {
        return Some(command)
    }
    
    if let Some(command) = ferrumc_commands::infrastructure::find_command(&input) {
        return Some(command)
    }
    
    while !input.is_empty() {
        // remove the last word and retry
        if let Some(pos) = input.rfind(char::is_whitespace) {
            input.truncate(pos);             
            
            if let Some(command) = ferrumc_commands::infrastructure::get_command_by_name(&input) {
                return Some(command)
            }
            
            if let Some(command) = ferrumc_commands::infrastructure::find_command(&input) {
                return Some(command)
            }
        } else {
            break // string does not have any further words, meaning it's just whitespace?
        }
    }
    
    None
}

fn create_ctx(input: String, command: Option<Arc<Command>>, sender: Sender) -> CommandContext {
    let input = input
        .strip_prefix(command.clone().map(|c| c.name).unwrap_or_default())
        .unwrap_or(&input)
        .trim_start();
    
    let input = CommandInput::of(input.to_string());
    CommandContext {
        input: input.clone(),
        command: command.unwrap_or(ROOT_COMMAND.clone()),
        sender,
    }
}

pub fn handle(
    events: Res<CommandSuggestionRequestReceiver>,
    query: Query<&StreamWriter>,
) {
    for (event, entity) in events.0.try_iter() {
        let command = find_command(event.input.clone());
        let ctx = create_ctx(event.input, command, Sender::Player(entity));
    }
}