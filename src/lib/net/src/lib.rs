extern crate core;

use bevy_ecs::prelude::Resource;
use bevy_ecs::world::World;
use crossbeam_channel::{Receiver, Sender};
use ferrumc_macros::setup_packet_handling;
use std::fmt::Display;
use std::sync::Arc;

pub mod auth;
pub mod compression;
mod conn_init;
pub mod connection;
pub mod errors;
pub mod packets;
pub mod server;

setup_packet_handling!("\\src\\packets\\incoming");

#[derive(Eq, PartialEq, Debug)]
pub enum ConnState {
    Handshake,
    Login,
    Status,
    Configuration,
    Play,
}

impl Display for ConnState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnState::Handshake => write!(f, "Handshake"),
            ConnState::Login => write!(f, "Login"),
            ConnState::Status => write!(f, "Status"),
            ConnState::Configuration => write!(f, "Configuration"),
            ConnState::Play => write!(f, "Play"),
        }
    }
}
