use async_trait::async_trait;
use ferrumc_net::{
    connection::StreamWriter, packets::outgoing::system_message::SystemMessagePacket, NetResult,
};
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::GlobalState;
use ferrumc_text::TextComponent;

#[async_trait]
pub trait SendMessageExt {
    async fn send_message(&self, message: TextComponent, state: &GlobalState) -> NetResult<()>;
    async fn send_actionbar(&self, message: TextComponent, state: &GlobalState) -> NetResult<()>;
}

#[async_trait]
impl SendMessageExt for usize {
    async fn send_message(&self, message: TextComponent, state: &GlobalState) -> NetResult<()> {
        let mut writer = state.universe.get_mut::<StreamWriter>(*self)?;
        writer
            .send_packet(
                &SystemMessagePacket::new(message, false),
                &NetEncodeOpts::WithLength,
            )
            .await
    }

    async fn send_actionbar(&self, message: TextComponent, state: &GlobalState) -> NetResult<()> {
        let mut writer = state.universe.get_mut::<StreamWriter>(*self)?;
        writer
            .send_packet(
                &SystemMessagePacket::new(message, true),
                &NetEncodeOpts::WithLength,
            )
            .await
    }
}
