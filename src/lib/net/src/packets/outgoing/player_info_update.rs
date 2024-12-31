use crate::utils::broadcast::get_all_play_players;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_ecs::entities::Entity;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;
use std::io::Write;
use tracing::{debug};

#[derive(NetEncode)]
#[packet(packet_id = 0x3E)]
pub struct PlayerInfoUpdatePacket {
    pub actions: u8,
    pub numbers_of_players: VarInt,
    pub players: Vec<PlayerWithActions>,
}

impl PlayerInfoUpdatePacket {
    pub fn with_players<T>(players: T) -> Self
    where
        T: IntoIterator<Item = PlayerWithActions>,
    {
        let players: Vec<PlayerWithActions> = players.into_iter().collect();
        Self {
            actions: players
                .iter()
                .map(|player| player.get_actions_mask())
                .fold(0, |acc, x| acc | x),
            numbers_of_players: VarInt::new(players.len() as i32),
            players,
        }
    }

    /// The packet to be sent to all already connected players when a new player joins the server
    pub fn new_player_join_packet(new_player_id: Entity, state: &GlobalState) -> Self {
        let identity = state
            .universe
            .get_component_manager()
            .get::<PlayerIdentity>(new_player_id)
            .unwrap();
        let uuid = identity.uuid;
        let name = identity.username.clone();

        let player = PlayerWithActions::add_player(uuid, name);

        Self::with_players(vec![player])
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
            .filter_map(|player| {
                let identity = state
                    .universe
                    .get_component_manager()
                    .get::<PlayerIdentity>(player)
                    .ok()?;
                let uuid = identity.uuid;
                let name = identity.username.clone();

                Some((uuid, name))
            })
            .map(|(uuid, name)| PlayerWithActions::add_player(uuid, name))
            .collect::<Vec<_>>();

        debug!("Sending PlayerInfoUpdatePacket with {:?} players", players);

        Self::with_players(players)
    }
}

#[derive(NetEncode, Debug)]
pub struct PlayerWithActions {
    pub uuid: u128,
    pub actions: Vec<PlayerAction>,
}

impl PlayerWithActions {
    pub fn get_actions_mask(&self) -> u8 {
        let mut mask = 0;
        for action in &self.actions {
            mask |= match action {
                PlayerAction::AddPlayer { .. } => 0x01,
            }
        }
        mask
    }

    pub fn add_player(uuid: impl Into<u128>, name: impl Into<String>) -> Self {
        Self {
            uuid: uuid.into(),
            actions: vec![PlayerAction::AddPlayer {
                name: name.into(),
                properties: LengthPrefixedVec::default(),
            }],
        }
    }
}

#[derive(NetEncode, Debug)]
pub enum PlayerAction {
    AddPlayer {
        name: String,
        properties: LengthPrefixedVec<PlayerProperty>,
    },
}

#[derive(NetEncode, Debug)]
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    pub signature: Option<String>,
}
