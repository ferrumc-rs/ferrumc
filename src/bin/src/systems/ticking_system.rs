use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_events::infrastructure::Event;
use ferrumc_net::connection::{ConnectionState, StreamWriter};
use ferrumc_net::packets::outgoing::keep_alive::{KeepAlive, KeepAlivePacket};
use ferrumc_net::packets::outgoing::update_time::TickEvent;
use ferrumc_net::GlobalState;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use futures::StreamExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use tracing::{debug, error, info};
pub struct TickingSystem;

static KILLED: AtomicBool = AtomicBool::new(false);

#[async_trait]
impl System for TickingSystem {
    async fn start(self: Arc<Self>, state: GlobalState) {
        // TODO game time must be loaded from a file
        let mut tick = 0;
        while !KILLED.load(Ordering::Relaxed) {
            let required_end = Instant::now() + Duration::from_millis(50);
            // TODO handle error
            let res = TickEvent::trigger(TickEvent::new(tick), state.clone()).await;

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
                        && (current_time - keep_alive.id) > fifteen_seconds_ms
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
                    (state, packet)
                },
            ));

            if res.is_err() {
                debug!("error handling tick event: {:?}", res);
            }
            let now = Instant::now();
            if required_end > now {
                tokio::time::sleep(required_end - now).await;
            } else {
                let time_debt = now - required_end;
                info!("Running behind by {:?}", time_debt);
            }

            tick += 1;
        }
    }

    async fn stop(self: Arc<Self>, _state: GlobalState) {
        debug!("Stopping ticking system...");
        KILLED.store(true, Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "ticking"
    }
}
