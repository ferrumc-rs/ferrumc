use crate::connection::StreamWriter;
use crate::packets::outgoing::status_response::StatusResponse;
use crate::packets::IncomingPacket;
use crate::{NetResult, ServerState};
use ferrumc_config::favicon::get_favicon_base64;
use ferrumc_config::statics::get_global_config;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::encode::NetEncodeOpts;
use rand::seq::IndexedRandom;
use std::sync::Arc;

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x00, state = "status")]
pub struct StatusRequestPacket {}

impl IncomingPacket for StatusRequestPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let response = StatusResponse::new(get_server_status(&state));

        let mut writer = state.universe.get_mut::<StreamWriter>(conn_id)?;

        writer
            .send_packet(&response, &NetEncodeOpts::WithLength)
            .await?;

        Ok(())
    }
}

fn get_server_status(state: &Arc<ServerState>) -> String {
    mod structs {
        #[derive(serde_derive::Serialize)]
        pub(super) struct ServerStatus<'a> {
            pub version: Version<'a>,
            pub players: Players<'a>,
            pub description: Description<'a>,
            pub favicon: &'a str,
            pub enforces_secure_chat: bool,
        }

        #[derive(serde_derive::Serialize)]
        pub(super) struct Version<'a> {
            pub name: &'a str,
            pub protocol: u16,
        }

        #[derive(serde_derive::Serialize)]
        pub(super) struct Players<'a> {
            pub max: u32,
            pub online: u16,
            pub sample: Vec<Player<'a>>,
        }

        #[derive(serde_derive::Serialize)]
        pub(super) struct Player<'a> {
            pub name: &'a str,
            pub id: &'a str,
        }

        pub(super) struct PlayerData {
            pub name: String,
            pub id: String,
        }

        #[derive(serde_derive::Serialize)]
        pub(super) struct Description<'a> {
            pub text: &'a str,
        }
    }

    let config = get_global_config();

    let version = structs::Version {
        name: "1.21.1",
        protocol: 767,
    };

    let online_players = state.universe.query::<&PlayerIdentity>().into_entities();
    let online_players_sample = online_players
        .iter()
        .take(5)
        .filter_map(|entity| state.universe.get::<PlayerIdentity>(*entity).ok())
        .map(|player| structs::PlayerData {
            name: player.username.clone(),
            id: uuid::Uuid::from_u128(player.uuid).to_string(),
        })
        .collect::<Vec<_>>();

    let online_players_sample = online_players_sample
        .iter()
        .map(|p| structs::Player {
            name: p.name.as_str(),
            id: p.id.as_str(),
        })
        .collect::<Vec<_>>();

    let players = structs::Players {
        max: config.max_players,
        online: online_players.len() as u16,
        sample: online_players_sample,
    };

    let motd = config.motd.choose(&mut rand::rng()).unwrap();
    let description = structs::Description { text: motd };

    let favicon = get_favicon_base64();
    // let favicon = "data:image/png;base64,<data>";

    let status = structs::ServerStatus {
        version,
        players,
        description,
        favicon,
        enforces_secure_chat: false,
    };

    serde_json::to_string(&status).unwrap()
}
