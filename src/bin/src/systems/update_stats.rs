use bevy_ecs::prelude::Res;
use ferrumc_state::GlobalStateResource;
use std::sync::atomic::Ordering::Relaxed;

pub fn update_stats(state: Res<GlobalStateResource>) {
    let stats = &state.0.stats;

    let mut sys = stats.system.write().unwrap();

    sys.refresh_all();

    stats.player_count.store(state.0.players.player_list.len() as u32, Relaxed);

    let uptime = state.0.start_time.elapsed().as_secs();
    stats.uptime.store(uptime, Relaxed);

    stats.memory_usage.store(sys.used_memory(), Relaxed);

    stats.cpu_usage.store(sys.global_cpu_usage(), Relaxed);

    stats.cores.store(sys.cpus().len() as u32, Relaxed);
}