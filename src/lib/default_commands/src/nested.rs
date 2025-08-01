// #[command("nested")]
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

// #[arg("message", QuotedStringParser)]
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
