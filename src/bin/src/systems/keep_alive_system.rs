use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::{ConnectionState, StreamWriter};
use ferrumc_net::packets::incoming::keep_alive;
use ferrumc_net::packets::outgoing::keep_alive::{KeepAlive, KeepAlivePacket};
use ferrumc_net::GlobalState;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use futures::StreamExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info};

pub struct KeepAliveSystem {
    shutdown: AtomicBool,
}

impl KeepAliveSystem {
    pub const fn new() -> Self {
        Self {
            shutdown: AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl System for KeepAliveSystem {
    async fn start(self: Arc<Self>, state: GlobalState) {
        loop {
            if self.shutdown.load(Ordering::Relaxed) {
                break;
            }

            let online_players = state.universe.query::<&PlayerIdentity>();
            info!("Online players: {}", online_players.count());

            tokio::time::sleep(Duration::from_secs(5)).await;

            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as i64;

            let fifteen_seconds_ms = 15000; // 15 seconds in milliseconds

            let query = state
                .universe
                .query::<(&mut StreamWriter, &ConnectionState, &KeepAlive)>()
                .into_entities()
                .into_iter()
                .filter_map(|entity| {
                    let conn_state = state.universe.get::<ConnectionState>(entity).ok()?;
                    let keep_alive = state.universe.get_mut::<KeepAlive>(entity).ok()?;

                    if matches!(*conn_state, ConnectionState::Play)
                        && (current_time - keep_alive.id) >= fifteen_seconds_ms
                    {
                        Some(entity)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if query.len() > 0 {
                info!("there are {:?} players to keep alive", query.len());
            }

            let packet = KeepAlivePacket::default();
            let packet = {
                let mut buffer = Vec::new();
                if packet
                    .encode(&mut buffer, &NetEncodeOpts::WithLength)
                    .is_err()
                {
                    error!("Error encoding keep alive packet");
                }
                buffer
            };
            tokio::spawn(futures::stream::iter(query.into_iter()).fold(
                (state.clone(), packet),
                move |(state, packet), entity| async move {
                    if let Ok(mut writer) = state.universe.get_mut::<StreamWriter>(entity) {
                        if let Err(e) = writer
                            .send_packet(&packet.as_slice(), &NetEncodeOpts::None)
                            .await
                        {
                            error!("Error sending update_time packet: {}", e);
                        }
                    }
                    debug!("Sent keep alive packet to {}", entity);
                    let mut keep_alive = state.universe.get_mut::<KeepAlive>(entity).unwrap();
                    *keep_alive = KeepAlive::from(current_time);
                    (state, packet)
                }
            ));
        }
    }

    async fn stop(self: Arc<Self>, _state: GlobalState) {
        tracing::debug!("Stopping keep alive system...");
        self.shutdown.store(true, Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "keep_alive"
    }
}
