use bevy_ecs::prelude::Query;
use ferrumc_commands::Sender;
use ferrumc_macros::command;
use ferrumc_nbt::NBT;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::show_dialog::{DialogBody, DialogContent, ShowDialog};
use ferrumc_net_codec::net_types::adhoc_id::AdHocID;
use ferrumc_text::TextComponent;

static CREDITS_TEXT: &str = include_str!("../../../../assets/data/credits.txt");

#[command("credits")]
fn test_command(#[sender] sender: Sender, query: Query<&StreamWriter>) {
    let conn = match sender {
        Sender::Server => {
            // Server cannot have credits
            return;
        }
        Sender::Player(entity) => query.get(entity).expect("sender does not exist"),
    };
    let lines = CREDITS_TEXT
        .lines()
        .map(|t| DialogBody {
            dialog_body_type: "minecraft:plain_message".to_string(),
            contents: TextComponent::from(t),
            width: Some(1024),
        })
        .collect::<Vec<_>>();
    let packet = ShowDialog {
        content: AdHocID::from(NBT::from(DialogContent {
            dialog_content_type: "minecraft:notice".to_string(),
            title: TextComponent::from("Credits"),
            body: lines,
        })),
    };
    conn.send_packet(packet).unwrap();
}
