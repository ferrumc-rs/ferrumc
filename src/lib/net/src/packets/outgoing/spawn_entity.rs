use crate::{NetResult, utils::ecs_helpers::EntityExt};
use ferrumc_state::GlobalState;
use ferrumc_core::{
    identity::player_identity::PlayerIdentity,
    transform::{position::Position, rotation::Rotation}
};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x01)]
pub struct SpawnEntityPacket {
    pub entity_id: VarInt,
    pub entity_uuid: u128,
    pub entity_type: VarInt,
    pub position: Position,
    pub pitch: u8,
    pub yaw: u8,
    pub head_yaw: u8,
    pub data: VarInt,
    pub velocity_x: u16,
    pub velocity_y: u16,
    pub velocity_z: u16,

}

impl SpawnEntityPacket {
    pub fn new(id: usize, state: GlobalState) -> NetResult<Self> {
        // only spawn players for now
        let identity = id.get::<PlayerIdentity>(&state)?;
        let position = id.get::<Position>(&state)?;
        let (yaw, pitch) = id.get::<Rotation>(&state)?.to_angle();

        Ok(Self {
            entity_id: VarInt::from(id),
            entity_uuid: identity.uuid,
            entity_type: VarInt::from(128), // hardcoded for now
            position: *position,
            pitch,
            yaw,
            head_yaw: yaw,
            data: VarInt::new(0),
            velocity_x: 0u16,
            velocity_y: 0u16,
            velocity_z: 0u16,
        })
    }
}
