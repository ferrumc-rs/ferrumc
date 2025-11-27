use crate::errors::NetError;

use bevy_ecs::prelude::{Entity, Query};
use ferrumc_components::player::identity::PlayerIdentity;
use ferrumc_components::player::transform::position::Position;
use ferrumc_components::player::transform::rotation::Rotation;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_protocol::codec::net_types::angle::NetAngle;
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_ADD_ENTITY, state = "play")]
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

impl SpawnEntityPacket {
    pub fn player(
        entity_id: Entity,
        query: Query<(&PlayerIdentity, &Position, &Rotation)>,
    ) -> Result<Self, NetError> {
        let (player_identity, position, rotation) = query
            .get(entity_id)
            .unwrap_or_else(|_| panic!("Failed to get player identity, position, and rotation for entity ID: {entity_id:?}"
            ));

        // runtime lookup with O(1) map.
        let player_type_id = get_registry_id("minecraft:entity_type.entries.minecraft:player")
            .expect("Player Entity Type ID missing from registry!");

        Ok(Self {
            entity_id: VarInt::new(player_identity.short_uuid),
            entity_uuid: player_identity.uuid.as_u128(),
            r#type: VarInt::new(player_type_id as i32),
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
