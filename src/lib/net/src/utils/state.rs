use crate::{
    connection::{ConnectionControl, ConnectionState, StreamWriter},
    errors::NetError,
    packets::outgoing::disconnect::*,
    NetResult,
};
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::GlobalState;
use tracing::{trace, warn};

use super::ecs_helpers::EntityExt;

pub trait TerminateConnectionPlayerExt {
    #[allow(async_fn_in_trait)]
    async fn terminate_connection(
        &self,
        state: GlobalState,
        reason: impl Into<ferrumc_text::TextComponent>,
    ) -> NetResult<()>;
}

impl TerminateConnectionPlayerExt for usize {
    // used codium for this function comment, very useful

    /// Terminates the connection of an entity with the given `conn_id`.
    ///
    /// Sends a disconnect packet with the given `reason` to the client, and marks the connection as
    /// terminated. This will cause the connection to be dropped on the next tick of the
    /// `ConnectionSystem`.
    ///
    /// # Errors
    ///
    /// Returns an error if the stream writer or connection control component cannot be accessed for
    /// the given `conn_id`.
    async fn terminate_connection(
        &self,
        state: GlobalState,
        reason: impl Into<ferrumc_text::TextComponent>,
    ) -> NetResult<()> {
        let mut writer = match self.get_mut::<StreamWriter>(&state.clone()) {
            Ok(writer) => writer,
            Err(e) => {
                warn!("Failed to get stream writer for entity {}: {}", self, e);
                return Err(NetError::ECSError(e));
            }
        };

        let conn_state = self.get::<ConnectionState>(&state.clone())?;

        if let Err(e) = writer
            .send_packet(
                &DisconnectPacket::from(&conn_state, reason)?,
                &NetEncodeOpts::WithLength,
            )
            .await
        {
            warn!(
                "Failed to send disconnect packet to entity {}: {}",
                self, e
            );
            return Err(e);
        }

        match self.get_mut::<ConnectionControl>(&state.clone()) {
            Ok(mut control) => {
                control.should_disconnect = true;

                trace!("Set should_disconnect to true for entity {}", self);
            }
            Err(e) => {
                warn!(
                    "Failed to get connection control for entity {}: {}",
                    self, e
                );
                return Err(NetError::ECSError(e));
            }
        }

        Ok(())
    }
}
