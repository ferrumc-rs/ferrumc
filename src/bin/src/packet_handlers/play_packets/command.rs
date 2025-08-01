use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::*;
use ferrumc_commands::{
    ctx::CommandContext,
    events::{CommandDispatchEvent, ResolvedCommandDispatchEvent},
    infrastructure,
    input::CommandInput,
    Command,
};
use ferrumc_net::{
    connection::StreamWriter, packets::outgoing::system_message::SystemMessagePacket,
    ChatCommandPacketReceiver,
};
use ferrumc_text::{NamedColor, TextComponent, TextComponentBuilder};
use tracing::error;

fn resolve(
    input: String,
    sender: Entity,
) -> Result<(Arc<Command>, Arc<CommandContext>), Box<TextComponent>> {
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
    let ctx = CommandContext::new(input.clone(), command.clone(), sender);

    if let Err(err) = command.validate(&ctx, &Arc::new(Mutex::new(input))) {
        return Err(Box::new(
            TextComponentBuilder::new("Invalid arguments")
                .extra(*err)
                .color(NamedColor::Red)
                .build(),
        ));
    }

    Ok((command, ctx))
}

pub fn handle(
    events: Res<ChatCommandPacketReceiver>,
    query: Query<&StreamWriter>,
    mut dispatch_events: EventWriter<CommandDispatchEvent>,
    mut resolved_dispatch_events: EventWriter<ResolvedCommandDispatchEvent>,
) {
    for (event, entity) in events.0.try_iter() {
        dispatch_events.write(CommandDispatchEvent {
            command: event.command.clone(),
            sender: entity,
        });

        let resolved = resolve(event.command, entity);
        match resolved {
            Err(err) => {
                let writer = query
                    .get(entity)
                    .expect("invalid sender, this should never happen");
                if let Err(err) = writer.send_packet(&SystemMessagePacket::new(*err, false)) {
                    error!("failed sending command error to player: {err}");
                }
            }

            Ok((command, ctx)) => {
                resolved_dispatch_events.write(ResolvedCommandDispatchEvent {
                    command,
                    ctx,
                    sender: entity,
                });
            }
        }
    }
}
