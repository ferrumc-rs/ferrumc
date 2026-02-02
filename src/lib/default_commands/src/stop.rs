use bevy_ecs::prelude::Res;
use ferrumc_commands::Sender;
use ferrumc_macros::command;
use ferrumc_state::GlobalStateResource;
use std::sync::atomic::Ordering::Relaxed;

#[command("stop")]
fn stop_command(#[sender] sender: Sender, state: Res<GlobalStateResource>) {
    if !matches!(sender, Sender::Server) {
        sender.send_message("This command can only be used by the server.".into(), false);
        return;
    }
    sender.send_message("Shutting down server...".into(), false);
    state.0.shut_down.store(true, Relaxed);
}
