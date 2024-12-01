use crate::{
    connection::{ConnectionControl, StreamWriter},
    errors::NetError,
    packets::outgoing::disconnect::DisconnectPacket,
    GlobalState, NetResult,
};
use ferrumc_net_codec::encode::NetEncodeOpts;
use tracing::{trace, warn};

use super::ecs_helpers::EntityExt;

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
pub async fn terminate_connection(
    state: GlobalState,
    conn_id: usize,
    reason: String,
) -> NetResult<()> {
    let mut writer = match conn_id.get_mut::<StreamWriter>(&state.clone()) {
        Ok(writer) => writer,
        Err(e) => {
            warn!("Failed to get stream writer for entity {}: {}", conn_id, e);
            return Err(NetError::ECSError(e));
        }
    };

    if let Err(e) = writer
        .send_packet(&DisconnectPacket::from_string(reason), &NetEncodeOpts::WithLength)
        .await
    {
        warn!(
            "Failed to send disconnect packet to entity {}: {}",
            conn_id, e
        );
        return Err(e);
    }

    match conn_id.get_mut::<ConnectionControl>(&state.clone()) {
        Ok(mut control) => {
            control.should_disconnect = true;

            trace!("Set should_disconnect to true for entity {}", conn_id);
        }
        Err(e) => {
            warn!(
                "Failed to get connection control for entity {}: {}",
                conn_id, e
            );
            return Err(NetError::ECSError(e));
        }
    }

    Ok(())
}
