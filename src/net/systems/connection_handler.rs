use async_trait::async_trait;
use ferrumc_macros::AutoGenName;
use tracing::{debug, error, info_span, Instrument};
use crate::net::systems::System;
use crate::state::GlobalState;
use crate::utils::prelude::*;

#[derive(AutoGenName)]
pub struct ConnectionHandler;

#[async_trait]
impl System for ConnectionHandler {
    async fn run(&self, state: GlobalState) {
        debug!("ConnectionHandler is starting up");

        if let Err(e) = Self::handle_connections(state).await {
            error!("There was an error in the ConnectionHandler: {:?}", e);
        }
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}

impl ConnectionHandler {
    async fn handle_connections(state: GlobalState) -> Result<()> {
        loop {
            let (stream, _) = state.server_stream.accept().await?;
            debug!("Accepted connection from {:?}", stream.peer_addr()?);
            let addy = stream.peer_addr()?;
            tokio::task::spawn(
                Self::handle_connection(
                    state.clone(),
                    stream
                ).instrument(info_span!("conn", %addy).or_current()),
            );
        }
    }

    async fn handle_connection(state: GlobalState, stream: tokio::net::TcpStream) -> Result<()> {
        crate::net::init_connection(stream, state).await?;
        Ok(())
    }
}