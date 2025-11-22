use ferrumc_commands::Sender;
use ferrumc_entities::{request_spawn, EntityType};
use ferrumc_macros::command;
use ferrumc_text::TextComponent;

/// Spawns a pig in front of the player.
#[command("spawnpig")]
fn spawnpig_command(#[sender] sender: Sender) {
    match sender {
        Sender::Player(entity) => {
            // Add spawn request to global queue - will be processed by spawn_command_processor system
            request_spawn(EntityType::Pig, entity);

            sender.send_message(TextComponent::from("Pig spawned!"), false);
        }
        Sender::Server => {
            sender.send_message(
                TextComponent::from("Only players can use this command"),
                false,
            );
        }
    }
}
