use crate::connection::StreamWriter;
use crate::packets::outgoing::status_response::StatusResponse;
use crate::packets::IncomingPacket;
use crate::{NetResult, ServerState};
use ferrumc_config::favicon::get_favicon_base64;
use ferrumc_config::statics::get_global_config;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::encode::NetEncodeOpts;
use rand::seq::IndexedRandom;
use std::sync::Arc;

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x00, state = "status")]
pub struct StatusRequestPacket {

}

impl IncomingPacket for StatusRequestPacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let response = StatusResponse::new(get_server_status());

        let mut writer = state
            .universe
            .get_mut::<StreamWriter>(conn_id)?;

        writer.send_packet(&response, &NetEncodeOpts::WithLength).await?;

        Ok(())
    }
}

fn get_server_status() -> String {
    mod structs {
        #[derive(serde_derive::Serialize)]
        pub(super) struct ServerStatus<'a> {
            pub version: Version<'a>,
            pub players: Players<'a>,
            pub description: Description<'a>,
            pub favicon: String,
            pub enforces_secure_chat: bool,
        }

        #[derive(serde_derive::Serialize)]
        pub(super)struct Version<'a> {
            pub name: &'a str,
            pub protocol: u16,
        }

        #[derive(serde_derive::Serialize)]
        pub(super)struct Players<'a> {
            pub max: u32,
            pub online: u16,
            pub sample: Vec<Player<'a>>,
        }

        #[derive(serde_derive::Serialize)]
        pub(super)struct Player<'a> {
            pub name: &'a str,
            pub id: &'a str,
        }

        #[derive(serde_derive::Serialize)]
        pub(super)struct Description<'a> {
            pub text: &'a str,
        }
    }

    let config = get_global_config();

    let version = structs::Version {
        name: "1.21.1",
        protocol: 767,
    };


    // TODO: Get online players and make players count dynamic
    let players = structs::Players {
        max: config.max_players,
        online: 0,
        sample: vec![structs::Player {
            name: "Sweattypalms",
            id: "00000000-0000-0000-0000-000000000000",
        }, structs::Player {
            name: "ReCore_",
            id: "00000000-0000-0000-0000-000000000000",
        }],
    };
    
    
    let motd = config.motd.choose(&mut rand::thread_rng()).unwrap();
    let description = structs::Description {
        text: motd,
    };
    
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

