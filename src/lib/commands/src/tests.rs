use std::sync::Arc;

use ferrumc_ecs::Universe;
use ferrumc_macros::Command;
use ferrumc_state::{GlobalState, ServerState};
use ferrumc_text::{TextComponentBuilder, TextContent};
use ferrumc_world::World;
use tokio::net::TcpListener;

use crate::{
    arg::{
        parser::{
            int::IntParser,
            string::{GreedyStringParser, QuotedStringParser},
        },
        CommandArgument,
    },
    ctx::CommandContext,
    executor,
    infrastructure::{find_command, register_command},
    input::CommandInput,
    CommandResult,
};

async fn state() -> GlobalState {
    Arc::new(ServerState {
        universe: Universe::new(),
        tcp_listener: TcpListener::bind("0.0.0.0:0").await.unwrap(),
        world: World::new().await,
    })
}

#[tokio::test]
async fn arg_parse_test() {
    async fn test_executor(ctx: Arc<CommandContext>) -> CommandResult {
        let quoted = ctx.arg::<String>("quoted");
        let greedy = ctx.arg::<String>("greedy");

        Ok(TextComponentBuilder::new(format!("{quoted:?} {greedy}")).build())
    }

    let command = crate::Command {
        name: "input_test",
        args: vec![
            CommandArgument {
                name: "quoted".to_string(),
                required: true,
                parser: Box::new(QuotedStringParser),
            },
            CommandArgument {
                name: "greedy".to_string(),
                required: true,
                parser: Box::new(GreedyStringParser),
            },
        ],
        executor: executor(test_executor),
    };
    let command = Arc::new(command);

    let state = state().await;

    let input = "\"hello\" no no no please no I'm so sorry";

    let ctx = CommandContext::new(CommandInput::of(input.to_string()), command.clone(), state);

    let result = command.execute(ctx).await;
    let TextContent::Text { text } = result.unwrap().content else {
        panic!("result is not text")
    };

    assert_eq!(text, input);
}

#[tokio::test]
async fn parse_test() {
    async fn test_executor(ctx: Arc<CommandContext>) -> CommandResult {
        let num = ctx.arg::<u32>("number");
        Ok(TextComponentBuilder::new(num.to_string()).build())
    }

    let command = crate::Command {
        name: "input_test",
        args: vec![CommandArgument {
            name: "number".to_string(),
            required: true,
            parser: Box::new(IntParser),
        }],
        executor: executor(test_executor),
    };
    let command = Arc::new(command);

    let state = state().await;

    let ctx = CommandContext::new(CommandInput::of("42".to_string()), command.clone(), state);

    register_command(command.clone());

    let found_command = find_command("input_test 42").unwrap();

    let result = found_command.execute(ctx).await;
    let TextContent::Text { text } = result.unwrap().content else {
        panic!("result is not text")
    };

    assert_eq!(text, "42".to_string());
}

#[derive(Command)]
#[command(test)]
struct TestCommand {
    #[sender]
    sender: String,
    message: GreedyStringParser,
}