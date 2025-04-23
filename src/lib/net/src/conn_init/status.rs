use crate::errors::NetError;
use crate::packets::incoming::ping::PingPacket;
use crate::packets::incoming::status_request::StatusRequestPacket;
use crate::packets::outgoing::ping_response::PongPacket;
use crate::packets::outgoing::status_response::StatusResponse;
use crate::trim_packet_head;
use ferrumc_config::favicon::get_favicon_base64;
use ferrumc_config::statics::get_global_config;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::{GlobalState, ServerState};
use rand::prelude::IndexedRandom;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub(super) async fn status(
    mut conn_read: &mut OwnedReadHalf,
    conn_write: &mut OwnedWriteHalf,
    state: GlobalState,
) -> Result<bool, NetError> {
    trim_packet_head!(conn_read, 0x00);

    // Wait for a status request packet
    let _ = StatusRequestPacket::decode_async(&mut conn_read, &NetDecodeOpts::None).await?;

    // Send a status response packet
    let sr_packet = StatusResponse {
        json_response: get_server_status(&state),
    };

    let mut packet_buffer = vec![];

    sr_packet
        .encode_async(&mut packet_buffer, &NetEncodeOpts::WithLength)
        .await?;

    conn_write.write_all(&packet_buffer).await?;

    trim_packet_head!(conn_read, 0x01);

    // Wait for a ping request packet
    let ping_req = PingPacket::decode_async(&mut conn_read, &NetDecodeOpts::None).await?;

    // Send a ping response packet
    let pong_packet = PongPacket {
        payload: ping_req.payload,
    };

    let mut packet_buffer = vec![];

    pong_packet
        .encode_async(&mut packet_buffer, &NetEncodeOpts::WithLength)
        .await?;

    conn_write.write_all(&packet_buffer).await?;

    Ok(true)
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
        protocol: crate::conn_init::PROTOCOL_VERSION_1_21_1 as u16,
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
