use std::sync::Arc;

use ferrumc_ecs::Universe;
use ferrumc_macros::{arg, command};
use ferrumc_state::{GlobalState, ServerState};
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
    graph::{node::CommandNodeType, CommandGraph},
    infrastructure::{find_command, register_command},
    input::CommandInput,
    Command, CommandResult,
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

        assert_eq!(
            format!("{quoted:?} {greedy}"),
            ctx.input.lock().unwrap().input
        );

        Ok(())
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

    let ctx = CommandContext::new(
        CommandInput::of(input.to_string()),
        command.clone(),
        state,
        0,
    );

    command.execute(ctx).await.unwrap();
}

#[tokio::test]
async fn parse_test() {
    async fn test_executor(ctx: Arc<CommandContext>) -> CommandResult {
        let num = ctx.arg::<u32>("number");
        assert_eq!(num.to_string(), ctx.input.lock().unwrap().input);
        Ok(())
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

    let ctx = CommandContext::new(
        CommandInput::of("42".to_string()),
        command.clone(),
        state,
        0,
    );

    register_command(command.clone());

    let found_command = find_command("input_test 42").unwrap();

    found_command.execute(ctx).await.unwrap();
}

#[arg("quoted", QuotedStringParser)]
#[command("test")]
async fn execute_test_command(_ctx: Arc<CommandContext>) -> CommandResult {
    Ok(())
}

#[tokio::test]
async fn macro_test() {
    let found_command = find_command("test").unwrap();
    assert_eq!(found_command.args.len(), 1);
}

#[tokio::test]
async fn graph_test() {
    let command = find_command("test").unwrap();
    let mut graph = CommandGraph::default();
    graph.push(command);

    for node in &graph.nodes {
        println!("{node:#?}");
    }

    assert_eq!(&graph.nodes.len(), &3);

    let literal_node = graph.nodes.get(1).unwrap();
    let arg_node = graph.nodes.get(2).unwrap();

    assert_eq!(literal_node.node_type(), CommandNodeType::Literal);
    assert_eq!(arg_node.node_type(), CommandNodeType::Argument);
    assert!(!arg_node.is_executable());
    assert!(literal_node.is_executable());
}
