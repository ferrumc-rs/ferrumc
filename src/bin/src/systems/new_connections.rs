use bevy_ecs::prelude::{Commands, MessageWriter, Res, Resource};
use crossbeam_channel::Receiver;
use ferrumc_components::{
    active_effects::ActiveEffects,
    health::Health,
    player::{
        abilities::PlayerAbilities,
        experience::Experience,
        gamemode::{GameMode, GameModeComponent},
        gameplay_state::ender_chest::EnderChest,
        hunger::Hunger,
        player_bundle::PlayerBundle,
    },
};
use ferrumc_core::{
    chunks::chunk_receiver::ChunkReceiver,
    conn::keepalive::KeepAliveTracker,
    transform::{grounded::OnGround, position::Position, rotation::Rotation},
};
use ferrumc_inventories::{hotbar::Hotbar, inventory::Inventory};
use ferrumc_messages::player_join::PlayerJoined;
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
    mut join_events: MessageWriter<PlayerJoined>,
) {
    if new_connections.0.is_empty() {
        return;
    }
    while let Ok(new_connection) = new_connections.0.try_recv() {
        let return_sender = new_connection.entity_return;

        // --- 1. Load all data from cache ---
        let (
            abilities,
            gamemode,
            position,
            rotation,
            inventory,
            health,
            hunger,
            experience,
            ender_chest,
            active_effects,
        ) = state
            .0
            .player_cache
            .get_and_remove(&new_connection.player_identity.uuid)
            .map(|data| {
                // A. Found in cache, use cached data
                (
                    data.abilities,
                    data.gamemode,
                    data.position,
                    data.rotation,
                    data.inventory,
                    data.health,
                    data.hunger,
                    data.experience,
                    data.ender_chest,
                    data.active_effects,
                )
            })
            .unwrap_or_else(|| {
                // B. Not in cache, use defaults
                (
                    PlayerAbilities::default(),
                    GameMode::default(),
                    Position::default(),
                    Rotation::default(),
                    Inventory::default(),
                    Health::default(),
                    Hunger::default(),
                    Experience::default(),
                    EnderChest::default(),
                    ActiveEffects::default(),
                )
            });

        // --- 2. Build the PlayerBundle ---
        let player_bundle = PlayerBundle {
            identity: new_connection.player_identity.clone(),
            abilities,
            gamemode: GameModeComponent(gamemode),
            position,
            rotation,
            on_ground: OnGround::default(),
            chunk_receiver: ChunkReceiver::default(),
            inventory,
            hotbar: Hotbar::default(),
            ender_chest,
            health,
            hunger,
            experience,
            active_effects,
        };

        // --- 3. Spawn the PlayerBundle, then .insert() the network components ---
        let mut entity_commands = cmd.spawn(player_bundle);

        entity_commands.insert((
            new_connection.stream,
            DisconnectHandle {
                sender: Some(new_connection.disconnect_handle),
            },
            KeepAliveTracker {
                last_sent_keep_alive: 0,
                last_received_keep_alive: Instant::now(),
                has_received_keep_alive: true,
            },
        ));

        let entity_id = entity_commands.id();

        state.0.players.player_list.insert(
            entity_id,
            (
                new_connection.player_identity.uuid.as_u128(),
                new_connection.player_identity.username.clone(),
            ),
        );

        trace!("Spawned entity for new connection: {:?}", entity_id);
        // Add the new entity to the global state
        state.0.players.player_list.insert(
            entity_id,
            (
                new_connection.player_identity.uuid.as_u128(),
                new_connection.player_identity.username.clone(),
            ),
        );

        // Fire PlayerJoinEvent
        join_events.write(PlayerJoined(new_connection.player_identity.clone()));

        if let Err(err) = return_sender.send(entity_id) {
            error!(
                "Failed to send entity ID back to the networking thread: {:?}",
                err
            );
        }
    }
}
