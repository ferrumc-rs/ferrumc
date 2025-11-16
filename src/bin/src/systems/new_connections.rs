use crate::events::player_join::PlayerJoinEvent;
use bevy_ecs::{
    event::EventWriter,
    prelude::{Commands, Res, Resource},
};
use crossbeam_channel::Receiver;

use ferrumc_components::player::gamemode::GameModeComponent;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::conn::keepalive::KeepAliveTracker;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_net::connection::{DisconnectHandle, NewConnection};
use ferrumc_state::GlobalStateResource;
use std::time::Instant;
use tracing::{error, trace};

#[derive(Resource)]
pub struct NewConnectionRecv(pub Receiver<NewConnection>);

pub fn accept_new_connections(
    mut cmd: Commands,
    new_connections: Res<NewConnectionRecv>,
    state: Res<GlobalStateResource>,
    mut join_events: EventWriter<PlayerJoinEvent>,
) {
    if new_connections.0.is_empty() {
        return;
    }
    while let Ok(new_connection) = new_connections.0.try_recv() {
        let return_sender = new_connection.entity_return;

        // --- Load abilities from cache, or use defaults ---
        // This removes the entry from the cache, ensuring it's not used again
        // until the player logs out
        let (abilities, gamemode) = state
            .0
            .player_cache
            .get_and_remove(&new_connection.player_identity.uuid)
            .map(|data| (data.abilities, data.gamemode)) // Extract both
            .unwrap_or_default();

        let entity = cmd.spawn((
            new_connection.stream,
            DisconnectHandle {
                sender: Some(new_connection.disconnect_handle),
            },
            Position::default(),
            ChunkReceiver::default(),
            Rotation::default(),
            OnGround::default(),
            new_connection.player_identity.clone(),
            KeepAliveTracker {
                last_sent_keep_alive: 0,
                last_received_keep_alive: Instant::now(),
                has_received_keep_alive: true,
            },
            Inventory::default(),
            Hotbar::default(),
            abilities,
            GameModeComponent(gamemode),
        ));

        state.0.players.player_list.insert(
            entity.id(),
            (
                new_connection.player_identity.uuid.as_u128(),
                new_connection.player_identity.username.clone(),
            ),
        );

        trace!("Spawned entity for new connection: {:?}", entity.id());
        // Add the new entity to the global state
        state.0.players.player_list.insert(
            entity.id(),
            (
                new_connection.player_identity.uuid.as_u128(),
                new_connection.player_identity.username.clone(),
            ),
        );

        // Fire PlayerJoinEvent
        join_events.write(PlayerJoinEvent(new_connection.player_identity.clone()));

        if let Err(err) = return_sender.send(entity.id()) {
            error!(
                "Failed to send entity ID back to the networking thread: {:?}",
                err
            );
        }
    }
}
