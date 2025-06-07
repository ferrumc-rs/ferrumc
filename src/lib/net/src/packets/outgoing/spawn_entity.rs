use crate::errors::NetError;

use bevy_ecs::prelude::{Entity, Query, World};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::{get_registry_entry, packet, NetEncode};
use ferrumc_net_codec::net_types::angle::NetAngle;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;
use ferrumc_core::entities::entity_kind::EntityKind;
use ferrumc_core::transform::Transform;

#[derive(NetEncode, Clone)]
#[packet(packet_id = "add_entity", state = "play")]
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

const PLAYER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:player");

impl SpawnEntityPacket {
    pub fn player(
        entity_id: Entity,
        query: Query<(&PlayerIdentity, &Position, &Rotation)>,
    ) -> Result<Self, NetError> {
        let (player_identity, position, rotation) = query
            .get(entity_id)
            .unwrap_or_else(|_| panic!("Failed to get player identity, position, and rotation for entity ID: {entity_id:?}"
            ));

        Ok(Self {
            entity_id: VarInt::new(player_identity.short_uuid),
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

    pub fn entity(
        entity_id: Entity,
        pos: &Position,
        rot: &Rotation,
        entity_kind: &EntityKind
    ) -> Result<Self, NetError> {
        // generate a uuid based on the entity ID
        let entity_uuid = entity_id.index() as u128; // Placeholder, UUIDs are not used for non-player entities

        Ok(Self {
            entity_id: VarInt::new(entity_id.index() as i32),
            entity_uuid,
            r#type: VarInt::new(entity_kind.get_id() as i32),
            x: pos.x,
            y: pos.y,
            z: pos.z,
            pitch: NetAngle::from_degrees(rot.pitch as f64),
            yaw: NetAngle::from_degrees(rot.yaw as f64),
            head_yaw: NetAngle::from_degrees(rot.yaw as f64),
            data: VarInt::new(0), // Placeholder for entity data
            velocity_x: 0,
            velocity_y: 0,
            velocity_z: 0,
        })
    }
}
