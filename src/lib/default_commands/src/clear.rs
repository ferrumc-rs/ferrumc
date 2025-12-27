use bevy_ecs::message::MessageWriter;
use ferrumc_commands::Sender;
use ferrumc_macros::command;
use ferrumc_messages::ClearPlayerInventory;

#[command("clear")]
fn tps_command(
    #[sender] sender: Sender,
    mut clear_inventory: MessageWriter<ClearPlayerInventory>
) {
    // 1. Ensure the sender is a player
    let player_entity = match sender {
        Sender::Server => {
            sender.send_message("Error: cannot change gamemode of server.".into(), false);
            return;
        }
        Sender::Player(entity) => entity,
    };
    let _ = clear_inventory.write(ClearPlayerInventory {
        player: player_entity,
    });
    dbg!();
}