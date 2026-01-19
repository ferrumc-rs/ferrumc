#![expect(clippy::type_complexity)]
use bevy_ecs::prelude::{Query, Res, ResMut};
use ferrumc_components::active_effects::ActiveEffects;
use ferrumc_components::health::Health;
use ferrumc_components::player::abilities::PlayerAbilities;
use ferrumc_components::player::experience::Experience;
use ferrumc_components::player::gamemode::GameModeComponent;
use ferrumc_components::player::gameplay_state::ender_chest::EnderChest;
use ferrumc_components::player::hunger::Hunger;
use ferrumc_components::player::offline_player_data::OfflinePlayerData;
use ferrumc_core::chunks::world_sync_tracker::WorldSyncTracker;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_state::GlobalStateResource;

pub fn sync_world(
    player_query: Query<(
        &PlayerIdentity,
        &PlayerAbilities,
        &GameModeComponent,
        &Position,
        &Rotation,
        &Inventory,
        &Health,
        &Hunger,
        &Experience,
        &EnderChest,
        &ActiveEffects,
    )>,
    state: Res<GlobalStateResource>,
    mut last_synced: ResMut<WorldSyncTracker>,
) {
    if state.0.shut_down.load(std::sync::atomic::Ordering::Relaxed) {
        return;
    }

    // Always schedule a sync; frequency is handled by the schedule period.
    state.0.world.sync().expect("Failed to sync world");

    for (
        identity,
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
    ) in player_query.iter()
    {
        let data = OfflinePlayerData {
            abilities: *abilities,
            gamemode: gamemode.0,
            position: (*position).into(),
            rotation: *rotation,
            inventory: inventory.clone(),
            health: *health,
            hunger: *hunger,
            experience: *experience,
            ender_chest: ender_chest.clone(),
            active_effects: active_effects.clone(),
        };
        state
            .0
            .world
            .save_player_data(identity.uuid, &data)
            .expect("Failed to save player data");
    }

    last_synced.last_synced = std::time::Instant::now();
}
