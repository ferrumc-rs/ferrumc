use crate::conn_init::{send_packet, trim_packet_head};
use crate::errors::NetError;
use crate::packets::incoming::ping::PingPacket;
use crate::packets::incoming::status_request::StatusRequestPacket;
use crate::packets::outgoing::ping_response::PongPacket;
use crate::packets::outgoing::status_response::StatusResponse;
use ferrumc_config::favicon::get_favicon_base64;
use ferrumc_config::statics::get_global_config;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_state::GlobalState;
use rand::prelude::IndexedRandom;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub(super) async fn status(
    mut conn_read: &mut OwnedReadHalf,
    conn_write: &mut OwnedWriteHalf,
    state: GlobalState,
) -> Result<bool, NetError> {
    trim_packet_head(conn_read, 0x00).await?;

    // Wait for a status request packet
    let _status_req =
        StatusRequestPacket::decode_async(&mut conn_read, &NetDecodeOpts::None).await?;

    // Send a status response packet
    let status_response = StatusResponse {
        json_response: get_server_status(&state),
    };

    send_packet(conn_write, &status_response).await?;

    trim_packet_head(conn_read, 0x01).await?;

    // Wait for a ping request packet
    let ping_req = PingPacket::decode_async(&mut conn_read, &NetDecodeOpts::None).await?;

    // Send a ping response packet
    let pong_packet = PongPacket {
        payload: ping_req.payload,
    };

    send_packet(conn_write, &pong_packet).await?;

    Ok(true)
}

fn get_server_status(state: &GlobalState) -> String {
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
        protocol: crate::conn_init::PROTOCOL_VERSION_1_21_5 as u16,
    };

    let online_players_sample = state
        .players
        .iter()
        .take(5)
        .map(|player_data| structs::PlayerData {
            name: player_data.value().clone(),
            id: uuid::Uuid::from_u128(*player_data.key()).to_string(),
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
        online: online_players_sample.len() as u16,
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
