use bevy_ecs::prelude::{Res, ResMut};
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::sys::stats_cooldown::StatsCooldown;
use ferrumc_state::GlobalStateResource;
use std::sync::atomic::Ordering::Relaxed;

pub fn update_stats(state: Res<GlobalStateResource>, mut cooldown: ResMut<StatsCooldown>) {
    let stats = &state.0.stats;

    if cooldown.last_update.elapsed().as_secs() < get_global_config().system_stats_cooldown as u64 {
        return; // Skip if cooldown has not elapsed
    }


    let mut sys = stats.system.lock();

    sys.refresh_all();

    stats.player_count.store(state.0.players.player_list.len() as u32, Relaxed);

    let uptime = state.0.start_time.elapsed().as_secs();
    stats.uptime.store(uptime, Relaxed);

    stats.memory_usage.store(sys.used_memory(), Relaxed);

    stats.cpu_usage.store(sys.global_cpu_usage(), Relaxed);

    stats.cores.store(sys.cpus().len() as u32, Relaxed);

    cooldown.last_update = std::time::Instant::now();
}