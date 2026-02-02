use std::sync::Arc;

use bevy_ecs::prelude::*;
use ferrumc_commands::{
    infrastructure,
    messages::{CommandDispatched, ResolvedCommandDispatched},
    Command, CommandContext, CommandInput, Sender,
};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::mq;
use ferrumc_net::ChatCommandPacketReceiver;
use ferrumc_state::{GlobalState, GlobalStateResource};
use ferrumc_text::{NamedColor, TextComponent, TextComponentBuilder};
use tracing::info;

pub fn resolve(
    input: String,
    sender: Sender,
    state: GlobalState,
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
        state,
    };

    Ok((command, ctx))
}

pub fn handle(
    receiver: Res<ChatCommandPacketReceiver>,
    mut dispatch_msgs: MessageWriter<CommandDispatched>,
    mut resolved_dispatch_msgs: MessageWriter<ResolvedCommandDispatched>,
    state: Res<GlobalStateResource>,
    query: Query<&PlayerIdentity>,
) {
    for (event, entity) in receiver.0.try_iter() {
        let sender = Sender::Player(entity);
        dispatch_msgs.write(CommandDispatched {
            command: event.command.clone(),
            sender,
        });

        let resolved = resolve(event.command.clone(), sender, state.0.clone());
        match resolved {
            Err(err) => {
                mq::queue(*err, false, entity);
            }

            Ok((command, ctx)) => {
                let Ok(player_id) = query.get(entity) else {
                    continue;
                };
                info!(
                    "Player {} executed command: /{}",
                    player_id.username, event.command
                );
                resolved_dispatch_msgs.write(ResolvedCommandDispatched {
                    command,
                    ctx,
                    sender,
                });
            }
        }
    }
}
