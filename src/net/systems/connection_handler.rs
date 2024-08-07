use async_trait::async_trait;
use ferrumc_macros::AutoGenName;
use tracing::debug;
use crate::net::systems::System;
use crate::state::GlobalState;

#[derive(AutoGenName)]
pub struct ConnectionHandler;

#[async_trait]
impl System for ConnectionHandler {
    async fn run(&self, _state: GlobalState) {
        debug!("ConnectionHandler is starting up");
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}