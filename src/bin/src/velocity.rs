use crate::events::*;
use ferrumc_config::statics::get_global_config;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_events::{errors::EventsError, infrastructure::Event};
use ferrumc_macros::event_handler;
use ferrumc_net::packets::incoming::server_bound_plugin_message::*;
use ferrumc_net::packets::outgoing::client_bound_plugin_message::*;
use ferrumc_net::utils::ecs_helpers::EntityExt;
use ferrumc_net::{connection::StreamWriter, errors::NetError, NetResult};
use ferrumc_net_codec::decode::NetDecode;
use ferrumc_net_codec::{decode::NetDecodeOpts, encode::NetEncodeOpts, net_types::var_int::VarInt};
use ferrumc_state::GlobalState;
use ferrumc_text::*;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::io::Cursor;
use tokio::io::AsyncReadExt;

type HmacSha256 = Hmac<Sha256>;

struct VelocityMessageId(u32);

#[event_handler]
async fn handle_login_start(
    event: PlayerStartLoginEvent,
    state: GlobalState,
) -> NetResult<PlayerStartLoginEvent> {
    if get_global_config().velocity.enabled {
        let entity = event.entity;
        if entity.get::<VelocityMessageId>(&state).is_ok() {
            return Ok(event);
        }

        let id = rand::random::<u32>();
        let mut writer = entity.get_mut::<StreamWriter>(&state.clone())?;
        writer
            .send_packet(
                &LoginPluginMessagePacket::<()>::new(id, String::from("velocity:player_info"), ()),
                &NetEncodeOpts::WithLength,
            )
            .await?;
        state
            .universe
            .add_component(entity, VelocityMessageId(id))?;

        // this stops the packet handler from doing login success
        Err(NetError::EventsError(EventsError::Cancelled))
    } else {
        Ok(event)
    }
}

#[event_handler]
async fn handle_velocity_response(
    event: LoginPluginResponseEvent,
    state: GlobalState,
) -> NetResult<LoginPluginResponseEvent> {
    let message = &event.packet;
    if message.message_id.val as u32 == event.entity.get::<VelocityMessageId>(&state.clone())?.0 {
        let len = message.data.len();

        let mut signature = vec![0u8; 32];
        let mut data = Vec::with_capacity(256);
        let mut buf = Cursor::new(&message.data);

        if len > 0 && message.success {
            buf.read_exact(&mut signature).await?;

            let index = buf.position();
            buf.read_to_end(&mut data).await?;
            buf.set_position(index);

            let version = VarInt::decode(&mut buf, &NetDecodeOpts::None)?;
            let _addr = String::decode(&mut buf, &NetDecodeOpts::None)?;

            if version != 1 {
                return Err(NetError::kick(
                    TextComponentBuilder::new("[FerrumC]")
                        .color(NamedColor::Blue)
                        .space()
                        .extra(
                            ComponentBuilder::text(
                                "This velocity modern forwarding version is not supported!",
                            )
                            .color(NamedColor::Red),
                        )
                        .build(),
                ));
            }
        } else {
            return Err(NetError::kick(
                ComponentBuilder::text("[FerrumC]")
                    .color(NamedColor::Blue)
                    .space()
                    .extra(
                        ComponentBuilder::text(
                            "The velocity proxy did not send forwarding information!",
                        )
                        .color(NamedColor::Red),
                    )
                    .build(),
            ));
        }

        let mut key = HmacSha256::new_from_slice(get_global_config().velocity.secret.as_bytes())
            .expect("Failed to create HmacSha256 for velocity secret");
        key.update(&data);

        if key.verify_slice(&signature[..]).is_ok() {
            let e = PlayerStartLoginEvent {
                entity: event.entity,
                profile: PlayerIdentity::decode(&mut buf, &NetDecodeOpts::None)?,
            };

            match PlayerStartLoginEvent::trigger(e, state.clone()).await {
                Ok(e) => {
                    state
                        .universe
                        .remove_component::<VelocityMessageId>(event.entity)?;

                    crate::send_login_success(state.clone(), event.entity, e.profile).await?;

                    Ok(event)
                }
                e => e.map(|_| event),
            }
        } else {
            Err(NetError::kick("Invalid proxy response!".to_string()))
        }
    } else {
        Ok(event)
    }
}
