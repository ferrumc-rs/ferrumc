use bevy_ecs::prelude::{Component, Entity, Query};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::debug;

#[derive(NetEncode)]
#[packet(packet_id = "player_info_update", state = "play")]
pub struct PlayerInfoUpdatePacket {
    pub actions: u8,
    pub numbers_of_players: VarInt,
    pub players: Vec<PlayerWithActions>,
}

impl PlayerInfoUpdatePacket {
    pub fn with_players(players: Vec<PlayerWithActions>) -> Self {
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
    pub fn new_player_join_packet(identity: PlayerIdentity) -> Self {
        let player = PlayerWithActions::add_player(identity.short_uuid, identity.username);

        Self::with_players(vec![player])
    }

    /// The packet to be sent to a new player when they join the server,
    /// To let them know about all the players that are already connected
    pub fn existing_player_info_packet(
        new_player_id: Entity,
        query: Query<(Entity, &PlayerIdentity)>,
    ) -> Self {
        let players: Vec<&PlayerIdentity> = {
            let players = query.iter().collect::<Vec<(_, _)>>();
            players
                .iter()
                .filter(|&player| player.0 == new_player_id)
                .map(|player| player.1)
                .collect()
        };

        let players = players
            .into_iter()
            .map(|player| {
                let uuid = player.short_uuid;
                let name = player.username.clone();

                (uuid, name)
            })
            .map(|(uuid, name)| PlayerWithActions::add_player(uuid, name))
            .collect::<Vec<_>>();

        debug!("Sending PlayerInfoUpdatePacket with {:?} players", players);

        Self::with_players(players)
    }
}

#[derive(NetEncode, Debug, Component)]
pub struct PlayerWithActions {
    pub uuid: i32,
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

    pub fn add_player(uuid: i32, name: impl Into<String>) -> Self {
        Self {
            uuid,
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
