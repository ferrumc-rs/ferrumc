use std::sync::Arc;

use bevy_ecs::prelude::*;
use ferrumc_commands::{
    events::{CommandDispatchEvent, ResolvedCommandDispatchEvent},
    infrastructure, Command, CommandContext, CommandInput, Sender,
};
use ferrumc_core::mq;
use ferrumc_net::ChatCommandPacketReceiver;
use ferrumc_text::{NamedColor, TextComponent, TextComponentBuilder};

fn resolve(
    input: String,
    sender: Sender,
) -> Result<(Arc<Command>, CommandContext), Box<TextComponent>> {
    let command = infrastructure::find_command(&input);
    if command.is_none() {
        return Err(Box::new(
            TextComponentBuilder::new("Unknown command")
                .color(NamedColor::Red)
                .build(),
        ));
    }

    let command = command.unwrap();
    let input = input
        .strip_prefix(command.name)
        .unwrap_or(&input)
        .trim_start();
    let input = CommandInput::of(input.to_string());
    let ctx = CommandContext {
        input: input.clone(),
        command: command.clone(),
        sender,
    };

    Ok((command, ctx))
}

pub fn handle(
    events: Res<ChatCommandPacketReceiver>,
    mut dispatch_events: EventWriter<CommandDispatchEvent>,
    mut resolved_dispatch_events: EventWriter<ResolvedCommandDispatchEvent>,
) {
    for (event, entity) in events.0.try_iter() {
        let sender = Sender::Player(entity);
        dispatch_events.write(CommandDispatchEvent {
            command: event.command.clone(),
            sender,
        });

        let resolved = resolve(event.command, sender);
        match resolved {
            Err(err) => {
                mq::queue(*err, false, entity);
            }

            Ok((command, ctx)) => {
                resolved_dispatch_events.write(ResolvedCommandDispatchEvent {
                    command,
                    ctx,
                    sender,
                });
            }
        }
    }
}
