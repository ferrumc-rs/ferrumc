use std::sync::{Arc, Mutex};

use ferrumc_commands::{ctx::CommandContext, infrastructure::find_command, input::CommandInput};
use ferrumc_macros::event_handler;
use ferrumc_net::{
    connection::StreamWriter,
    errors::NetError,
    packets::{
        incoming::command::CommandDispatchEvent, outgoing::system_message::SystemMessagePacket,
    },
};
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::GlobalState;
use ferrumc_text::{NamedColor, TextComponentBuilder};

#[event_handler]
async fn handle_command_dispatch(
    event: CommandDispatchEvent,
    state: GlobalState,
) -> Result<CommandDispatchEvent, NetError> {
    let mut writer = state.universe.get_mut::<StreamWriter>(event.conn_id)?;

    let command = find_command(event.command.as_str());
    if command.is_none() {
        writer
            .send_packet(
                &SystemMessagePacket::new(
                    TextComponentBuilder::new("Unknown command")
                        .color(NamedColor::Red)
                        .build(),
                    false,
                ),
                &NetEncodeOpts::WithLength,
            )
            .await?;
        return Ok(event);
    }

    let command = command.unwrap();

    let input = &event
        .command
        .strip_prefix(command.name)
        .unwrap_or(&event.command)
        .trim_start();
    let input = CommandInput::of(input.to_string());
    let ctx = CommandContext::new(input.clone(), command.clone(), state.clone(), event.conn_id);
    if let Err(err) = command.validate(&ctx, &Arc::new(Mutex::new(input))) {
        writer
            .send_packet(
                &SystemMessagePacket::new(
                    TextComponentBuilder::new("Invalid arguments: ")
                        .extra(err)
                        .color(NamedColor::Red)
                        .build(),
                    false,
                ),
                &NetEncodeOpts::WithLength,
            )
            .await?;
        return Ok(event);
    }

    drop(writer); // Avoid deadlocks if the executor accesses the stream writer
    if let Err(err) = command.execute(ctx).await {
        let mut writer = state.universe.get_mut::<StreamWriter>(event.conn_id)?;
        writer
            .send_packet(
                &SystemMessagePacket::new(
                    TextComponentBuilder::new("Failed executing command: ")
                        .extra(err)
                        .color(NamedColor::Red)
                        .build(),
                    false,
                ),
                &NetEncodeOpts::WithLength,
            )
            .await?;
    };

    Ok(event)
}
