use std::sync::{Arc, Mutex};

use ferrumc_commands::{ctx::CommandContext, infrastructure::find_command, input::CommandInput};
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::ServerState;
use ferrumc_text::{NamedColor, TextComponentBuilder};

use crate::{
    connection::StreamWriter,
    packets::{outgoing::system_message::SystemMessagePacket, IncomingPacket},
    NetResult,
};

#[derive(NetDecode, Debug, Clone)]
#[packet(packet_id = 0x04, state = "play")]
pub struct ChatCommandPacket {
    command: String,
}

impl IncomingPacket for ChatCommandPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let mut writer = state.universe.get_mut::<StreamWriter>(conn_id)?;

        let command = find_command(self.command.as_str());
        if let None = command {
            // invalid command TwT, we send a message that the given command
            // could not be found.
            writer
                .send_packet(
                    &SystemMessagePacket::new(
                        TextComponentBuilder::new("unknown command")
                            .color(NamedColor::Red)
                            .build(),
                        false,
                    ),
                    &NetEncodeOpts::WithLength,
                )
                .await?;
            return Ok(());
        }

        let command = command.unwrap();

        let input = &self
            .command
            .strip_prefix(command.name)
            .unwrap_or(&self.command);
        let input = CommandInput::of(input.to_string());
        let ctx = CommandContext::new(input.clone(), command.clone(), state.clone(), conn_id);
        if let Err(err) = command.validate(&ctx, &Arc::new(Mutex::new(input))) {
            writer
                .send_packet(
                    &SystemMessagePacket::new(
                        TextComponentBuilder::new("invalid args: ")
                            .extra(err)
                            .color(NamedColor::Red)
                            .build(),
                        false,
                    ),
                    &NetEncodeOpts::WithLength,
                )
                .await?;
            return Ok(());
        }

        drop(writer); // Avoid deadlocks if the executor accesses the stream writer
        if let Err(err) = command.execute(ctx).await {
            let mut writer = state.universe.get_mut::<StreamWriter>(conn_id)?;
            writer
                .send_packet(
                    &SystemMessagePacket::new(
                        TextComponentBuilder::new("command error: ")
                            .extra(err)
                            .color(NamedColor::Red)
                            .build(),
                        false,
                    ),
                    &NetEncodeOpts::WithLength,
                )
                .await?;
        };

        Ok(())
    }
}
