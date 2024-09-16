use base64::Engine;
use ferrumc_codec::network_types::varint::VarInt;
use rand::prelude::IndexedRandom;
use serde::Serialize;
use tokio::io::AsyncReadExt;
use tokio::sync::OnceCell;
use tracing::debug;

use ferrumc_macros::{packet, NetDecode};
use uuid::Uuid;

use crate::net::packets::outgoing::status::OutgoingStatusResponse;
use crate::net::packets::{ConnectionId, IncomingPacket};
use crate::state::GlobalState;
use crate::utils::components::player::Player;
use crate::utils::config;
use crate::utils::prelude::*;

/// The status packet is sent by the client to the server to request the server's status.
///
/// Usually sent after handshaking is completed.
#[derive(NetDecode)]
#[packet(packet_id = 0x00, state = "status")]
pub struct Status;

/// The response to the status packet.
/// Sent as json.
#[derive(Serialize)]
struct JsonResponse {
    version: Version,
    players: Players,
    description: Description,
    favicon: &'static String,
}

#[derive(Serialize)]
struct Version {
    name: String,
    protocol: u32,
}

#[derive(Serialize)]
struct Players {
    max: i32,
    online: i32,
    sample: Vec<Sample>,
}

#[derive(Serialize)]
struct Sample {
    name: String,
    id: String,
}

#[derive(Serialize)]
struct Description {
    text: String,
}

impl IncomingPacket for Status {
    async fn handle(self, conn_id: ConnectionId, state: GlobalState) -> Result<()> {
        debug!("Handling status request packet");
        let config = config::get_global_config();

        let conn = state.connections.get_connection(conn_id)?;
        let conn = conn.read().await;

        let random_motd = config.motd.choose(&mut rand::thread_rng()).unwrap().clone();

        //Queries all players and makes a Sample struct from them
        let player_query = state.world.query::<&Player>();
        let players = player_query.iter().await.collect::<Vec<_>>();
        let player_samples: Vec<Sample> = players.iter().map(|(_, player)| Sample{
            name: player.username.to_string(),
            id: Uuid::from_u128(player.uuid).to_string(),
        }).collect();

        let response = OutgoingStatusResponse {
            packet_id: VarInt::new(0x00),
            json_response: serde_json::ser::to_string(&JsonResponse {
                version: Version {
                    name: "1.20.6".to_string(),
                    // Allow any protocol version for now. To check the ping and stuff
                    protocol: conn.metadata.protocol_version as u32,
                },
                players: Players {
                    max: config.max_players,
                    online: player_samples.len() as i32,
                    sample: player_samples,
                },
                description: Description { text: random_motd },
                favicon: get_encoded_favicon().await,
            })
            .unwrap(),
        };

        conn.send_packet(response).await?;
        /*let mut cursor = std::io::Cursor::new(Vec::new());
        response.net_encode(&mut cursor).await?;
        let response = cursor.into_inner();

        let response = &*response;

        conn.socket.write(response).await?;*/

        Ok(())
    }
}

/// Get the favicon as a base64 encoded string.
///
/// This is cached in a `OnceCell` to avoid reading the file every time.
async fn get_encoded_favicon() -> &'static String {
    static FAVICON: OnceCell<String> = OnceCell::const_new();
    FAVICON
        .get_or_init(|| async {
            let mut data = Vec::new();
            let Ok(mut image) = tokio::fs::File::open("icon-64.png").await else {
                return String::new();
            };
            image.read_to_end(&mut data).await.unwrap_or_default();
            let data = base64::engine::general_purpose::STANDARD.encode(&data);
            format!("data:image/png;base64,{}", data)
        })
        .await
}
