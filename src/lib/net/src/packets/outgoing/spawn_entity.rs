use crate::errors::NetError;

use bevy_ecs::prelude::{Entity, Query};
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::{get_registry_entry, packet, NetEncode};
use ferrumc_net_codec::net_types::angle::NetAngle;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
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
    /// Creates a spawn entity packet from direct component values.
    ///
    /// This is useful when you have the component values directly
    /// rather than needing to query them.
    pub fn new(
        entity_id: i32,
        entity_uuid: u128,
        entity_type_id: i32,
        position: &Position,
        rotation: &Rotation,
    ) -> Self {
        Self {
            entity_id: VarInt::new(entity_id),
            entity_uuid,
            r#type: VarInt::new(entity_type_id),
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
        }
    }

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
            entity_uuid: player_identity.uuid.as_u128(),
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

    /// Creates a spawn entity packet for any entity (mob, projectile, etc.).
    ///
    /// # Arguments
    ///
    /// * `entity` - Bevy entity to spawn
    /// * `entity_type_id` - Protocol ID for the entity type (from vanilla data)
    /// * `query` - Query to get entity components
    pub fn entity(
        entity: Entity,
        entity_type_id: u16,
        query: Query<(&EntityIdentity, &Position, &Rotation)>,
    ) -> Result<Self, NetError> {
        let (identity, position, rotation) = query.get(entity).unwrap_or_else(|_| {
            panic!("Failed to get entity identity, position, and rotation for entity: {entity:?}")
        });

        Ok(Self {
            entity_id: VarInt::new(identity.entity_id),
            entity_uuid: identity.uuid.as_u128(),
            r#type: VarInt::new(entity_type_id as i32),
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
