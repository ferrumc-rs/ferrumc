use ferrumc_macros::{packet, NetEncode};
use std::io::Write;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_ecs::entities::Entity;
use ferrumc_net_codec::net_types::angle::NetAngle;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;
use crate::NetResult;
use crate::utils::ecs_helpers::EntityExt;

#[derive(NetEncode)]
#[packet(packet_id = 0x01)]
pub struct SpawnEntityPacket {
    entity_id: VarInt,
    entity_uuid: u128,
    r#type: VarInt,
    x: f64,
    y: f64,
    z: f64,
    pitch: NetAngle,
    yaw: NetAngle,
    head_yaw: NetAngle,
    data: VarInt,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
}

const PLAYER_ID: u8 = 128;

impl SpawnEntityPacket {
    pub fn player(entity_id: Entity, state: &GlobalState) -> NetResult<Self> {
        let player_identity = entity_id.get::<PlayerIdentity>(state)?;
        let position = entity_id.get::<Position>(state)?;
        let rotation = entity_id.get::<Rotation>(state)?;
        
        
        Ok(Self {
            entity_id: VarInt::new(entity_id as i32),
            entity_uuid: player_identity.uuid,
            r#type: VarInt::new(PLAYER_ID as i32),
            x: position.x,
            y: position.y,
            z: position.z,
            pitch: NetAngle::from_degrees(rotation.pitch as f64),
            yaw: NetAngle::from_degrees(rotation.yaw as f64),
            head_yaw: NetAngle::from_degrees(rotation.yaw as f64),
            data: VarInt::new(0),
            velocity_x: 0,
            velocity_y: 0,
            velocity_z: 0,
        })
    }
}
