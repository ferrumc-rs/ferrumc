use log::info;
use serde::Serialize;
use ferrumc_macros::Decode;
use ferrumc_utils::encoding::varint::VarInt;
use crate::Connection;
use crate::packets::IncomingPacket;
use crate::packets::outgoing::status::OutgoingStatusResponse;
use serde_json;
use ferrumc_utils::config;

#[derive(Decode)]
pub struct IncomingStatusRequest;

#[derive(Serialize)]
struct JsonResponse {
    version: Version,
    players: Players,
    description: Description,
}
#[derive(Serialize)]
struct Version {
    name: String,
    protocol: u32,
}
#[derive(Serialize)]
struct Players {
    max: u32,
    online: u32,
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

impl IncomingPacket for IncomingStatusRequest {
    async fn handle(&self, _: &mut Connection) -> Result<Option<Vec<u8>>, ferrumc_utils::error::Error> {
        info!("Handling status request packet");
        let config = config::ServerConfig::new()?;

        let response = OutgoingStatusResponse {
            packet_id: VarInt::new(0x00),
            json_response: serde_json::ser::to_string(&JsonResponse {
                version: Version {
                    name: "1.20.1".to_string(),
                    protocol: 763,
                },
                players: Players {
                    max: config.max_players,
                    online: 2,
                    sample: vec![
                        Sample {
                            name: "Recore_".to_string(),
                            id: "2b3414ed-468a-45c2-b113-6c5f47430edc".to_string(),
                        },
                        Sample {
                            name: "sweatypalms".to_string(),
                            id: "26d88d10-f052-430f-9406-e6c3089792c4".to_string(),
                        }
                    ],
                },
                description: Description {
                    text: config.motd,
                },
            }).unwrap(),
        };

        Ok(Some(response.encode().await?))
    }
}