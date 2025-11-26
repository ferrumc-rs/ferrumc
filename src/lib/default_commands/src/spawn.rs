use ferrumc_commands::{
    arg::{
        primitive::PrimitiveArgument,
        utils::parser_error,
        CommandArgument,
        ParserResult,
    },
    CommandContext,
    Sender,
    Suggestion,
};
use ferrumc_entities::{request_spawn, EntityType};
use ferrumc_macros::command;
use ferrumc_text::TextComponent;

/// Wrapper type for EntityType that implements CommandArgument
#[derive(Debug, Clone, Copy)]
struct EntityTypeArg(EntityType);

impl CommandArgument for EntityTypeArg {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let str = ctx.input.read_string();

        let value = match &*str.to_lowercase() {
            "pig" => EntityType::Pig,
            // Add more entity types here as they're implemented
            // "cow" => EntityType::Cow,
            // "sheep" => EntityType::Sheep,
            _ => {
                return Err(parser_error(&format!(
                    "Unknown entity type: '{}'. Currently supported: pig",
                    str
                )))
            }
        };

        Ok(EntityTypeArg(value))
    }

    fn primitive() -> PrimitiveArgument {
        // We're parsing a single word
        PrimitiveArgument::word()
    }

    fn suggest(ctx: &mut CommandContext) -> Vec<Suggestion> {
        ctx.input.read_string();

        // Only suggest "pig" for now - add more as they're implemented
        vec![Suggestion::of("pig")]
    }
}

/// Spawns an entity in front of the player.
///
/// Usage: /spawn <entity_type>
/// Currently supported: pig
#[command("spawn")]
fn spawn_command(#[sender] sender: Sender, #[arg] entity_type: EntityTypeArg) {
    match sender {
        Sender::Player(entity) => {
            // Add spawn request to global queue - will be processed by spawn_command_processor system
            request_spawn(entity_type.0, entity);

            // Get entity name for message
            let entity_name = match entity_type.0 {
                EntityType::Pig => "Pig",
            };

            sender.send_message(
                TextComponent::from(format!("{} spawned!", entity_name)),
                false,
            );
        }
        Sender::Server => {
            sender.send_message(
                TextComponent::from("Only players can use this command"),
                false,
            );
        }
    }
}
