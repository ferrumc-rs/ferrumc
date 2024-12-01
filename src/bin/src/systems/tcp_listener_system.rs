use std::sync::Arc;
use async_trait::async_trait;
use tracing::{debug, error, info, info_span, Instrument};
use ferrumc_net::connection::handle_connection;
use ferrumc_state::GlobalState;
use crate::systems::definition::System;
use crate::Result;

pub struct TcpListenerSystem;

#[async_trait]
impl System for TcpListenerSystem {
    async fn start(self: Arc<Self>, state: GlobalState) {
        if let Err(e) = TcpListenerSystem::initiate_loop(state).await {
            error!("TCP listener system failed with error: {:?}", e);
        }
    }

    async fn stop(self: Arc<Self>, _state: GlobalState) {
        debug!("Stopping TCP listener system...");
    }

    fn name(&self) -> &'static str {
        "tcp"
    }
}

impl TcpListenerSystem {
    async fn initiate_loop(state: GlobalState) -> Result<()> {
        let tcp_listener = &state.tcp_listener;
        info!("Server is listening on [{}]", tcp_listener.local_addr()?);
        
        loop {
            debug!("Accepting connection");
            let (stream, _) = tcp_listener.accept().await?;
            let addy = stream.peer_addr()?;
            tokio::task::spawn(
                handle_connection(Arc::clone(&state), stream)
                    .instrument(info_span!("conn", %addy).or_current())
            );
        }
        
        #[allow(unreachable_code)]
        Ok(())
    }
}