use crate::connection::StreamWriter;
use crate::NetResult;
use async_trait::async_trait;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_ecs::entities::Entity;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_state::GlobalState;
use futures::StreamExt;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use tracing::debug;

type AsyncCallbackFn = Box<
    dyn Fn(Entity, &GlobalState) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> + Send + Sync,
>;
type SyncCallbackFn = Box<dyn Fn(Entity, &GlobalState) + Send + Sync>;

#[derive(Default)]
pub struct BroadcastOptions {
    pub only_entities: Option<HashSet<Entity>>,
    pub except_entities: Option<HashSet<Entity>>,
    pub async_callback: Option<AsyncCallbackFn>,
    pub sync_callback: Option<SyncCallbackFn>,
}

impl BroadcastOptions {
    pub fn only<I>(mut self, entities: I) -> Self
    where
        I: IntoIterator<Item = Entity>,
    {
        self.only_entities = Some(entities.into_iter().collect());
        self
    }

    pub fn except<I>(mut self, entities: I) -> Self
    where
        I: IntoIterator<Item = Entity>,
    {
        self.except_entities = Some(entities.into_iter().collect());
        self
    }

    pub fn all(mut self) -> Self {
        self.only_entities = None;
        self
    }

    pub fn with_async_callback<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(Entity, &GlobalState) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.async_callback = Some(Box::new(move |entity, state| Box::pin(f(entity, state))));
        self
    }

    pub fn with_sync_callback<F>(mut self, f: F) -> Self
    where
        F: Fn(Entity, &GlobalState) + Send + Sync + 'static,
    {
        self.sync_callback = Some(Box::new(f));
        self
    }
}

/// Get all players in the 'play' state, so the players playing the playable game.
pub fn get_all_play_players(state: &GlobalState) -> HashSet<Entity> {
    // If it needs a chunk, then it's player!! :)
    // !!!= === =.>>> if it works dont break it
    state
        .universe
        .get_component_manager()
        .get_entities_with::<ChunkReceiver>()
        .into_iter()
        .collect()
}

pub async fn broadcast(
    packet: &impl NetEncode,
    state: &GlobalState,
    opts: BroadcastOptions,
) -> NetResult<()> {
    let mut entities = match opts.only_entities {
        None => get_all_play_players(state),
        Some(entities) => entities,
    };

    // Remove excluded entities if any
    if let Some(except_entities) = opts.except_entities {
        entities.retain(|entity| !except_entities.contains(entity));
    }

    // No entities to broadcast to
    if entities.is_empty() {
        return Ok(());
    }

    // Pre-encode the packet to save resources.
    let packet = {
        let mut buffer = Vec::new();
        packet.encode(&mut buffer, &NetEncodeOpts::WithLength)?;

        buffer
    };

    let (state, packet, async_callback, sync_callback) =
        (state, packet, opts.async_callback, opts.sync_callback);

    futures::stream::iter(entities.into_iter())
        .fold(
            (state, packet, async_callback, sync_callback),
            move |(state, packet, async_callback, sync_callback), entity| {
                async move {
                    let Ok(mut writer) = state.universe.get_mut::<StreamWriter>(entity) else {
                        return (state, packet, async_callback, sync_callback);
                    };

                    if let Err(e) = writer.send_packet(&packet, &NetEncodeOpts::None).await {
                        debug!("Error sending packet: {}", e);
                    }

                    // Execute sync callback first if it exists
                    if let Some(ref callback) = sync_callback {
                        callback(entity, state);
                    }

                    // Then execute async callback if it exists
                    if let Some(ref callback) = async_callback {
                        callback(entity, state).await;
                    }

                    (state, packet, async_callback, sync_callback)
                }
            },
        )
        .await;

    Ok(())
}

#[async_trait]
pub trait BroadcastToAll {
    async fn broadcast(
        &self,
        packet: &(impl NetEncode + Sync),
        opts: BroadcastOptions,
    ) -> NetResult<()>;
}

#[async_trait]
impl BroadcastToAll for GlobalState {
    async fn broadcast(
        &self,
        packet: &(impl NetEncode + Sync),
        opts: BroadcastOptions,
    ) -> NetResult<()> {
        broadcast(packet, self, opts).await
    }
}
