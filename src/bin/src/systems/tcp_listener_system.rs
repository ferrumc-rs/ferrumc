use crate::errors::BinaryError;
use crate::systems::definition::System;
use ferrumc_net::connection::handle_connection;
use ferrumc_state::GlobalState;
use std::sync::Arc;
use std::thread;
use tracing::{debug, error, info, info_span, Instrument};

pub struct TcpListenerSystem;

impl System for TcpListenerSystem {
    fn start(self: Arc<Self>, state: GlobalState) {
        if let Err(e) = TcpListenerSystem::initiate_loop(state) {
            error!("TCP listener system failed with error: {:?}", e);
        }
    }

    fn stop(self: Arc<Self>, _state: GlobalState) {
        debug!("Stopping TCP listener system...");
    }

    fn name(&self) -> &'static str {
        "tcp"
    }
}

impl TcpListenerSystem {
    fn initiate_loop(state: GlobalState) -> Result<(), BinaryError> {
        let tcp_listener = &state.tcp_listener;
        info!("Server is listening on [{}]", tcp_listener.local_addr()?);

        loop {
            debug!("Accepting connection");
            let (stream, _) = tcp_listener.accept()?;
            let addy = stream.peer_addr()?;
            thread::spawn({
                let state = Arc::clone(&state);
                move || {
                    let _ = handle_connection(Arc::clone(&state), stream)
                        .instrument(info_span!("conn", %addy).or_current());
                }
            });
        }

        #[allow(unreachable_code)]
        Ok(())
    }
}
