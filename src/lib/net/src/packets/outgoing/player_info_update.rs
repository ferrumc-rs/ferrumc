use base64::Engine;
use bevy_ecs::prelude::{Component, Entity, Query};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use tracing::debug;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;

#[derive(NetEncode)]
#[packet(packet_id = "player_info_update", state = "play")]
pub struct PlayerInfoUpdatePacket {
    pub actions: u8,
    pub players: LengthPrefixedVec<PlayerWithActions>,
}

impl PlayerInfoUpdatePacket {
    pub fn with_players(players: Vec<PlayerWithActions>) -> Self {
        let players: Vec<PlayerWithActions> = players.into_iter().collect();
        Self {
            actions: players
                .iter()
                .map(|player| player.get_actions_mask())
                .fold(0, |acc, x| acc | x),
            players: LengthPrefixedVec::new(players),
        }
    }

    /// The packet to be sent to all already connected players when a new player joins the server
    pub fn new_player_join_packet(identity: &PlayerIdentity) -> Self {
        Self::with_players(vec![PlayerWithActions::add_player(identity)])
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
            .map(|identity| PlayerWithActions::add_player(identity))
            .collect::<Vec<_>>();

        debug!("Sending PlayerInfoUpdatePacket with {:?} players", players);

        Self::with_players(players)
    }
}

#[derive(NetEncode, Debug, Component)]
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
                PlayerAction::UpdateListed { .. } => 0x08,
            }
        }
        mask
    }

    pub fn add_player(identity: &PlayerIdentity) -> Self {
        Self {
            uuid: identity.uuid.as_u128(),
            actions: vec![
                PlayerAction::AddPlayer {
                    name: identity.username.clone(),
                    properties: LengthPrefixedVec::new(
                        identity.properties
                            .iter()
                            .map(|property| PlayerProperty {
                                name: property.name.clone(),
                                value: base64::engine::general_purpose::STANDARD.encode(&property.value),
                                signature: PrefixedOptional::new(property.signature.clone())
                            })
                            .collect(),
                    ),
                },
                PlayerAction::UpdateListed { is_listed: true },
            ],
        }
    }
}

#[derive(NetEncode, Debug)]
pub enum PlayerAction {
    AddPlayer {
        name: String,
        properties: LengthPrefixedVec<PlayerProperty>,
    },
    UpdateListed {
        is_listed: bool,
    }
}

#[derive(NetEncode, Debug)]
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
    pub signature: PrefixedOptional<String>,
}
