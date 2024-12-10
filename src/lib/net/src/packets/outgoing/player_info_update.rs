use ferrumc_macros::{NetEncode, packet};
use ferrumc_net_codec::net_types::{
    var_int::VarInt,
    length_prefixed_vec::LengthPrefixedVec,
};
use ferrumc_core::identity::player_identity::*;
use bitmask_enum::bitmask;
use std::io::Write;

#[bitmask(u8)]
#[derive(NetEncode)]
pub enum PlayerActions {
    AddPlayer = 0x01,
    InitializeChat = 0x02,
    UpdateGameMode = 0x04,
    UpdateListed = 0x08,
    UpdateLatency = 0x10,
    UpdateDisplayName = 0x20,
}

#[derive(NetEncode, Debug, Eq, PartialEq, Clone)]
pub enum PlayerAction {
    AddPlayer {
        username: String,
        properties: LengthPrefixedVec<IdentityProperty>,
    },
    InitializeChat { // TODO
    },
    UpdateGameMode(VarInt),
    UpdateListed(bool),
    UpdateLatency(VarInt),
    UpdateDisplayName { // TODO
    },
}

impl PlayerAction {
    pub fn flags(&self) -> PlayerActions {
        match self {
            Self::AddPlayer { .. } => PlayerActions::AddPlayer,
            Self::InitializeChat { .. } => PlayerActions::InitializeChat,
            Self::UpdateGameMode(..) => PlayerActions::UpdateGameMode,
            Self::UpdateListed(..) => PlayerActions::UpdateListed,
            Self::UpdateLatency(..) => PlayerActions::UpdateLatency,
            Self::UpdateDisplayName { .. } => PlayerActions::UpdateDisplayName,
        }
    }
}

#[derive(NetEncode, Debug, Eq, PartialEq)]
pub struct PlayerInfo {
    pub uuid: u128,
    pub actions: Vec<PlayerAction>,
}

impl PlayerInfo {
    pub fn from(profile: &PlayerIdentity) -> Self {
        Self {
            uuid: profile.uuid,
            actions: vec![
                PlayerAction::AddPlayer {
                    username: profile.username.clone(),
                    properties: profile.properties.clone(),
                },
                PlayerAction::UpdateListed(true),
            ],
        }
    }
}

#[derive(NetEncode, Debug)]
#[packet(packet_id = 0x3E)]
pub struct PlayerInfoUpdatePacket {
    actions: PlayerActions,
    infos: LengthPrefixedVec<PlayerInfo>,
}

impl PlayerInfoUpdatePacket {
    pub fn new(infos: Vec<PlayerInfo>) -> Self {
        Self {
            actions: Self::get_actions(&infos),
            infos: LengthPrefixedVec::new(infos),
        }
    }

    fn get_actions(infos: &[PlayerInfo]) -> PlayerActions { 
        let first = &infos[0].actions;
        let mut flags = PlayerActions::none();

        for action in first.iter() {
            flags |= action.flags();
        }

        flags
    }
}
