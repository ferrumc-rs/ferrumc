use bevy_ecs::prelude::Bundle;

use crate::chunks::chunk_receiver::ChunkReceiver;
use crate::inventory::hotbar::Hotbar;
use crate::inventory::Inventory;
use crate::player::{
    abilities::PlayerAbilities,
    experience::Experience,
    gamemode::GameModeComponent,
    gameplay_mechanics::{active_effects::ActiveEffects, ender_chest::EnderChest},
    health::Health,
    hunger::Hunger,
    identity::PlayerIdentity,
    transform::{grounded::OnGround, position::Position, rotation::Rotation},
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub identity: PlayerIdentity,
    pub abilities: PlayerAbilities,
    pub gamemode: GameModeComponent,
    pub position: Position,
    pub rotation: Rotation,
    pub on_ground: OnGround,
    pub chunk_receiver: ChunkReceiver,
    pub inventory: Inventory,
    pub hotbar: Hotbar,
    pub health: Health,
    pub hunger: Hunger,
    pub experience: Experience,
    pub ender_chest: EnderChest,
    pub active_effects: ActiveEffects,
}
