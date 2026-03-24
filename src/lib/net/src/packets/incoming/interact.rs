//! Interact Entity packet.
//!
//! Sent when a player interacts with another entity (attack, use, etc).

use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Interaction types for the interact packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, NetDecode)]
#[net(type_cast = "VarInt", type_cast_handler = "value.0 as u8")]
#[repr(u8)]
pub enum InteractionType {
    /// Interact with the entity (right-click)
    Interact = 0,
    /// Attack the entity (left-click)
    Attack = 1,
    /// Interact at a specific position
    InteractAt = 2,
}

/// Sent when a player interacts with an entity.
///
/// This packet is used for both attacking (left-click) and interacting (right-click).
#[derive(Debug)]
#[packet(packet_id = "interact", state = "play")]
pub struct InteractEntity {
    /// The entity ID being interacted with
    pub entity_id: VarInt,
    /// The type of interaction
    pub interaction_type: InteractionType,
    pub target_x: Option<f32>,
    pub target_y: Option<f32>,
    pub target_z: Option<f32>,
    pub hand: Option<VarInt>,
    /// Whether the player is sneaking
    pub sneaking: bool,
}

impl ferrumc_net_codec::decode::NetDecode for InteractEntity {
    fn decode<R: std::io::Read>(
        cursor: &mut R,
        opts: &ferrumc_net_codec::decode::NetDecodeOpts,
    ) -> Result<Self, ferrumc_net_codec::decode::errors::NetDecodeError> {
        let entity_id = VarInt::decode(cursor, opts)?;
        let interaction_type = InteractionType::decode(cursor, opts)?;

        let mut target_x = None;
        let mut target_y = None;
        let mut target_z = None;
        let mut hand = None;

        match interaction_type {
            InteractionType::InteractAt => {
                target_x = Some(f32::decode(cursor, opts)?);
                target_y = Some(f32::decode(cursor, opts)?);
                target_z = Some(f32::decode(cursor, opts)?);
                hand = Some(VarInt::decode(cursor, opts)?);
            }
            InteractionType::Interact => {
                hand = Some(VarInt::decode(cursor, opts)?);
            }
            InteractionType::Attack => {}
        }

        let sneaking = bool::decode(cursor, opts)?;

        let packet = Self {
            entity_id,
            interaction_type,
            target_x,
            target_y,
            target_z,
            hand,
            sneaking,
        };

        Ok(packet)
    }

    async fn decode_async<R: tokio::io::AsyncRead + std::marker::Unpin>(
        cursor: &mut R,
        opts: &ferrumc_net_codec::decode::NetDecodeOpts,
    ) -> Result<Self, ferrumc_net_codec::decode::errors::NetDecodeError> {
        let entity_id = VarInt::decode_async(cursor, opts).await?;
        let interaction_type = InteractionType::decode_async(cursor, opts).await?;

        let mut target_x = None;
        let mut target_y = None;
        let mut target_z = None;
        let mut hand = None;

        match interaction_type {
            InteractionType::InteractAt => {
                target_x = Some(f32::decode_async(cursor, opts).await?);
                target_y = Some(f32::decode_async(cursor, opts).await?);
                target_z = Some(f32::decode_async(cursor, opts).await?);
                hand = Some(VarInt::decode_async(cursor, opts).await?);
            }
            InteractionType::Interact => {
                hand = Some(VarInt::decode_async(cursor, opts).await?);
            }
            InteractionType::Attack => {}
        }

        let sneaking = bool::decode_async(cursor, opts).await?;

        let packet = Self {
            entity_id,
            interaction_type,
            target_x,
            target_y,
            target_z,
            hand,
            sneaking,
        };

        Ok(packet)
    }
}

impl InteractEntity {
    // Check if this is an attack interaction.
    pub fn is_attack(&self) -> bool {
        self.interaction_type == InteractionType::Attack
    }
}
