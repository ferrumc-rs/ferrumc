use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::event_handler;
use ferrumc_net::{
    packets::{
        incoming::chat_message::ChatMessageEvent, outgoing::system_message::SystemMessagePacket,
    },
    utils::broadcast::{BroadcastOptions, BroadcastToAll},
    NetResult,
};
use ferrumc_state::GlobalState;
use ferrumc_text::TextComponentBuilder;

#[event_handler]
async fn chat_message(event: ChatMessageEvent, state: GlobalState) -> NetResult<ChatMessageEvent> {
    let identity = state.universe.get::<PlayerIdentity>(event.player_conn_id)?;
    let message =
        TextComponentBuilder::new(format!("<{}> {}", identity.username, event.message)).build();
    let packet = SystemMessagePacket::new(message, false);
    state
        .broadcast(&packet, BroadcastOptions::default().all())
        .await?;

    Ok(event)
}
