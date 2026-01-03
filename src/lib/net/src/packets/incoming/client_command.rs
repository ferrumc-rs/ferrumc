//! Client Command packet.
//!
//! Sent by the client to perform various actions:
//! - Action 0: Request respawn after death

use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Client command actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientCommandAction {
    /// Request to respawn after death
    PerformRespawn = 0,
    /// Request game statistics (not implemented)
    RequestStats = 1,
}

impl From<i32> for ClientCommandAction {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::PerformRespawn,
            1 => Self::RequestStats,
            _ => Self::PerformRespawn, // Default to respawn
        }
    }
}

/// Sent by the client to request respawn or stats.
#[derive(NetDecode, Debug)]
#[packet(packet_id = "client_command", state = "play")]
pub struct ClientCommand {
    /// The action to perform (0 = respawn, 1 = request stats)
    pub action: VarInt,
}

impl ClientCommand {
    /// Get the action as a typed enum.
    pub fn action_type(&self) -> ClientCommandAction {
        ClientCommandAction::from(self.action.0)
    }

    /// Check if this is a respawn request.
    pub fn is_respawn_request(&self) -> bool {
        self.action.0 == 0
    }
}
