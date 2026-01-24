use bevy_ecs::prelude::MessageWriter;
use bimap::BiMap;
use ferrumc_commands::{
    arg::{primitive::PrimitiveArgument, utils::parser_error, CommandArgument, ParserResult},
    CommandContext, Sender, Suggestion,
};
use ferrumc_macros::command;
use ferrumc_messages::{EntityType, SpawnEntityCommand};
use ferrumc_text::TextComponent;
use lazy_static::lazy_static;

/// Wrapper type for EntityType that implements CommandArgument
#[derive(Debug, Clone, Copy)]
struct EntityTypeArg(EntityType);

lazy_static! {
    static ref MAPPED_ENTITIES: BiMap<&'static str, EntityType> = {
        let mut m = BiMap::new();

        // Add supported entities here
        m.insert("allay", EntityType::Allay);
        m.insert("armadillo", EntityType::Armadillo);
        m.insert("axolotl", EntityType::Axolotl);
        m.insert("bat", EntityType::Bat);
        m.insert("bee", EntityType::Bee);
        m.insert("camel", EntityType::Camel);
        m.insert("cat", EntityType::Cat);
        m.insert("chicken", EntityType::Chicken);
        m.insert("cod", EntityType::Cod);
        m.insert("cow", EntityType::Cow);
        m.insert("dolphin", EntityType::Dolphin);
        m.insert("donkey", EntityType::Donkey);
        m.insert("fox", EntityType::Fox);
        m.insert("frog", EntityType::Frog);
        m.insert("goat", EntityType::Goat);
        m.insert("horse", EntityType::Horse);
        m.insert("llama", EntityType::Llama);
        m.insert("mooshroom", EntityType::Mooshroom);
        m.insert("ocelot", EntityType::Ocelot);
        m.insert("panda", EntityType::Panda);
        m.insert("parrot", EntityType::Parrot);
        m.insert("pig", EntityType::Pig);
        m.insert("polar_bear", EntityType::PolarBear);
        m.insert("pufferfish", EntityType::Pufferfish);
        m.insert("rabbit", EntityType::Rabbit);
        m.insert("salmon", EntityType::Salmon);
        m.insert("sheep", EntityType::Sheep);
        m.insert("sniffer", EntityType::Sniffer);
        m.insert("squid", EntityType::Squid);
        m.insert("strider", EntityType::Strider);
        m.insert("tadpole", EntityType::Tadpole);
        m.insert("tropical_fish", EntityType::TropicalFish);
        m.insert("turtle", EntityType::Turtle);
        m.insert("wolf", EntityType::Wolf);
        m.insert("glow_squid", EntityType::GlowSquid);
        m.insert("mule", EntityType::Mule);

        m
    };
}

impl CommandArgument for EntityTypeArg {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        let str = ctx.input.read_string();

        let value = match MAPPED_ENTITIES.get_by_left(str.as_str()) {
            Some(&entity_type) => entity_type,
            None => {
                return Err(parser_error(
                    format!("Unknown entity type: {}", str).as_str(),
                ))
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

        MAPPED_ENTITIES
            .iter()
            .map(|(&name, _)| Suggestion::of(name))
            .collect()
    }
}

/// Spawns an entity in front of the player.
///
/// Usage: /spawn <entity_type>
/// Supported: allay, armadillo, axolotl, bat, bee, camel, cat, chicken, cod, cow, dolphin, donkey, fox, frog, goat, horse, llama, mooshroom, ocelot, panda, parrot, pig
#[command("spawn")]
fn spawn_command(
    #[sender] sender: Sender,
    #[arg] entity_type: EntityTypeArg,
    mut spawn_commands: MessageWriter<SpawnEntityCommand>,
) {
    match sender {
        Sender::Player(entity) => {
            // Write spawn command message - will be processed by spawn_command_processor system
            spawn_commands.write(SpawnEntityCommand {
                entity_type: entity_type.0,
                player_entity: entity,
            });

            // Get entity name for message
            let entity_name = MAPPED_ENTITIES
                .get_by_right(&entity_type.0)
                .unwrap_or(&"unknown");

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
