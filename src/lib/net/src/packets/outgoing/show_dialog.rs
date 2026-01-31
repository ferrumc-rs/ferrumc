use ferrumc_macros::{packet, NBTSerialize, NetEncode};
use ferrumc_nbt::NBT;
use ferrumc_net_codec::net_types::adhoc_id::AdHocID;
use ferrumc_text::TextComponent;

#[derive(NetEncode)]
#[packet(packet_id = "show_dialog", state = "play")]
pub struct ShowDialog {
    pub content: AdHocID<NBT<DialogContent>>,
}

#[derive(NBTSerialize)]
pub struct DialogContent {
    #[nbt(rename = "type")]
    pub dialog_content_type: String,
    pub title: TextComponent,
    pub body: Vec<DialogBody>,
}

#[derive(NBTSerialize, Debug)]
pub struct DialogBody {
    #[nbt(rename = "type")]
    pub dialog_body_type: String,
    pub contents: TextComponent,
    pub width: Option<i32>,
}
