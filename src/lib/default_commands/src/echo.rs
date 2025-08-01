use bevy_ecs::prelude::*;
use ferrumc_commands::arg::parser::string::GreedyStringParser;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_net::{
    connection::StreamWriter, packets::outgoing::system_message::SystemMessagePacket,
};
use ferrumc_text::TextComponentBuilder;
use tracing::error;

#[command("echo")]
fn test_command(
    #[parser(GreedyStringParser)] message: String,
    #[sender] sender: Entity,
    query: Query<(&StreamWriter, &PlayerIdentity)>,
) {
    let (writer, identity) = query.get(sender).expect("sender has no stream writer");
    if let Err(err) = writer.send_packet(&SystemMessagePacket::new(
        TextComponentBuilder::new(format!("{} said: ", identity.username))
            .extra(TextComponentBuilder::new(message).build())
            .build(),
        false,
    )) {
        error!("failed sending command error to player: {err}");
    }
}

// #[arg("message", GreedyStringParser::new())]
// #[command("echo")]
// async fn echo(ctx: Arc<CommandContext>) -> CommandResult {
//     let message = ctx.arg::<String>("message");
//     let identity = ctx
//         .state
//         .universe
//         .get::<PlayerIdentity>(ctx.connection_id)
//         .expect("failed to get identity");

//     ctx.connection_id
//         .send_message(
//             TextComponentBuilder::new(format!("{} said: {message}", identity.username))
//                 .color(NamedColor::Green)
//                 .build(),
//             &ctx.state,
//         )
//         .await
//         .expect("failed sending message");

//     Ok(())
// }
