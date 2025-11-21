use bevy_ecs::{
    event::EventReader,
    prelude::{Query, Res},
    system::Commands,
};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::{grounded::OnGround, position::Position, rotation::Rotation};
use ferrumc_core::{
    conn::player_disconnect_event::PlayerDisconnectEvent, player::gamemode::GameModeComponent,
};
use ferrumc_state::GlobalStateResource;

pub fn handle(
    mut cmd: Commands,
    mut events: EventReader<PlayerDisconnectEvent>,
    state: Res<GlobalStateResource>,
    query: Query<(
        &PlayerIdentity,
        &Position,
        &OnGround,
        &Rotation,
        &GameModeComponent,
    )>,
) {
    for event in events.read() {
        if let Ok((identity, position, on_ground, rotation, game_mode)) = query.get(event.entity) {
            let uuid = identity.uuid.as_u128();
            let username = &identity.username;

            let player_data = ferrumc_core::data::player::PlayerData {
                pos: position.into(),
                on_ground: on_ground.0,
                dimension: "overworld".to_string(),
                rotation: (rotation.yaw, rotation.pitch),
                player_game_type: game_mode.0 as u8,
            };

            if let Err(e) = state.0.world.save_player_state(uuid, &player_data) {
                tracing::error!("Failed to save player state for {}: {}", username, e);
            } else {
                tracing::info!("Player state saved for {}", username);
            }
        }

        cmd.entity(event.entity).despawn();
    }
}
