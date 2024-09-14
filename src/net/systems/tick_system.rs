use async_trait::async_trait;

use crate::net::packets::outgoing::login_plugin_request::LoginPluginRequest;
use crate::net::systems::System;
use crate::net::ConnectionWrapper;
use crate::state::GlobalState;
use crate::utils::components::player::Player;
use ferrumc_macros::AutoGenName;
use tracing::warn;

#[derive(AutoGenName)]
pub struct TickSystem;

#[async_trait]
impl System for TickSystem {
    async fn run(&self, state: GlobalState) {
        let mut query = state.world.query::<(&ConnectionWrapper, &Player)>();

        let width = 40;
        let total_width = width * 2;
        let mut offset = 0;

        loop {
            let mut crab_wave = vec![" "; total_width];

            for (index, wave) in crab_wave.iter_mut().enumerate().take(total_width) {
                let wave_height = ((index as f64 * 0.2).sin() + 1.0) * 2.0;
                if wave_height.round() as usize == 2 {
                    *wave = "🦀";
                }
            }

            let visible_wave: String = crab_wave
                .iter()
                .cycle()
                .skip(offset)
                .take(width)
                .cloned()
                .collect();

            while let Some((_, (conn, _))) = query.next().await {
                let packet = LoginPluginRequest::server_brand(&visible_wave).await;
                let conn = conn.0.read().await;
                if let Err(e) = conn.send_packet(packet).await {
                    warn!("Failed to send packet: {}", e);
                    continue;
                }
                // trace!("Ticked connection for player `{}`", player.get_username());
            }

            offset = (offset + 1) % total_width;

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}
