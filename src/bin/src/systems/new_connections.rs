use bevy_ecs::prelude::{Commands, Res, Resource};
use crossbeam_channel::Receiver;
use ferrumc_components::player::offline_player_data::StorageOfflinePlayerData;
use ferrumc_components::player::{
    gamemode::GameModeComponent, offline_player_data::OfflinePlayerData,
    pending_events::PendingPlayerJoin, player_bundle::PlayerBundle, sneak::SneakState,
    swimming::SwimmingState,
};
use ferrumc_core::{
    chunks::chunk_receiver::ChunkReceiver, conn::keepalive::KeepAliveTracker,
    transform::grounded::OnGround,
};
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_inventories::sync::{EquipmentState, NeedsInventorySync};
use ferrumc_net::connection::{DisconnectHandle, NewConnection};
use ferrumc_state::GlobalStateResource;
use std::time::Instant;
use tracing::{debug, error, trace};

#[derive(Resource)]
pub struct NewConnectionRecv(pub Receiver<NewConnection>);

pub fn accept_new_connections(
    mut cmd: Commands,
    new_connections: Res<NewConnectionRecv>,
    state: Res<GlobalStateResource>,
) {
    if new_connections.0.is_empty() {
        return;
    }
    while let Ok(new_connection) = new_connections.0.try_recv() {
        let return_sender = new_connection.entity_return;

        // Load player data from storage, fall back to defaults for new players
        let player_data: OfflinePlayerData = state
            .0
            .world
            .load_player_data::<StorageOfflinePlayerData>(new_connection.player_identity.uuid)
            .ok()
            .flatten()
            .map(|storage| {
                debug!(
                    "Loaded player data for {} from storage",
                    new_connection.player_identity.username
                );
                OfflinePlayerData::from(storage)
            })
            .unwrap_or_else(|| {
                debug!(
                    "No saved data for {}, using defaults",
                    new_connection.player_identity.username
                );
                OfflinePlayerData::default()
            });
        // --- 2. Build the PlayerBundle ---
        let hotbar = Hotbar::default();

        // Compute initial equipment state BEFORE moving into bundle
        let initial_equipment = EquipmentState::from_inventory(&player_data.inventory, &hotbar);

        let player_bundle = PlayerBundle {
            identity: new_connection.player_identity.clone(),
            abilities: player_data.abilities,
            gamemode: GameModeComponent(player_data.gamemode),
            position: player_data.position,
            rotation: player_data.rotation,
            on_ground: OnGround::default(),
            chunk_receiver: ChunkReceiver::default(),
            inventory: player_data.inventory,
            hotbar,
            ender_chest: player_data.ender_chest,
            health: player_data.health,
            hunger: player_data.hunger,
            experience: player_data.experience,
            active_effects: player_data.active_effects,
            swimming: SwimmingState::default(),
            sneak: SneakState::default(),
        };

        // --- 3. Spawn the PlayerBundle, then .insert() the network components ---
        let mut entity_commands = cmd.spawn(player_bundle);

        // Add network components and the pending join marker.
        // The marker triggers `emit_player_joined` to fire the actual event
        // after `apply_deferred` flushes the entity into existence.
        entity_commands.insert((
            new_connection.stream,
            DisconnectHandle {
                sender: Some(new_connection.disconnect_handle),
            },
            KeepAliveTracker {
                last_sent_keep_alive_id: 0,
                last_received_keep_alive: Instant::now(),
                has_received_keep_alive: true,
                last_sent_keep_alive: Instant::now(),
            },
            PendingPlayerJoin(new_connection.player_identity.clone()),
            // Inventory sync markers
            NeedsInventorySync,
            initial_equipment,
        ));

        let entity_id = entity_commands.id();

        // Add the new player to the global player list (used for server list player count)
        state.0.players.player_list.insert(
            entity_id,
            (
                new_connection.player_identity.uuid.as_u128(),
                new_connection.player_identity.username.clone(),
            ),
        );

        trace!("Spawned entity for new connection: {:?}", entity_id);

        if let Err(err) = return_sender.send(entity_id) {
            error!(
                "Failed to send entity ID back to the networking thread: {:?}",
                err
            );
        }
    }
}
