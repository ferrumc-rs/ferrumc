use std::sync::Arc;

use ferrumc_commands::{
    arg::parser::string::GreedyStringParser, arg::CommandArgument, ctx::CommandContext, executor,
    infrastructure::register_command, Command, CommandResult,
};
use ferrumc_macros::{arg, command};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::system_message::SystemMessagePacket;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_text::{NamedColor, TextComponentBuilder};
use tracing::error;

#[arg("message", GreedyStringParser)]
#[command("echo")]
async fn echo(ctx: Arc<CommandContext>) -> CommandResult {
    let message = ctx.arg::<String>("message");
    let mut writer = match ctx
        .state
        .universe
        .get_mut::<StreamWriter>(ctx.connection_id)
    {
        Ok(writer) => writer,
        Err(err) => {
            error!(
                "failed retrieving stream writer for conn id {}: {err}",
                ctx.connection_id
            );
            return Ok(());
        }
    };

    if let Err(err) = writer
        .send_packet(
            &SystemMessagePacket::new(
                TextComponentBuilder::new(message)
                    .color(NamedColor::Green)
                    .build(),
                false,
            ),
            &NetEncodeOpts::WithLength,
        )
        .await
    {
        error!("failed sending packet: {err}");
        return Ok(());
    }

    Ok(())
}
