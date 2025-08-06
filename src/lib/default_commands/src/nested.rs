use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_net::{
    connection::StreamWriter, packets::outgoing::system_message::SystemMessagePacket,
};
use ferrumc_text::TextComponentBuilder;
use tracing::error;
// async fn root(ctx: Arc<CommandContext>) -> CommandResult {
//     ctx.connection_id
//         .send_message(
//             TextComponentBuilder::new("Executed /nested").build(),
//             &ctx.state,
//         )
//         .await
//         .expect("failed sending message");
//     Ok(())
// }

#[command("nested")]
fn nested_command(#[sender] sender: Entity, query: Query<(&StreamWriter, &PlayerIdentity)>) {
    let (writer, identity) = query.get(sender).expect("sender has no stream writer");
    if let Err(err) = writer.send_packet(&SystemMessagePacket::new(
        TextComponentBuilder::new(format!("{} executed /nested", identity.username)).build(),
        false,
    )) {
        error!("failed sending command error to player: {err}");
    }
}

#[command("nested nested")]
fn nested_nested_command(#[sender] sender: Entity, query: Query<(&StreamWriter, &PlayerIdentity)>) {
    let (writer, identity) = query.get(sender).expect("sender has no stream writer");
    if let Err(err) = writer.send_packet(&SystemMessagePacket::new(
        TextComponentBuilder::new(format!("{} executed /nested nested", identity.username)).build(),
        false,
    )) {
        error!("failed sending command error to player: {err}");
    }
}
// #[arg("message", QuotedStringParser)]
// o
// #[arg("word", SingleStringParser)]
// #[arg("number", IntParser)]
// #[command("nested abc")]
// async fn abc(ctx: Arc<CommandContext>) -> CommandResult {
//     let message = ctx.arg::<String>("message");
//     let word = ctx.arg::<String>("word");
//     let number = ctx.arg::<i32>("number");

//     ctx.connection_id
//         .send_message(
//             TextComponentBuilder::new(format!(
//                 "Message: {message:?}, Word: {word:?}, Number: {number}"
//             ))
//             .build(),
//             &ctx.state,
//         )
//         .await
//         .expect("failed sending message");

//     Ok(())
// }
