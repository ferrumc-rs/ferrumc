use bevy_ecs::entity::Entity;
use ferrumc_macros::command;

#[command("test")]
fn test_command(
    #[parser(GreedyStringParser)] thing: String,
    #[parser(GreedyStringParser)] things: String,
    #[sender] sender: Entity,
) {
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
