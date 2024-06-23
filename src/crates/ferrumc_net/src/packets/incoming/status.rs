use log::info;
use serde::Serialize;
use ferrumc_macros::Decode;
use ferrumc_utils::encoding::varint::VarInt;
use crate::Connection;
use crate::packets::IncomingPacket;
use crate::packets::outgoing::status::OutgoingStatusResponse;
use serde_json;

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
}
#[derive(Serialize)]
struct Description {
    text: String,
}

impl IncomingPacket for IncomingStatusRequest {
    async fn handle(&self, conn: &mut Connection) -> Result<Option<Vec<u8>>, Error> {
        info!("Handling status request packet");

        let response = OutgoingStatusResponse {
            packet_id: VarInt::new(0x00),
            json_response: serde_json::ser::to_string(&JsonResponse {
                version: Version {
                    name: "1.20.1".to_string(),
                    protocol: 763,
                },
                players: Players {
                    max: 20,
                    online: 0,
                },
                description: Description {
                    text: "Hello, world!".to_string(),
                },
            }).unwrap(),
        };

        Ok(Some(response.encode().await?))
    }
}