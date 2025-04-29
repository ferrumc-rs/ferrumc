pub mod incoming;
pub mod outgoing;
pub mod packet_events;

#[enum_delegate::register]
pub trait IncomingPacket {
    fn handle(
        self,
        conn_id: usize,
        state: std::sync::Arc<ferrumc_state::ServerState>,
    ) -> Result<(), NetError>;
}

use crate::errors::NetError;
use incoming::*;
#[enum_delegate::implement(IncomingPacket)]
pub enum AnyIncomingPacket {
    Handshake(handshake::Handshake),
    LoginStartPacket(login_start::LoginStartPacket),
    ClientInformation(client_information::ClientInformation),
    LoginAcknowledgedPacket(login_acknowledged::LoginAcknowledgedPacket),
    AckFinishConfigurationPacket(ack_finish_configuration::AckFinishConfigurationPacket),
    ChunkBatchAck(chunk_batch_ack::ChunkBatchAck),
    IncomingKeepAlivePacket(keep_alive::IncomingKeepAlivePacket),
    SwingArmPacket(swing_arm::SwingArmPacket),
    PlaceBlock(place_block::PlaceBlock),
    ServerBoundKnownPacks(server_bound_known_packs::ServerBoundKnownPacks),
    ServerBoundPluginMessage(server_bound_plugin_message::ServerBoundPluginMessage),
    PlayerAction(player_action::PlayerAction),
    PingPacket(ping::PingPacket),
    SetPlayerPositionAndRotationPacket(
        set_player_position_and_rotation::SetPlayerPositionAndRotationPacket,
    ),
    SetPlayerPositionPacket(set_player_position::SetPlayerPositionPacket),
    StatusRequestPacket(status_request::StatusRequestPacket),
    SetPlayerRotationPacket(set_player_rotation::SetPlayerRotationPacket),
    PlayerCommandPacket(player_command::PlayerCommandPacket),
    ConfirmPlayerTeleport(confirm_player_teleport::ConfirmPlayerTeleport),
}
