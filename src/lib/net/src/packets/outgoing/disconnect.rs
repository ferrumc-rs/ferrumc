use crate::connection::ConnectionState;
use crate::{errors::NetError, NetResult};
use ferrumc_macros::{packet, NetEncode};
use ferrumc_text::*;
use std::io::Write;

pub const DISCONNECT_STRING: &str = "§cDisconnected";

#[derive(NetEncode)]
pub enum DisconnectPacket {
    Login(LoginDisconnectPacket),
    Play(PlayDisconnectPacket),
}

#[derive(NetEncode)]
#[packet(packet_id = "login_disconnect", state = "login")]
pub struct LoginDisconnectPacket {
    pub reason: JsonTextComponent,
}

#[derive(NetEncode)]
#[packet(packet_id = "disconnect", state = "play")]
pub struct PlayDisconnectPacket {
    pub reason: TextComponent,
}

impl DisconnectPacket {
    pub fn from<C: Into<TextComponent>>(state: &ConnectionState, reason: C) -> NetResult<Self> {
        match state {
            ConnectionState::Login => Ok(DisconnectPacket::Login(LoginDisconnectPacket::new(
                reason.into(),
            ))),
            ConnectionState::Play => Ok(DisconnectPacket::Play(PlayDisconnectPacket::new(reason))),
            _ => Err(NetError::InvalidState(state.clone() as u8)),
        }
    }
}

impl LoginDisconnectPacket {
    pub fn new<C: Into<JsonTextComponent>>(reason: C) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}

impl PlayDisconnectPacket {
    pub fn new<C: Into<TextComponent>>(reason: C) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}
