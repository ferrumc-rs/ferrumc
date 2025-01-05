use crate::utils::{broadcast::get_all_play_players, ecs_helpers::EntityExt};
use ferrumc_core::identity::player_identity::*;
use ferrumc_ecs::entities::Entity;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::{length_prefixed_vec::LengthPrefixedVec, var_int::VarInt};
use ferrumc_state::GlobalState;
use std::io::Write;
use tracing::debug;

#[derive(NetEncode, Debug, Eq, PartialEq, Clone)]
pub enum PlayerAction {
    AddPlayer {
        username: String,
        properties: LengthPrefixedVec<IdentityProperty>,
    },
    InitializeChat {
        // TODO
    },
    UpdateGameMode(VarInt),
    UpdateListed(bool),
    UpdateLatency(VarInt),
    UpdateDisplayName {
        // TODO
    },
}

impl PlayerAction {
    pub fn flags(&self) -> u8 {
        match self {
            Self::AddPlayer { .. } => 0x01,
            Self::InitializeChat { .. } => 0x02,
            Self::UpdateGameMode(..) => 0x04,
            Self::UpdateListed(..) => 0x08,
            Self::UpdateLatency(..) => 0x10,
            Self::UpdateDisplayName { .. } => 0x20,
        }
    }
}

#[derive(NetEncode, Debug, Eq, PartialEq)]
pub struct PlayerInfo {
    pub uuid: u128,
    pub actions: Vec<PlayerAction>,
}

#[derive(NetEncode)]
#[packet(packet_id = "player_info_update", state = "play")]
pub struct PlayerInfoUpdatePacket {
    actions: u8,
    infos: LengthPrefixedVec<PlayerInfo>,
}

impl PlayerInfoUpdatePacket {
    pub fn with_players<T>(players: T) -> Self
    where
        T: IntoIterator<Item = PlayerInfo>,
    {
        let players: Vec<_> = players.into_iter().collect();
        Self {
            actions: players
                .iter()
                .map(|player| {
                    player
                        .actions
                        .iter()
                        .fold(0, |acc, action| acc | action.flags())
                })
                .fold(0, |acc, x| acc | x),
            infos: LengthPrefixedVec::new(players),
        }
    }

    /// The packet to be sent to all already connected players when a new player joins the server
    pub fn new_player_join_packet(new_player_id: Entity, state: &GlobalState) -> Self {
        let identity = new_player_id.get::<PlayerIdentity>(state).unwrap();
        Self::with_players(vec![PlayerInfo::from(&identity)])
    }

    /// The packet to be sent to a new player when they join the server,
    /// To let them know about all the players that are already connected
    pub fn existing_player_info_packet(new_player_id: Entity, state: &GlobalState) -> Self {
        let players = {
            let mut players = get_all_play_players(state);
            players.retain(|&player| player != new_player_id);
            players
        };

        let players = players
            .into_iter()
            .filter_map(|player| state.universe.get::<PlayerIdentity>(player).ok())
            .map(|identity| PlayerInfo::from(&identity))
            .collect::<Vec<_>>();

        debug!("Sending PlayerInfoUpdatePacket with {:?} players", players);

        Self::with_players(players)
    }
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
