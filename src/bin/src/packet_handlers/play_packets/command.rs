use std::sync::Arc;

use bevy_ecs::prelude::*;
use ferrumc_commands::{
    infrastructure,
    messages::{CommandDispatched, ResolvedCommandDispatched},
    Command, CommandContext, CommandInput, Sender,
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
    receiver: Res<ChatCommandPacketReceiver>,
    mut dispatch_msgs: MessageWriter<CommandDispatched>,
    mut resolved_dispatch_msgs: MessageWriter<ResolvedCommandDispatched>,
) {
    for (event, entity) in receiver.0.try_iter() {
        let sender = Sender::Player(entity);
        dispatch_msgs.write(CommandDispatched {
            command: event.command.clone(),
            sender,
        });

        let resolved = resolve(event.command, sender);
        match resolved {
            Err(err) => {
                mq::queue(*err, false, entity);
            }

            Ok((command, ctx)) => {
                resolved_dispatch_msgs.write(ResolvedCommandDispatched {
                    command,
                    ctx,
                    sender,
                });
            }
        }
    }
}
